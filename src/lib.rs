use rayon::prelude::*;
use std::fs::{metadata, read_dir, read_link, DirEntry};
use std::path::Path;
use std::sync::Mutex;
use std::time::Instant;
use tinytemplate::TinyTemplate;

#[macro_use]
extern crate serde_derive;

pub struct RunOptions {
    pub verbose: bool,
    pub recurse: bool,
    pub multithread: bool,
    pub template: String,
}

#[derive(Serialize)]
pub struct FileStats {
    path: String,
    name: String,
    time_s: u64,

    size_mb: u64,
    size_b: u64,
}

pub fn run(path: &str, opts: RunOptions) {
    if opts.recurse {
        println!("Reading size of path recursively {}", path);
    } else {
        println!("Reading size of path {}", path);
    }
    let results_mutex = Mutex::new(Vec::new());
    check_path(path, &opts, true, &results_mutex);

    let results: &Vec<FileStats> = &results_mutex.lock().unwrap();
    for stats in results {
        size_read(stats, &opts);
    }
}

fn size_read(stats: &FileStats, opts: &RunOptions) {
    let mut template = TinyTemplate::new();
    template.add_template("template", &opts.template).unwrap();

    let rendered = template.render("template", &stats).unwrap();
    println!("{}", rendered);
}

fn check_path<P: AsRef<Path> + Copy>(
    path: P,
    opts: &RunOptions,
    add: bool,
    results: &Mutex<Vec<FileStats>>,
) -> u64 {
    // Check if this is a symlink, and abort if so (would need to solve symlink-loops)
    if let Ok(_) = read_link(path) {
        if opts.verbose {
            println!("   Skipping symlink {}", path.as_ref().display());
        }
        return 0;
    }

    let start = Instant::now();
    let size: u64;
    if path.as_ref().is_dir() {
        let dir_read = read_dir(path);
        match dir_read {
            Err(e) => {
                if opts.verbose {
                    println!("Error reading dir ({}) {}", path.as_ref().display(), e)
                };
                return 0;
            }
            Ok(files) => {
                if opts.verbose && !opts.recurse {
                    println!("   Reading in dir {}", path.as_ref().display());
                }
                let total: Mutex<u64> = Mutex::new(0);
                let mut file_vec = Vec::new();
                for entry in files {
                    file_vec.push(entry.unwrap());
                }

                let file_process = |entry: &DirEntry| {
                    let size = check_path(entry.path().as_path(), opts, opts.recurse, results);
                    let mut mut_total = total.lock().unwrap();
                    *mut_total += size;
                };

                if opts.multithread {
                    file_vec.par_iter().for_each(file_process);
                } else {
                    file_vec.iter().for_each(file_process);
                };
                size = *total.lock().unwrap();
            }
        }
    } else {
        match metadata(path) {
            Err(err) => {
                if opts.verbose {
                    println!(
                        "Failed to read file metadata ({}) {}",
                        path.as_ref().display(),
                        err
                    )
                };
                return 0;
            }
            Ok(meta) => {
                size = meta.len();
            }
        }
    }

    if add {
        let as_path = path.as_ref();
        let mut res_unlocked = results.lock().unwrap();
        let duration = start.elapsed();
        res_unlocked.push(FileStats {
            name: String::from(as_path.file_name().unwrap().to_str().unwrap()),
            path: String::from(as_path.to_str().unwrap()),

            time_s: duration.as_secs(),

            size_mb: size / 1000000,
            size_b: size,
        });
    }
    return size;
}