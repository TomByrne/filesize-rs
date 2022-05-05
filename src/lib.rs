use rayon::prelude::*;
use std::sync::Mutex;
use std::time::Instant;
use systems::FileSystem;
use options::{ Options, OutputOption, FileStats, Handlers };
use std::sync::Arc;

pub mod options;
pub mod systems;

#[macro_use]
extern crate serde_derive;

pub struct FileContext<'a> {
    parents_last: &'a Vec<bool>,
    depth: u32,
    index: u32,
    total: u32,
}

pub fn run<'a, T: Sync>(path: &str, opts: Options, handlers:Handlers<'a, T>, data:&'a T, system: &Arc<dyn FileSystem>) -> Vec<FileStats> {
    let recurse = if let OutputOption::All = opts.output { true } else { false }; 
    if opts.verbose {
        if recurse {
            println!("Reading size of path recursively {}", path);
        } else {
            println!("Reading size of path {}", path);
        }
    }
    let results_mutex = Mutex::new(Vec::new());
    let context = FileContext {
        parents_last: &Vec::new(),
        depth: 0,
        index: 0,
        total: 1,
    };
    check_path(path, &opts, &handlers, data, system, true, context, &results_mutex);
    
    let mut results = results_mutex.lock().unwrap();
    results.sort_by(|a, b| a.path.to_lowercase().cmp(&b.path.to_lowercase()));

    if let Some(handle) = &handlers.post {
        for stats in results.iter() {
            handle(stats.clone(), data);
        }
    }

    return results.to_vec();
}

fn check_path<'a, T: Sync>(
    path: &str,
    opts: &Options,
    handlers: &Handlers<'a, T>,
    data: &'a T,
    system: &Arc<dyn FileSystem>,
    output: bool,
    context: FileContext,
    results: &Mutex<Vec<FileStats>>,
) -> u64 {

    if !system.is_valid(path, opts) {
        return 0;
    }

    let is_dir = system.is_parent(path, opts);

    let mut stats = FileStats {
        name: system.get_name(path, opts),
        path: String::from(path),
        is_dir: is_dir,
        child_count: 0,
        has_children: false,

        depth: context.depth,
        index: context.index,
        total: context.total,
        first: context.index == 0,
        last: context.index == context.total-1,
        parents_last: context.parents_last.to_vec(),

        time_s: 0,

        size_mb: 0,
        size_b: 0,
    };

    if output {
        if let Some(handle) = &handlers.start {
            handle(stats.clone(), data);
        }
    }
    if opts.verbose {
        println!("check_path {} {}", path, is_dir);
    }

    let start = Instant::now();
    let size: u64;
    if is_dir {
        match system.get_children(path, opts) {
            None => {
                return 0;
            }
            Some(files) => {
                stats.child_count = files.len();
                stats.has_children = files.len() > 0;

                let recurse = if let OutputOption::All = opts.output { true } else { false }; 
                if opts.verbose && !recurse {
                    println!("   Reading in parent {}", path);
                }
                let total: Mutex<u64> = Mutex::new(0);
                let mut child_parents_last = context.parents_last.to_vec();
                child_parents_last.push(context.index == context.total-1);

                let file_process = |(i, entry): (usize, &String)| {
                    let child_context = FileContext {
                        parents_last: &child_parents_last,
                        depth: context.depth + 1,
                        index: i as u32,
                        total: files.len() as u32
                    };
                    let size = check_path(entry, opts, handlers, data, &system.clone(), recurse, child_context, results);
                    let mut mut_total = total.lock().unwrap();
                    *mut_total += size;
                    
                    if output {
                        if let Some(handle) = &handlers.prog {
                            let mut stats_copy = stats.clone();
                            update_stats(&mut stats_copy, &start, mut_total.clone());
                            handle(stats_copy, data);
                        }
                    }
                };

                if opts.multithread {
                    files.par_iter().enumerate().for_each(file_process);
                } else {
                    files.iter().enumerate().for_each(file_process);
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



    if output {
        update_stats(&mut stats, &start, size);
        
        if let Some(handle) = &handlers.end {
            handle(stats.clone(), data);
        }

        let mut res_unlocked = results.lock().unwrap();
        res_unlocked.push(stats);
    }
    return size;
}

fn update_stats(stats: &mut FileStats, start: &Instant, size: u64) {
    let duration = start.elapsed();
    stats.time_s = duration.as_secs();
    stats.size_mb = size / 1000000;
    stats.size_b = size;
}