use std::fs::{metadata, read_dir, read_link, DirEntry};
use std::path::Path;
use crate::options::Options;
use crate::systems::FileSystem;

#[derive(Clone, Copy)]
pub struct Fs {}

impl FileSystem for Fs {
    fn is_valid(&self, path: &str, opts: &Options) -> bool
    {
        // Check if this is a symlink, and abort if so (would need to solve symlink-loops)
        if let Ok(_) = read_link(path) {
            if opts.verbose {
                println!("   Skipping symlink {}", path);
            }
            return false;
        }
        return true;
    }

    fn is_parent(&self, path: &str, _opts: &Options) -> bool
    {
        let as_path = Path::new(path);
        return as_path.is_dir();
    }
    
    fn get_children(&self, path: &str, opts: &Options) -> Option<Vec<String>>
    {
        match read_dir(path) {
            Err(e) => {
                if opts.verbose {
                    println!("Error reading dir ({}) {}", path, e)
                };
                return None;
            }

            Ok(files) => {
                let mut file_vec = Vec::new();
                for entry in files {
                    let entry: DirEntry = entry.unwrap();
                    file_vec.push(String::from(entry.path().to_str().unwrap()));
                }

                return Some(file_vec);
            }
        }
    }
    
    // TODO: Fix this up
    fn get_size(&self, path: &str, opts: &Options) -> u64
    {
        match metadata(path) {
            Err(err) => {
                if opts.verbose {
                    println!(
                        "Failed to read file metadata ({}) {}",
                        path,
                        err
                    )
                };
                return 0;
            }
            Ok(meta) => return meta.len()
        }
    }
    
    
    fn get_name(&self, path: &str, _opts: &Options) -> String
    {
        let as_path = Path::new(path);
        return String::from(as_path.file_name().unwrap().to_str().unwrap());
    }
}