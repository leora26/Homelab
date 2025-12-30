use std::num::NonZeroU32;
use std::path::PathBuf;
use derive_new::new;
use fast_image_resize::{FilterType, Image, PixelType, ResizeAlg, Resizer};
use tokio::task;
use crate::domain::file::{File, FileType};

pub trait PreviewService {
    fn spawn_generation (file: File, storage_path: PathBuf);
    fn generate_image_preview (input_path: &PathBuf, output_path: &PathBuf) -> Result<(), String>;
}

#[derive(new)]
pub struct PreviewServiceImpl;

impl PreviewService for PreviewServiceImpl {
    fn spawn_generation(file: File, storage_path: PathBuf) {
        tokio::spawn(async move {
            let file_path = file.build_file_path(&storage_path);
            let preview_path = file_path.with_extension("preview");

            let thread_result = match file.file_type {
                FileType::Image => {
                    task::spawn_blocking(move || {
                        Self::generate_image_preview(&file_path, &preview_path)
                    })
                        .await
                }
                FileType::Video => {
                    // TODO: implement GPU accelerated video privew generation
                    Ok(Ok(()))
                }
                // For text/unknown, we just return "Success" (Ok) doing nothing
                _ => Ok(Ok(())),
            };

            match thread_result {
                Err(join_err) => {
                    // TODO: send to admin console
                    eprintln!("CRITICAL: Preview thread crashed for file {}: {}", file.id, join_err);
                },
                Ok(logic_result) => {
                    match logic_result {
                        Ok(_) => {
                            // TODO: Maybe set some kind of flag in the database for a given file
                            println!("Preview generated for {}", file.id);
                        },
                        Err(app_err) => {
                            // TODO: Send error to admin console
                            eprintln!("Preview generation failed for file {}: {}", file.id, app_err);
                        }
                    }
                }
            }

        });
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

        let src_image = Image::from_vec_u8(
            src_w,
            src_h,
            img.to_rgba8().into_raw(),
            PixelType::U8x4,
        ).map_err(|_| "Failed to create source buffer")?;

        let dst_width = NonZeroU32::new(new_width).ok_or("Calculated width is 0")?;
        let dst_height = NonZeroU32::new(new_height).ok_or("Calculated height is 0")?;

        let mut dst_image = Image::new(
            dst_width,
            dst_height,
            PixelType::U8x4
        );

        let mut resizer = Resizer::new(ResizeAlg::Convolution(FilterType::Lanczos3));

        resizer.resize(&src_image.view(), &mut dst_image.view_mut())
            .map_err(|_| "Failed to resize image")?;

        image::save_buffer_with_format(
            output_path,
            dst_image.buffer(),
            new_width,
            new_height,
            image::ColorType::Rgba8,
            image::ImageFormat::Jpeg
        ).map_err(|e| format!("Disk Write Error: {}", e))?;

        Ok(())
    }
}