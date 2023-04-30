use anyhow::Result;
use clap::Parser;
use image::io::Reader as ImageReader;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Debug, Error)]
enum CliError {
    #[error("ディレクトリではありません")]
    NotADirectory,
    #[error("ファイルの変換に失敗しました: [{0}]")]
    ConvertError(#[from] image::ImageError),
}

#[derive(Parser, Debug)]
struct Cli {
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    let path = args.path;

    match convert_manager(&path) {
        Ok(_) => println!("Success"),
        Err(e) => println!("Error: {}", e),
    }
}

fn convert_jpg_to_png(path: &PathBuf) -> Result<(), CliError> {
    let img = ImageReader::open(path)
        .map_err(|_| CliError::NotADirectory)?
        .decode()
        .map_err(CliError::ConvertError)?;
    let mut new_path = path.to_path_buf();
    println!("{} -> {}", path.display(), new_path.display());
    new_path.set_extension("png");
    img.save(new_path).map_err(CliError::ConvertError)?;

    Ok(())
}

fn convert_manager(path: &Path) -> Result<(), CliError> {
    if !path.is_dir() {
        return Err(CliError::NotADirectory);
    }

    let dir_entries = path.read_dir().map_err(|_| CliError::NotADirectory)?;

    for entry in dir_entries {
        let path = entry.map_err(|_| CliError::NotADirectory)?.path();
        if path.is_dir() {
            convert_manager(&path)?;
        } else if let Some(extension) = path.extension() {
            if extension == "jpg" {
                if let Err(e) = convert_jpg_to_png(&path) {
                    println!(
                        "failed convert {}, error: {}",
                        path.to_str().unwrap_or("?"),
                        e
                    );
                }
            }
        }
    }

    Ok(())
}
