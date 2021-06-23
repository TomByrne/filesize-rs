use clap::{app_from_crate, Arg};
use fstat::{run, RunOptions};

fn main() {
    let matches = app_from_crate!()
        .arg(
            Arg::new("template")
                .about("Template for output")
                .short('t')
                .long("template")
                .required(false)
                .default_value("Size of {path} is {size_mb}mb"),
        )
        .arg("-r, --recurse 'The file/folder path to check'")
        .arg("-v, --verbose 'Whether to print verbose logs'")
        .arg("-s, --single-thread 'Whether to skip multi-threading (performance check)'")
        .arg("<path> 'The file/folder path to check'")
        .get_matches();

    let path = matches.value_of("path").unwrap();

    let opts = RunOptions {
        multithread: (matches.occurrences_of("single-thread") == 0),
        verbose: (matches.occurrences_of("verbose") > 0),
        recurse: matches.occurrences_of("recurse") > 0,
        template: String::from(matches.value_of("template").unwrap()),
    };

    run(path, opts);
}