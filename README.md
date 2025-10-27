# FileOrganizer

**FileOrganizer** is a Rust-based CLI tool to automatically organize your files by **type** and **modified date**, with **duplicate detection** and an optional **watch mode** for real-time organization.

## Working demo

<img width="1289" height="414" alt="Screenshot 2025-10-25 at 5 45 58 PM" src="https://github.com/user-attachments/assets/c0dd5ad2-56cf-49c3-b170-07a263f853c4" />

## After it works file structure looks like this 

<img width="418" height="183" alt="Screenshot 2025-10-25 at 6 00 04 PM" src="https://github.com/user-attachments/assets/3fbd271e-9821-4aea-ae83-349d0678bb32" />

---

## Usage / Testing

You can run the program with different flags. Below is a table of common test commands:

| Command | Description |
|---------|-------------|
| `cargo run -- --path ./test-folder` | Organize files in `./test-folder` once, without watching. |
| `cargo run -- --path ./test-folder --watch` | Organize files in `./test-folder` and continuously watch for new files. |


---


## Features

- 🗂️ Organizes files into folders by type:
  - Images: `jpg`, `jpeg`, `png`, `bmp`, `tiff`
  - GIFs: `gif`
  - Videos: `mp4`, `mov`, `avi`, `mkv`
  - Audio: `mp3`, `wav`, `flac`
  - Documents: `pdf`, `docx`, `txt`
  - Archives: `zip`, `rar`, `7z`
  - Others: anything else
- Creates **date-based subfolders** (e.g., `images/2025-10-25`)
- Detects **duplicate files** using SHA256 hashing
- Optional **watch mode** to auto-organize new files in real time
- Fast and lightweight using Rust standard library and powerful crates

---

## TODO / Planned Features

- **Metadata-based sorting (`--metadata`)**
  
  If `--metadata` is used:
  - The program will try to read EXIF metadata (like “Date Taken”) from image files.
  - It will sort images into folders based on that date, instead of just using file type or last modified date.

  If `--metadata` is **not** used:
  - The program falls back to regular sorting based on file extension and last modified date.


- **Upload to cloud / Google Drive via CLI** *(Planned)*

  - Integrate CLI commands to upload organized files to Google Drive or other cloud storage.
  - This will allow automatic backup after organization.

- **Future enhancements**
  - Support for more file types and custom folder structures.
  - Improved error handling and logging.
  - Configurable rules via a `.toml` or `.yaml` config file.




## Installation

1. Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed.
2. Clone this repository: https://github.com/praptisharma28/FileOrganizer.git

```bash
git clone https://github.com/praptisharma28/FileOrganizer.git
cd FileOrganizer
cargo build 

