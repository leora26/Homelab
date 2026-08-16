use derive_new::new;
use fast_image_resize::{FilterType, Image, PixelType, ResizeAlg, Resizer};
use homelab_core::nas_domain::file::{File, FileType};
use std::env;
use std::num::NonZeroU32;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::sync::OnceLock;
use tokio::process::Command;
use tokio::task;
use crate::service::contract::preview_service::PreviewService;

#[derive(new)]
pub struct PreviewServiceImpl;

impl PreviewService for PreviewServiceImpl {
    fn spawn_generation(file: File, storage_path: PathBuf) {
        tokio::spawn(async move {
            let file_path = file.build_file_path(&storage_path);
            let base_path = file_path.clone();

            let ffmpeg_binary = Self::ffmpeg_binary();

            let thread_result = match file.file_type {
                FileType::Image => {
                    task::spawn_blocking(move || {
                        Self::generate_image_preview(&file_path, &base_path)
                    })
                        .await
                }
                FileType::Video => {
                    let f_path = file_path.to_string_lossy().to_string();
                    let preview_path = base_path.with_extension("preview.jpg");
                    let p_path = preview_path.to_string_lossy().to_string();

                    let video_result =
                        match Self::try_extract_cover(ffmpeg_binary, &f_path, &p_path).await {
                            Ok(_) => Ok(()),
                            Err(_) => {
                                match Self::extract_frame_gpu(ffmpeg_binary, &f_path, &p_path).await
                                {
                                    Ok(_) => Ok(()),
                                    Err(gpu_err) => {
                                        eprintln!(
                                            "GPU Preciew failed for {}: {}. Retrying with CPU",
                                            file.id, gpu_err
                                        );
                                        Self::extract_frame_cpu(ffmpeg_binary, &f_path, &p_path)
                                            .await
                                    }
                                }
                            }
                        };

                    // A failed attempt can still leave a zero-byte file behind, and the read
                    // side only checks whether the preview *exists* — so a leftover stub is
                    // served as a broken image forever. Clear it when nothing worked.
                    if video_result.is_err() {
                        let _ = tokio::fs::remove_file(&preview_path).await;
                    }

                    Ok(video_result)
                }
                FileType::Audio => {
                    let f_path = file_path.to_string_lossy().to_string();
                    let preview_path = base_path.with_extension("preview.jpg");
                    let p_path = preview_path.to_string_lossy().to_string();

                    // Audio without embedded cover art is normal, not a failure — but the
                    // stub ffmpeg may have opened still has to go.
                    if Self::try_extract_cover(ffmpeg_binary, &f_path, &p_path).await.is_err() {
                        let _ = tokio::fs::remove_file(&preview_path).await;
                    }

                    Ok(Ok(()))
                }
                FileType::Pdf if file.name.ends_with(".pdf") => {
                    let f_path = file_path.to_string_lossy().to_string();

                    match Self::generate_pdf_preview(&f_path, &base_path).await {
                        Ok(_) => Ok(Ok(())),
                        Err(e) => Ok(Err(e)),
                    }
                }

                // For text/unknown, we just return "Success" (Ok) doing nothing
                _ => Ok(Ok(())),
            };

            match thread_result {
                Err(join_err) => {
                    // TODO: send to admin console
                    eprintln!(
                        "CRITICAL: Preview thread crashed for file {}: {}",
                        file.id, join_err
                    );
                }
                Ok(logic_result) => {
                    match logic_result {
                        Ok(_) => {
                            // TODO: Maybe set some kind of flag in the database for a given file
                            println!("Preview generated for {}", file.id);
                        }
                        Err(app_err) => {
                            // TODO: Send error to admin console
                            eprintln!(
                                "Preview generation failed for file {}: {}",
                                file.id, app_err
                            );
                        }
                    }
                }
            }
        });
    }
}

impl PreviewServiceImpl {
    /// Resolves the transcoder once per process.
    ///
    /// Jellyfin's build stays the default because it ships the NVENC/CUDA support the GPU
    /// path needs, but hard-coding it meant every video preview died with ENOENT on a host
    /// that only has the distro package. `FFMPEG_PATH` wins if set; otherwise we fall back
    /// to whatever `ffmpeg` is on PATH.
    fn ffmpeg_binary() -> &'static str {
        static BINARY: OnceLock<String> = OnceLock::new();

        BINARY
            .get_or_init(|| {
                if let Ok(configured) = env::var("FFMPEG_PATH") {
                    return configured;
                }

                const JELLYFIN: &str = "/usr/lib/jellyfin-ffmpeg/ffmpeg";

                if Path::new(JELLYFIN).exists() {
                    return JELLYFIN.to_string();
                }

                "ffmpeg".to_string()
            })
            .as_str()
    }

    async fn generate_pdf_preview(input: &str, base_path: &PathBuf) -> Result<(), String> {
        let parent = base_path.parent().ok_or("Invalid parent dir")?;
        let temp_prefix = base_path
            .file_stem()
            .ok_or("Invalid temp prefix")?
            .to_string_lossy();
        let temp_prefix_path = parent.join(format!("{}_temp", temp_prefix));

        let status = Command::new("pdftoppm")
            .arg("-jpeg")
            .arg("-f")
            .arg("1")
            .arg("-l")
            .arg("1")
            .arg("-scale-to")
            .arg("320")
            .arg(input)
            .arg(&temp_prefix_path)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .await
            .map_err(|e| e.to_string())?;

        if !status.success() {
            return Err(format!("pdftoppm failed with code: {}", status));
        }

        let generated_filename = format!("{}-1.jpg", temp_prefix_path.to_string_lossy());

        let final_output = base_path.with_extension("preview.jpg");

        tokio::fs::rename(generated_filename, final_output)
            .await
            .map_err(|e| format!("Failed to rename PDF preview: {}", e))?;

        Ok(())
    }

    fn generate_image_preview(input_path: &PathBuf, base_path: &PathBuf) -> Result<(), String> {
        let img = image::open(input_path)
            .map_err(|e| format!("Corrupt or unsupported image format: {}", e))?;

        let has_alpha = img.color().has_alpha();
        let width = img.width() as f32;
        let height = img.height() as f32;
        let max_dim = 100.0;
        let scale = (max_dim / width).min(max_dim / height).min(1.0);

        let new_width = (width * scale).round() as u32;
        let new_height = (height * scale).round() as u32;

        let src_w = NonZeroU32::new(img.width()).ok_or("Image width is 0")?;
        let src_h = NonZeroU32::new(img.height()).ok_or("Image height is 0")?;
        let dst_width = NonZeroU32::new(new_width).ok_or("Calculated width is 0")?;
        let dst_height = NonZeroU32::new(new_height).ok_or("Calculated height is 0")?;

        let mut resizer = Resizer::new(ResizeAlg::Convolution(FilterType::Lanczos3));

        if has_alpha {
            let rgba_img = img.into_rgba8();
            let src_img = Image::from_vec_u8(src_w, src_h, rgba_img.into_raw(), PixelType::U8x4)
                .map_err(|e| format!("Image conversion failed: {}", e))?;

            let dst_len = (new_width * new_height * 4) as usize;
            let mut dst_image = Image::from_vec_u8(
                dst_width, dst_height, vec![0u8; dst_len], PixelType::U8x4
            ).map_err(|_| "Failed to map destination buffer")?;

            resizer.resize(&src_img.view(), &mut dst_image.view_mut()).map_err(|_| "Failed to resize")?;

            let final_output = base_path.with_extension("preview.png");
            image::save_buffer_with_format(
                &final_output, dst_image.buffer(), new_width, new_height,
                image::ColorType::Rgba8, image::ImageFormat::Png,
            ).map_err(|e| format!("Failed to save image: {}", e))?;
        } else {
            let rgba_img = img.into_rgb8();
            let src_img = Image::from_vec_u8(src_w, src_h, rgba_img.into_raw(), PixelType::U8x3)
                .map_err(|e| format!("Image conversion failed: {}", e))?;

            let dst_len = (new_width * new_height * 3) as usize;
            let mut dst_image = Image::from_vec_u8(
                dst_width, dst_height, vec![0u8; dst_len], PixelType::U8x3
            ).map_err(|_| "Failed to map destination buffer")?;

            resizer.resize(&src_img.view(), &mut dst_image.view_mut()).map_err(|_| "Failed to resize")?;

            let final_output = base_path.with_extension("preview.jpg");
            image::save_buffer_with_format(
                &final_output, dst_image.buffer(), new_width, new_height,
                image::ColorType::Rgb8, image::ImageFormat::Jpeg,
            ).map_err(|e| format!("Failed to save image: {}", e))?;
        }

        Ok(())
    }

    async fn try_extract_cover(ffmpeg: &str, input: &str, output: &str) -> Result<(), String> {
        let status = Command::new(ffmpeg)
            .arg("-y")
            .arg("-i")
            .arg(input)
            .arg("-map")
            .arg("0:v")
            .arg("-map")
            .arg("-0:V")
            .arg("-c")
            .arg("copy")
            .arg(output)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .await
            .map_err(|e| e.to_string())?;

        if status.success() {
            Ok(())
        } else {
            Err("No cover found".into())
        }
    }

    /// Grabs a frame, optionally through the GPU decoder.
    ///
    /// Uses `output()` rather than `status()` deliberately. `status()` drops all three
    /// stdio handles the moment the child is spawned so it can't deadlock on a pipe nobody
    /// reads — which closes the read end of a piped stderr. ffmpeg writes its banner there
    /// before doing anything else, takes SIGPIPE, and dies with signal 13 every single
    /// time. `output()` keeps the pipe and drains it, so the process survives *and* we get
    /// ffmpeg's actual complaint instead of an exit code.
    async fn extract_frame(
        ffmpeg: &str,
        input: &str,
        output: &str,
        hwaccel: bool,
    ) -> Result<(), String> {
        let mut command = Command::new(ffmpeg);

        command.arg("-y");

        if hwaccel {
            command.arg("-hwaccel").arg("cuda");
        }

        let result = command
            .arg("-ss")
            .arg("00:00:05")
            .arg("-i")
            .arg(input)
            .arg("-frames:v")
            .arg("1")
            // `-2` rather than `-1`: the height is derived from the aspect ratio, and
            // mjpeg can't encode an odd one.
            .arg("-vf")
            .arg("scale=320:-2")
            .arg(output)
            .stdin(Stdio::null())
            .output()
            .await
            .map_err(|e| e.to_string())?;

        if result.status.success() {
            return Ok(());
        }

        // Everything before the failure is banner and stream metadata.
        let stderr = String::from_utf8_lossy(&result.stderr);
        let reason = stderr
            .lines()
            .rev()
            .find(|line| !line.trim().is_empty())
            .unwrap_or("no stderr output");

        Err(format!("{} ({})", reason.trim(), result.status))
    }

    async fn extract_frame_gpu(ffmpeg: &str, input: &str, output: &str) -> Result<(), String> {
        Self::extract_frame(ffmpeg, input, output, true).await
    }

    async fn extract_frame_cpu(ffmpeg: &str, input: &str, output: &str) -> Result<(), String> {
        Self::extract_frame(ffmpeg, input, output, false).await
    }
}
