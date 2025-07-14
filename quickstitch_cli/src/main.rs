pub mod _cli;

use clap::{value_parser, Args, Parser, ValueEnum};
use log::{error, info};
use quickstitch as qs;
use quickstitch::Stitcher;
use std::path::{Path, PathBuf};
use std::process::exit;
use std::time::Instant;

#[derive(Debug, Clone, ValueEnum)]
enum ImageFormat {
    Png,
    Webp,
    Jpg,
    Jpeg,
}
#[derive(Debug, Clone, ValueEnum)]
enum Sort {
    Natural,
    Logical,
}
#[derive(Debug, Clone, Args)]
#[group(required = true, multiple = false)]
struct Input {
    /// The images to stitch.
    images: Option<Vec<PathBuf>>,
    /// A directory of images to stitch.
    #[clap(long, short, alias = "dir")]
    dir: Option<PathBuf>,
}

/// Quickly stitch raws.
///
/// A list of images can provided as input, or the `--dir` flag can be used
/// instead to specify a directory of images to stitch.
#[derive(Debug, Clone, Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[clap(flatten)]
    input: Input,

    /// The output directory to place the stitched images in.
    #[clap(long, short, default_value = "./stitched")]
    output: PathBuf,

    /// The sorting method used to sort the images before stitching (only works with `--dir`).
    ///
    /// Given the images ["9.jpeg", "10.jpeg", "8.jpeg", "11.jpeg"]:
    ///   - Logical: ["10.jpeg", "11.jpeg", 8.jpeg", "9.jpeg"]
    ///   - Natural: ["8.jpeg", "9.jpeg", "10.jpeg", "11.jpeg"]
    #[clap(long, default_value_t = Sort::Natural, verbatim_doc_comment)]
    #[arg(value_enum)]
    sort: Sort,

    /// The max height for stitched images.
    ///
    /// Stitched images will aim to be as tall as this parameter,
    /// but they may be shorter if visual elements are in the way.
    #[clap(long, visible_alias = "max", default_value_t = 5000)]
    max_height: usize,

    /// The minimum height for stitched images.
    #[clap(long, visible_alias = "min", default_value_t = 0)]
    min_height: usize,

    /// The interval at which lines of pixels are scanned. For example,
    /// a value of 5 means every 5th horizontal line of pixels will be
    /// analyzed.
    #[clap(long, default_value_t = 5)]
    scan_interval: usize,

    /// The threshold value between 0 and 255 for determining when a line of
    /// pixels should not be used as a splitpoint. 0 would allow the line
    /// to be used as a splitpoint regardless of the line's pixels' values,
    /// while 255 would only allow the line to be used as a splitpoint if
    /// all the pixels in the line have the same value.
    #[clap(long, short, default_value_t = 220)]
    #[arg(value_parser(value_parser!(u8).range(0..=255)))]
    sensitivity: u8,

    /// The file extension/type used for exporting the stitched images.
    #[clap(long, short, default_value_t = ImageFormat::Jpg)]
    #[arg(value_enum)]
    format: ImageFormat,

    /// The image quality to aim for when compressing.
    ///
    /// A value from 1 to 100 may be provided to specify the amount
    /// of compression to be used.
    /// A lower value represents more compression. This flag only takes
    /// effect when `--format` is passed a value of `jpg` (the default value)
    /// or `jpeg`. Otherwise, it will be ignored.
    #[clap(long, short, default_value_t = 100)]
    #[arg(value_parser(value_parser!(u8).range(1..=100)))]
    quality: u8,

    /// The fixed width of the final stitched images, in pixels.
    #[clap(long, short)]
    width: Option<u32>,

    /// Enable debug mode.
    ///
    /// Using the stitcher in debug mode will result in red and light blue lines
    /// in the resulting stitched images, with red lines denoting selected cut
    /// points and light blue lines denoting potential cut points that were skipped.
    #[clap(long, default_value_t = false)]
    debug: bool,
}

fn main() {
    env_logger::init();
    let cli = Cli::parse();

    let stitcher = Stitcher::new();
    let now = Instant::now();
    let loaded = match (cli.input.images, cli.input.dir) {
        (Some(images), None) => {
            let paths: Vec<&Path> = images.iter().map(PathBuf::as_path).collect();
            stitcher.load(&paths, cli.width, true)
        }
        (None, Some(dir)) => stitcher.load_dir(
            &dir,
            cli.width,
            true,
            match cli.sort {
                Sort::Natural => qs::Sort::Natural,
                Sort::Logical => qs::Sort::Logical,
            },
        ),
        _ => unimplemented!("arg group rules ensure only one of the two is provided"),
    };
    let loaded = match loaded {
        Ok(stitcher) => {
            info!("Images loaded successfully in {:?}", now.elapsed());
            stitcher
        }
        Err(e) => {
            error!("Unable to load images: {e}");
            exit(1);
        }
    };
    let now = Instant::now();
    let stitched = loaded.stitch(
        cli.max_height,
        cli.min_height,
        cli.scan_interval,
        cli.sensitivity,
    );
    info!("Splitpoints found in {:?}", now.elapsed());
    let now = Instant::now();

    match std::fs::create_dir_all(&cli.output) {
        Ok(_) => {}
        Err(e) => {
            error!("Unable to create output file directory: {e}");
            exit(1);
        }
    }
    let errs = stitched.export(
        &cli.output,
        match cli.format {
            ImageFormat::Png => qs::ImageOutputFormat::Png,
            ImageFormat::Webp => qs::ImageOutputFormat::Webp,
            ImageFormat::Jpg => qs::ImageOutputFormat::Jpg(cli.quality),
            ImageFormat::Jpeg => qs::ImageOutputFormat::Jpeg(cli.quality),
        },
        cli.debug,
    );
    match errs {
        Ok(_) => info!("Images exported in {:?}", now.elapsed()),
        Err(e) => {
            for err in e {
                error!("Unable to export image: {err}");
            }
            exit(1);
        }
    }
}
