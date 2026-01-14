use derive_new::new;
use fast_image_resize::{FilterType, Image, PixelType, ResizeAlg, Resizer};
use homelab_core::file::{File, FileType};
use std::num::NonZeroU32;
use std::path::PathBuf;
use std::process::Stdio;
use tokio::process::Command;
use tokio::task;

pub trait PreviewService {
    fn spawn_generation(file: File, storage_path: PathBuf);
}

#[derive(new)]
pub struct PreviewServiceImpl;

impl PreviewService for PreviewServiceImpl {
    fn spawn_generation(file: File, storage_path: PathBuf) {
        tokio::spawn(async move {
            let file_path = file.build_file_path(&storage_path);
            let preview_path = file_path.with_extension("preview");

            let ffmpeg_binary = "/usr/lib/jellyfin-ffmpeg/ffmpeg";

            let thread_result = match file.file_type {
                FileType::Image => {
                    task::spawn_blocking(move || {
                        Self::generate_image_preview(&file_path, &preview_path)
                    })
                    .await
                }
                FileType::Video => {
                    let f_path = file_path.to_string_lossy().to_string();
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

                    Ok(video_result)
                }
                FileType::Audio => {
                    let f_path = file_path.to_string_lossy().to_string();
                    let p_path = preview_path.to_string_lossy().to_string();

                    let _ = Self::try_extract_cover(ffmpeg_binary, &f_path, &p_path).await;
                    Ok(Ok(()))
                }
                FileType::Pdf if file.name.ends_with(".pdf") => {
                    let f_path = file_path.to_string_lossy().to_string();

                    match Self::generate_pdf_preview(&f_path, &preview_path).await {
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
    async fn generate_pdf_preview(input: &str, output_path: &PathBuf) -> Result<(), String> {
        let parent = output_path.parent().ok_or("Invalid parent dir")?;
        let temp_prefix = output_path
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

        tokio::fs::rename(generated_filename, output_path)
            .await
            .map_err(|e| format!("Failed to rename PDF preview: {}", e))?;

        Ok(())
    }

    fn generate_image_preview(input_path: &PathBuf, output_path: &PathBuf) -> Result<(), String> {
        let img = image::open(input_path)
            .map_err(|e| format!("Corrupt or unsupported image format: {}", e))?;

        let width = img.width() as f32;
        let height = img.height() as f32;
        let max_dim = 100.0;
        let scale = (max_dim / width).min(max_dim / height).min(1.0);

        let new_width = (width * scale).round() as u32;
        let new_height = (height * scale).round() as u32;

        let src_w = NonZeroU32::new(img.width()).ok_or("Image width is 0")?;
        let src_h = NonZeroU32::new(img.height()).ok_or("Image height is 0")?;

        let src_image =
            Image::from_vec_u8(src_w, src_h, img.to_rgba8().into_raw(), PixelType::U8x4)
                .map_err(|_| "Failed to create source buffer")?;

        let dst_width = NonZeroU32::new(new_width).ok_or("Calculated width is 0")?;
        let dst_height = NonZeroU32::new(new_height).ok_or("Calculated height is 0")?;

        let mut dst_image = Image::new(dst_width, dst_height, PixelType::U8x4);

        let mut resizer = Resizer::new(ResizeAlg::Convolution(FilterType::Lanczos3));

        resizer
            .resize(&src_image.view(), &mut dst_image.view_mut())
            .map_err(|_| "Failed to resize image")?;

        image::save_buffer_with_format(
            output_path,
            dst_image.buffer(),
            new_width,
            new_height,
            image::ColorType::Rgba8,
            image::ImageFormat::Jpeg,
        )
        .map_err(|e| format!("Disk Write Error: {}", e))?;

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

    async fn extract_frame_gpu(ffmpeg: &str, input: &str, output: &str) -> Result<(), String> {
        let status = Command::new(ffmpeg)
            .arg("-y")
            .arg("-hwaccel")
            .arg("cuda")
            .arg("-ss")
            .arg("00:00:05")
            .arg("-i")
            .arg(input)
            .arg("-frame:v")
            .arg("1")
            .arg("-vf")
            .arg("scale=320:-1")
            .arg(output)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .status()
            .await
            .map_err(|e| e.to_string())?;

        if status.success() {
            Ok(())
        } else {
            Err(format!("GPU Extraction exit code: {}", status))
        }
    }

    async fn extract_frame_cpu(ffmpeg: &str, input: &str, output: &str) -> Result<(), String> {
        let status = Command::new(ffmpeg)
            .arg("-y")
            .arg("-ss")
            .arg("00:00:05")
            .arg("-i")
            .arg(input)
            .arg("-frame:v")
            .arg("1")
            .arg("-vf")
            .arg("scale=320:-1")
            .arg(output)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .status()
            .await
            .map_err(|e| e.to_string())?;

        if status.success() {
            Ok(())
        } else {
            Err(format!("CPU Extraction exit code: {}", status))
        }
    }
}
