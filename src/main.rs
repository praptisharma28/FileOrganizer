//code to organize files into images, gifs and videos etc. using idiomatic rust

use std::{env, fs, io, path::Path};


fn main()-> io::Result<()>{ //to read from files, its almost same as result

   let args : Vec<String> = env::args().collect(); // get cli arguments
   //check if user has provided a path
   let folder_path= if args.len() > 1{
    &args[1]
   }else{
  "./Downloads"
   };
    println!("Organizing files in: {}", folder_path);
    
    match organize_files(folder_path) {
        Ok(()) => println!("Files organized successfully!"),
        Err(e) => println!("Error organizing files: {}", e),
    }
    Ok(())

}

fn organize_files(folder_path:&str)-> io::Result<()>{
    let all_files=fs::read_dir(folder_path)?; // read all files in the directory
    for files in all_files{
        let file= files?;
        let path=file.path();
        if path.is_file(){ //checking if a path is a file
            let extension = path.extension().and_then(|ext| ext.to_str()).unwrap_or("").to_lowercase();
            let target_folder = match extension.as_str(){
                "jpg" |"jpeg" | "png"| "bmp" | "tiff" => "images",
                "gif" => "gifs",
                "mp4"| "mov" | "avi"| "mkv" => "videos",
                "mp3" | "wav"| "flac" => "audio",
                "pdf" | "docx"| "txt" => "documents",
                "zip"| "rar" | "7z" => "archives",
                _ => "others",
            };
            let pathfornewfolder = Path::new(folder_path).join(target_folder);
            if !pathfornewfolder.exists(){
                fs::create_dir(&pathfornewfolder)?; // create new directory if not exists
            }
            let file_name = path.file_name().unwrap(); //get the file name
            let new_location =pathfornewfolder.join(file_name); //new location for the file
            fs::rename(&path, &new_location).unwrap(); //move the file to new location
        
        }
    }
    Ok(())
}