use rayon::prelude::*;
use std::sync::Mutex;
use std::time::Instant;
use tinytemplate::TinyTemplate;
use systems::FileSystem;
use options::{ Options, OutputOption };
use std::sync::Arc;

pub mod options;
pub mod systems;

#[macro_use]
extern crate serde_derive;


#[derive(Serialize)]
pub struct FileStats {
    path: String,
    name: String,
    time_s: u64,

    size_mb: u64,
    size_b: u64,
}

pub fn run(path: &str, opts: Options, system: &Arc<dyn FileSystem>) {
    let recurse = if let OutputOption::All = opts.output { true } else { false }; 
    if recurse {
        println!("Reading size of path recursively {}", path);
    } else {
        println!("Reading size of path {}", path);
    }
    let results_mutex = Mutex::new(Vec::new());
    check_path(path, &opts, system, true, &results_mutex);

    let mut results = results_mutex.lock().unwrap();
    results.sort_by(|a, b| a.path.to_lowercase().cmp(&b.path.to_lowercase()));
    for stats in results.iter() {
        render_output(stats, &opts);
    }
}

fn render_output(stats: &FileStats, opts: &Options) {
    let mut template = TinyTemplate::new();
    template.add_template("template", &opts.template).unwrap();

    let rendered = template.render("template", &stats).unwrap();
    println!("{}", rendered);
}

fn check_path(
    path: &str,
    opts: &Options,
    system: &Arc<dyn FileSystem>,
    add: bool,
    results: &Mutex<Vec<FileStats>>,
) -> u64 {

    if !system.is_valid(path, opts) {
        return 0;
    }

    let start = Instant::now();
    let size: u64;
    if system.is_parent(path, opts) {
        match system.get_children(path, opts) {
            None => {
                return 0;
            }
            Some(files) => {
                let recurse = if let OutputOption::All = opts.output { true } else { false }; 
                if opts.verbose && !recurse {
                    println!("   Reading in parent {}", path);
                }
                let total: Mutex<u64> = Mutex::new(0);

                let file_process = |entry: &String| {
                    let size = check_path(entry, opts, &system.clone(), recurse, results);
                    let mut mut_total = total.lock().unwrap();
                    *mut_total += size;
                };

                if opts.multithread {
                    files.par_iter().for_each(file_process);
                } else {
                    files.iter().for_each(file_process);
                };
                size = *total.lock().unwrap();
            }
        }
    } else {
        match system.get_size(path, opts) {
            None => {
                return 0;
            }
            Some(s) => {
                size = s;
            }
        }
    }

    if add {
        let mut res_unlocked = results.lock().unwrap();
        let duration = start.elapsed();
        res_unlocked.push(FileStats {
            name: system.get_name(path, opts),
            path: String::from(path),

            time_s: duration.as_secs(),

            size_mb: size / 1000000,
            size_b: size,
        });
    }
    return size;
}