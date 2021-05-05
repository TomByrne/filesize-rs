use clap::app_from_crate;
use std::fs::{ metadata, read_dir };
use std::path::Path;

struct FilesizeOpts {
    verbose: bool,
    recurse: bool,
}

fn main() {
    let matches = app_from_crate!()
        .arg("-r, --recurse 'The file/folder path to check'")
        .arg("-v, --verbose 'Whether to print verbose logs'")
        .arg("<path> 'The file/folder path to check'")
        .get_matches();

    // You can check the value provided by positional arguments, or option arguments
    let path = matches.value_of("path").unwrap();

    let opts = FilesizeOpts {
        verbose: (matches.occurrences_of("verbose") > 0),
        recurse: matches.occurrences_of("recurse") > 0,
    };

    if opts.recurse { println!("Reading size of path recursively {}", path); }
    else { println!("Reading size of path {}", path); }
    
    get_size(path, &opts, Some(&size_read));
}

fn size_read(path:&Path, size:u64) {
    println!("Size of {} is {}mb", path.display(), size / 1000000);
}

fn get_size<P: AsRef<Path>+Copy>(path:P, opts:&FilesizeOpts, cb: Option<&dyn Fn(&Path, u64)>) -> u64 {
    let meta = metadata(path).unwrap();
    let size;
    if meta.is_dir() {

        let files = read_dir(path).unwrap();
        
        if opts.verbose && !opts.recurse { println!("   Reading in dir {}", path.as_ref().display()); }

        let mut total = 0;
        let child_cb = if opts.recurse { cb } else { None };
        for entry in files {
            total += get_size(&entry.unwrap().path().as_path(), opts, child_cb)
        }
        size = total;
    }else{
        size = meta.len();
    }
    if let Some(callback) = cb {
        callback(path.as_ref(), size);
    }
    return size;
}