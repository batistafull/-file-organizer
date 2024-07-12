use std::env;
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    for entry in fs::read_dir(".")? {
        let path = entry?.path();
        if path.is_file() {
            let path_name = path.display();
            let file_path = path.extension();

            match file_path {
                Some(ext) => {
                    let exe_path =
                        env::current_exe().expect("Error");
                    let exe_name = exe_path.file_name().unwrap().to_str().unwrap();

                    if exe_name != path_name.to_string() {
                        let dir_name = format!("Folder {}", ext.to_str().unwrap());
                        let _ = fs::create_dir(dir_name.clone());
                        let old_path = format!("{}", path_name);
                        let new_path = format!("{}/{}", dir_name, path_name);
                        let _ = fs::rename(old_path, new_path);
                    }
                }
                None => println!(""),
            };
        }
    }
    Ok(())
}