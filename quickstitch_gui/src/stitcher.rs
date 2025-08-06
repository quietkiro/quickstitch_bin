use std::path::PathBuf;

use quickstitch::{
    ImageLoaderError, ImageOutputFormat, ImageSplitterError, Sort, Splitpoint, Stitcher,
};
use thiserror::Error;

use crate::gui::{
    io_section::{ImageFormat, InputType, SortMethod},
    limit_section::WidthType,
};

#[derive(Error, Debug)]
pub enum StitcherError {
    #[error("Input directory must be set")]
    NoInputDirectory,
    #[error("At least one input image must be provided")]
    NoInputImages,
    #[error("Output directory must be set")]
    NoOutputDirectory,
    #[error("Compression quality cannot be empty")]
    EmptyQuality,
    #[error("Output image width cannot be empty")]
    EmptyOutputImageWidth,
    #[error("Max output height cannot be empty")]
    EmptyMaxOutputHeight,
    #[error("Min output height cannot be empty")]
    EmptyMinOutputHeight,
    #[error("Scan interval cannot be empty")]
    EmptyScanInterval,
    #[error("Sensitivity cannot be empty")]
    EmptySensitivity,
    #[error("Image loader error: {0}")]
    ImageLoaderError(ImageLoaderError),
    #[error("Image splitter errors: {0}")]
    ImageSplitterError(ImageSplitterError),
}

pub fn stitcher(
    input_type: InputType,
    input_directory: Option<PathBuf>,
    image_sorting: SortMethod,
    image_files: Vec<PathBuf>,
    ignore_unloadable: bool,
    output_directory: Option<PathBuf>,
    output_format: ImageFormat,
    quality: Option<u8>,
    output_width_type: WidthType,
    image_width: Option<u32>,
    max_image_height: Option<usize>,
    min_image_height: Option<usize>,
    scan_interval: Option<usize>,
    sensitivity: Option<u8>,
    debug: bool,
) -> Result<Vec<Splitpoint>, StitcherError> {
    // Required fields validation

    let (input_dir, image_files) = match input_type {
        InputType::Directory => {
            let input_dir = match input_directory {
                Some(dir) => dir,
                None => return Err(StitcherError::NoInputDirectory),
            };
            (input_dir, vec![])
        }
        InputType::Images => {
            if image_files.is_empty() {
                return Err(StitcherError::NoInputImages);
            }
            (PathBuf::new(), image_files)
        }
    };

    let image_sorting = match image_sorting {
        SortMethod::Natural => Sort::Natural,
        SortMethod::Logical => Sort::Logical,
    };
    let output_directory = match output_directory {
        Some(dir) => dir,
        None => return Err(StitcherError::NoOutputDirectory),
    };
    let width = match output_width_type {
        WidthType::Auto => None,
        WidthType::Fixed => match image_width {
            Some(width) => Some(width),
            None => return Err(StitcherError::EmptyOutputImageWidth),
        },
    };
    let max_image_height = match max_image_height {
        Some(max) => max,
        None => return Err(StitcherError::EmptyMaxOutputHeight),
    };
    let min_image_height = match min_image_height {
        Some(min) => min,
        None => return Err(StitcherError::EmptyMinOutputHeight),
    };
    let quality = match output_format {
        ImageFormat::JPEG => match quality {
            Some(quality) => quality,
            None => return Err(StitcherError::EmptyQuality),
        },
        ImageFormat::WebP => 0,
        ImageFormat::PNG => 0,
    };
    let scan_interval = match scan_interval {
        Some(interval) => interval,
        None => return Err(StitcherError::EmptyScanInterval),
    };
    let sensitivity = match sensitivity {
        Some(sensitivity) => sensitivity,
        None => return Err(StitcherError::EmptySensitivity),
    };
    let output_filetype = match output_format {
        ImageFormat::JPEG => ImageOutputFormat::Jpg(quality),
        ImageFormat::WebP => ImageOutputFormat::Webp,
        ImageFormat::PNG => ImageOutputFormat::Png,
    };

    // Using Quickstitch

    let stitcher = Stitcher::new();
    let loaded = match input_type {
        InputType::Directory => {
            stitcher.load_dir(input_dir, width, ignore_unloadable, image_sorting)
        }
        InputType::Images => stitcher.load(&image_files, width, ignore_unloadable),
    };
    let loaded = match loaded {
        Ok(loaded) => loaded,
        Err(e) => return Err(StitcherError::ImageLoaderError(e)),
    };
    let stitched = loaded.stitch(
        max_image_height,
        min_image_height,
        scan_interval,
        sensitivity,
    );
    match stitched.export(output_directory, output_filetype, debug) {
        Ok(_) => {}
        Err(e) => {
            return Err(StitcherError::ImageSplitterError(
                e.into_iter().next().unwrap(),
            ));
        }
    }

    Ok(stitched.splitpoits().to_vec())
}
