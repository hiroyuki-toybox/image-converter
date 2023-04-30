use anyhow::Result;
use clap::Parser;
use image::io::Reader as ImageReader;
use thiserror::Error;

#[derive(Debug, Error)]
enum CliError {
    #[error("ディレクトリではありません")]
    NotADirectory,
    #[error("ファイルの変換に失敗しました: [{0}]")]
    ConvertError(String),
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

fn convert_jpg_to_png(path: &std::path::PathBuf) -> Result<(), CliError> {
    let img = ImageReader::open(path)
        .map_err(|_| CliError::NotADirectory)
        .and_then(|reader| {
            reader
                .decode()
                .map_err(|err| CliError::ConvertError(err.to_string()))
        })?;
    let mut new_path = path.to_path_buf();
    println!("{} -> {}", path.display(), new_path.display());
    new_path.set_extension("png");
    img.save(new_path)
        .map_err(|err| CliError::ConvertError(err.to_string()))?;

    Ok(())
}

fn convert_manager(path: &std::path::Path) -> Result<(), CliError> {
    if !path.is_dir() {
        return Err(CliError::NotADirectory);
    }

    let dir_item = path.read_dir().map_err(|_| CliError::NotADirectory)?;

    for item in dir_item {
        let path = item.map_err(|_| CliError::NotADirectory)?.path();
        if path.is_dir() {
            convert_manager(&path)?;
        } else if let Some(extension) = path.extension() {
            if extension == "jpg" {
                convert_jpg_to_png(&path)?;
            }
        }
    }

    Ok(())
}
