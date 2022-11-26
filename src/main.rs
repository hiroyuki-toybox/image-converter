use clap::Parser;
use image::io::Reader as ImageReader;

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

fn convert_jpg_to_png(path: &std::path::PathBuf) -> Result<(), image::ImageError> {
    let img = ImageReader::open(path)?.decode()?;
    let mut new_path = path.to_path_buf();
    println!("{} -> {}", path.display(), new_path.display());
    new_path.set_extension("png");
    img.save(new_path)?;

    Ok(())
}

fn convert_manager(path: &std::path::Path) -> Result<(), std::io::Error> {
    if !path.is_dir() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Not a directory",
        ));
    }

    let dir_item = path.read_dir()?;

    for item in dir_item {
        let item = item?;
        let path = item.path();
        if path.is_dir() {
            convert_manager(&path)?;
        }

        if path.extension().unwrap() == "jpg" {
            if let Err(e) = convert_jpg_to_png(&path) {
                println!("failed convert {}, error: {}", path.to_str().unwrap(), e);
            }
        }
    }

    Ok(())
}
