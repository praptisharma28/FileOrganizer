//code to organize files into images, gifs and videos etc. using idiomatic rust

use clap::Parser;
use std::{
    fs, io,
    path::{Path, PathBuf},
};

#[derive(Parser, Debug)]
#[command(version = "0.1.0", about = "file_organizer", long_about = None)]
struct Args {
    #[arg(short, long, help = "Sets the file path to use")]
    path: PathBuf,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    organize_files(&args.path)?;

    Ok(())
}

fn categorize_file(ext: &str) -> &str {
    match ext {
        "jpg" | "jpeg" | "png" | "bmp" | "tiff" => "images",
        "gif" => "gifs",
        "mp4" | "mov" | "avi" | "mkv" => "videos",
        "mp3" | "wav" | "flac" => "audio",
        "pdf" | "docx" | "txt" => "documents",
        "zip" | "rar" | "7z" => "archives",
        _ => "others",
    }
}

fn organize_files(folder_path: &Path) -> io::Result<()> {
    for files in fs::read_dir(folder_path)? {
        let file = files?;

        let path = file.path();

        //checking if a path is a file
        if path.is_file() {
            let extension = path
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("")
                .to_lowercase();

            let target_folder = categorize_file(&extension);

            let new_folder_path = folder_path.join(target_folder);

            // create new directory if not exists
            if !new_folder_path.exists() {
                fs::create_dir(&new_folder_path)?;
            }

            if let Some(file_name) = path.file_name() {
                let new_location = new_folder_path.join(file_name);
                fs::rename(path, new_location)?; //move the file to new location
            }
        }
    }
    Ok(())
}
