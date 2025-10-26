// Import standard library modules
use std::fs::File;
use std::io::Read;
use std::sync::mpsc;
use std::{collections::HashSet, fs, io, path::Path};

// External crates
use chrono::{DateTime, Local}; // For handling file modified date/time
use clap::{Arg, Command};
use notify::{
    event::ModifyKind, Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher,
};
use sha2::{Digest, Sha256};

fn main() -> io::Result<()> {
    // CLI argument parsing using clap
    let matches = Command::new("FileOrganizer")
        .version("1.0")
        .about("Organize your files by type and date, with duplicate detection and watch mode.")
        .arg(
            Arg::new("path")
                .short('p')
                .long("path")
                .default_value("./Downloads")
                .help("Path to the folder you want to organize."),
        )
        .arg(
            Arg::new("watch")
                .short('w')
                .long("watch")
                .num_args(0)
                .help("Enable watch mode to auto-organize new files."),
        )
        .get_matches();

    // Extract values from the parsed CLI arguments
    let folder_path = matches.get_one::<String>("path").unwrap();
    let watch_mode = matches.get_flag("watch");

    //  Step 1: Organize all existing files once
    organize_files(folder_path)?;

    //  Step 2: If watch mode is enabled, keep watching for new files
    if watch_mode {
        println!("Watching for new files in {}", folder_path);
        watch_folder(folder_path)?;
    }

    Ok(())
}

///  Organizes files by type (images, videos, etc.) and modified date
/// Also detects duplicates based on file hash
fn organize_files(folder_path: &str) -> io::Result<()> {
    let all_files = fs::read_dir(folder_path)?; // Read all entries in the directory
    let mut seen_hashes = HashSet::new(); // Track hashes to detect duplicates

    for entry in all_files {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            //  Step 1: Hash the file to check for duplicates
            let hash = hash_file(&path)?;
            if seen_hashes.contains(&hash) {
                println!(" Duplicate found: {:?}", path.file_name().unwrap());
                move_to_folder(&path, folder_path, "duplicates")?;
                continue;
            } else {
                seen_hashes.insert(hash);
            }

            // 🔍 Step 2: Identify file type by extension
            let extension = path
                .extension()
                .and_then(|ext| ext.to_str())
                .unwrap_or("")
                .to_lowercase();

            // Match extension to a target folder
            let target_folder = match extension.as_str() {
                "jpg" | "jpeg" | "png" | "bmp" | "tiff" => "images",
                "gif" => "gifs",
                "mp4" | "mov" | "avi" | "mkv" => "videos",
                "mp3" | "wav" | "flac" => "audio",
                "pdf" | "docx" | "txt" => "documents",
                "zip" | "rar" | "7z" => "archives",
                _ => "others",
            };

            //  Step 3: Create date-based subfolder like images/2025-10-25
            let metadata = fs::metadata(&path)?;
            let modified_time: DateTime<Local> = metadata.modified()?.into();
            let date_str = modified_time.format("%Y-%m-%d").to_string();
            let date_folder = format!("{}/{}", target_folder, date_str);

            //  Step 4: Move file to new location
            move_to_folder(&path, folder_path, &date_folder)?;
        }
    }
    Ok(())
}

///  Moves a file into its destination folder
/// If folder doesn’t exist, it creates it
fn move_to_folder(path: &Path, base_folder: &str, subfolder: &str) -> io::Result<()> {
    // Combine base folder + subfolder name → final path
    let path_for_new_folder = Path::new(base_folder).join(subfolder);
    if !path_for_new_folder.exists() {
        fs::create_dir_all(&path_for_new_folder)?; // Create nested directories if missing
    }

    let file_name = path.file_name().unwrap(); // Extract file name
    let new_location = path_for_new_folder.join(file_name);

    // Only move if file doesn’t already exist in destination
    if !new_location.exists() {
        fs::rename(path, &new_location)?;
        println!(" Moved {:?} → {:?}", file_name, new_location);
    }
    Ok(())
}

/// Hash the contents of a file using SHA256
/// Used to detect duplicates
fn hash_file(path: &Path) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0; 4096]; // Read in chunks to handle large files

    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    // Convert hash result to hex string
    Ok(format!("{:x}", hasher.finalize()))
}

fn watch_folder(folder_path: &str) -> io::Result<()> {
    // Create an mpsc channel to receive file system events
    let (tx, rx) = mpsc::channel();

    //  New API: Pass config instead of Duration
    let mut watcher =
        RecommendedWatcher::new(tx, Config::default()).expect("Failed to initialize watcher");

    //  Convert folder_path to &Path
    watcher
        .watch(Path::new(folder_path), RecursiveMode::NonRecursive)
        .expect("Failed to start watching folder");

    println!("👀 Watching folder: {}", folder_path);

    // Loop to handle events
    for res in rx {
        match res {
            Ok(Event { kind, .. }) => {
                // Only trigger on file creation or modification events
                if matches!(
                    kind,
                    EventKind::Create(_) | EventKind::Modify(ModifyKind::Data(_))
                ) {
                    println!(" New file detected. Reorganizing...");
                    if let Err(e) = organize_files(folder_path) {
                        eprintln!(" Error during reorganization: {:?}", e);
                    }
                }
            }
            Err(e) => println!(" Watch error: {:?}", e),
        }
    }

    Ok(())
}
