use std::fs::{self, DirEntry};
use std::path::Path;
use std::process::Command;

fn optimize_img(path: &DirEntry, root_path: &Path) {
    let path_binding = path.path();
    let root_str = root_path.to_str().expect("Path should exist.");
    let name_binding = path.file_name();
    let path_name = path_binding.to_str().expect("Filename should exist.");
    let current_data = name_binding.to_str().expect("Current path should exist.");

    let output_path = format!(
        "{}{}{}",
        root_str,
        "/optimized/",
        name_binding.to_string_lossy()
    );

    let new_dir = format!("{}/{}", root_str, "optimized");
    let path = Path::new(&new_dir);

    if !path.exists() {
        println!("Created missing folder: {:?}", path);
        let _ = fs::create_dir_all(path);
    }

    if current_data.contains(".webp") {
        println!("optimizing {:?} ...", name_binding);

        let optimize = Command::new("cwebp")
            .args(["-q", "80", path_name, "-o", &output_path])
            .output()
            .expect("Failed to optimize image");

        match optimize.status.success() {
            true => println!("Optimized {}", path_name),
            false => println!("Error {}", String::from_utf8_lossy(&optimize.stderr)),
        }
    } else if !path_name.contains("optimized") {
        println!("Entering subfolder: {}", path_name);
        image_optimizer(Path::new(path_name));
    }
}

fn image_optimizer(root_path: &Path) {
    let paths = fs::read_dir(root_path);
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
        .for_each(|path| optimize_img(path, root_path));
}

fn main() {
    let root_path = Path::new("./src/images");
    image_optimizer(root_path);
}
