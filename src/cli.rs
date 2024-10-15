use std::path::PathBuf;

use argh::FromArgs;

#[derive(FromArgs, Debug)]
/// webp-convert-api provides a WebP image conversion service.
pub struct CliArgs {
    #[argh(option, short = 'P')]
    /// write pid to file
    pub pidfile: Option<PathBuf>,
}
