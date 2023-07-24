use std::fs::{self, DirEntry};
use std::path::Path;
use std::process::Command;

fn optimize_img(path: &DirEntry, path_str: &str) {
    let path_binding = path.path();
    let name_binding = path.file_name();
    let path_name = path_binding.to_str().unwrap();
    let current_data = name_binding.to_str().unwrap();

    println!("{:?}", name_binding);
    let output_path = format!(
        "{}{}{}",
        path_str,
        "/optimized/",
        name_binding.to_string_lossy()
    );

    let new_dir = format!("{}/{}", path_str, "optimized");
    let path = Path::new(&new_dir);

    if !path.exists() {
        println!("Created missing folder: {:?}", path);
        let _ = fs::create_dir_all(&path);
    }

    if current_data.contains(".webp") {
        let optimize = Command::new("cwebp")
            .args(["-q", "80", path_name, "-o", output_path.as_str()])
            .output()
            .expect("Failed to optimize image.");

        if optimize.status.success() {
            let s = String::from_utf8_lossy(&optimize.stdout);
            print!("Optimized: {}", s);
        } else {
            let s = String::from_utf8_lossy(&optimize.stderr);
            print!("Error: {}", s);
        }
    } else {
        if !path_name.contains("optimized") {
            println!("Entering subfolder: {}", path_name);
            image_optimizer(path_name);
        }
    }
}

fn image_optimizer(path_str: &str) {
    let paths = fs::read_dir(path_str);
    let mut path_vec: Vec<DirEntry> = Vec::new();

    match paths {
        Ok(paths) => paths.into_iter().for_each(|path| match path {
            Ok(file_path) => path_vec.push(file_path),
            Err(e) => println!("{}", e),
        }),
        Err(e) => println!("{}", e),
    }

    path_vec
        .iter()
        .for_each(|path| optimize_img(path, path_str));
}

fn main() {
    let root_path = "./src/images";
    image_optimizer(root_path);
}
