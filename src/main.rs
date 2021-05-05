use clap::{ app_from_crate, Arg };
use std::fs::{ metadata, read_dir };
use std::path::Path;
use tinytemplate::TinyTemplate;

#[macro_use]
extern crate serde_derive;

struct FilesizeOpts {
    verbose: bool,
    recurse: bool,
    template: String,
}

#[derive(Serialize)]
struct TemplateContext {
    path: String,
    name: String,
    size_mb: u64,
    size: u64,
}

fn main() {
    let matches = app_from_crate!()
        .arg(
            Arg::new("template")
                .about("Template for output")
                .short('t')
                .long("template") 
                .required(false)
                .default_value("Size of {path} is {size_mb}mb")
        )
        .arg("-r, --recurse 'The file/folder path to check'")
        .arg("-v, --verbose 'Whether to print verbose logs'")
        .arg("<path> 'The file/folder path to check'")
        .get_matches();

    // You can check the value provided by positional arguments, or option arguments
    let path = matches.value_of("path").unwrap();

    let opts = FilesizeOpts {
        verbose: (matches.occurrences_of("verbose") > 0),
        recurse: matches.occurrences_of("recurse") > 0,
        template: String::from(matches.value_of("template").unwrap()),
    };

    if opts.recurse { println!("Reading size of path recursively {}", path); }
    else { println!("Reading size of path {}", path); }
    
    get_size(path, &opts, Some(&size_read));
}

fn size_read(path:&Path, size:u64, opts:&FilesizeOpts) {
    let mut template = TinyTemplate::new();
    template.add_template("template", &opts.template).unwrap();

    let context = TemplateContext {
        name: String::from(path.file_name().unwrap().to_str().unwrap()),
        path: String::from(path.to_str().unwrap()),
        size_mb: size / 1000000,
        size: size
    };

    let rendered = template.render("template", &context).unwrap();
    println!("{}", rendered);
}

fn get_size<P: AsRef<Path>+Copy>(path:P, opts:&FilesizeOpts, cb: Option<&dyn Fn(&Path, u64, &FilesizeOpts)>) -> u64 {
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
        callback(path.as_ref(), size, opts);
    }
    return size;
}