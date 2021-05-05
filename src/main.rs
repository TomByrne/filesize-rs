use clap::app_from_crate;
use std::fs::{ metadata, read_dir };
use std::path::Path;

fn main() {
    let matches = app_from_crate!()
        // .arg("-r, --recurse 'The file/folder path to check'")
        .arg("-v, --verbose 'Whether to print verbose logs'")
        .arg("<path> 'The file/folder path to check'")
        .get_matches();

    // You can check the value provided by positional arguments, or option arguments
    let path = matches.value_of("path").unwrap();
    
    // let recurse =  matches.occurrences_of("recurse") > 0;
    let verbose =  matches.occurrences_of("verbose") > 0;

    println!("Reading size of path {}", path);
    let size = get_size(path, &verbose);
    println!("Size of {} is {}mb", path, size / 1000000);
}

fn get_size<P: AsRef<Path>+Copy>(path:P, verbose:&bool) -> u64 {
    let meta = metadata(path).unwrap();
    if meta.is_dir() {

        let files = read_dir(path).unwrap();
        
        if *verbose { println!("   Reading in dir {}", path.as_ref().display()); }

        let mut total = 0;

        for entry in files {
            total += get_size(&entry.unwrap().path(), verbose)
        }
        return total;
    }else{
        return meta.len();
    }
}