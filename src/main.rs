use clap::{app_from_crate, Arg};
use fstat::run;
use fstat::options::{ Options, FileStats };
use fstat::systems::FileSystem;
use std::sync::Arc;
use tinytemplate::TinyTemplate;


fn main() {
    let matches = app_from_crate!()
        .arg(
            Arg::new("template")
                .about("Template for output generated after all processing is finished")
                .short('t')
                .long("template")
                .takes_value(true)
        )
        
        .arg(
            Arg::new("template-start")
                .about("Template for output generated when a file/folder begins being processed")
                .long("template-start")
                .takes_value(true)
        )
        
        .arg(
            Arg::new("template-prog")
                .about("Template for output generated while a folder is being processed (after each child returns)")
                .long("template-prog")
                .takes_value(true)
        )
        
        .arg(
            Arg::new("template-end")
                .about("Template for output generated after a file/folder is finished being processed (but before all items are completed)")
                .long("template-end")
                .takes_value(true)
        )
        
        .arg(
            Arg::new("output")
                .about("Which entries to run output (may need to recurse regardless).\nroot = Just the entity specified by the path arg.\nall = All descendants of the entity specified by the path arg.")
                .short('o')
                .long("output")
                .required(false)
                .possible_values(&["root", "all"])
                .default_value("root")
        )
        
        .arg(
            Arg::new("file-system")
                .about("Which file system integration to use.\nstd = Standard file system IO")
                .short('f')
                .long("file-system")
                .required(false)
                .possible_values(&["std"])
                .default_value("std")
        )

        .arg("-v, --verbose 'Whether to print verbose logs'")
        .arg("-s, --single-thread 'Whether to avoid multi-threading'")
        .arg("<path> 'The file/folder path to check'")
        .get_matches();

    // println!("{:?}", matches);

    let path = matches.value_of("path").unwrap();
    let output = matches.value_of_t("output").unwrap_or_else(|e| e.exit());

    let empty = "";

    let mut opts = Options {
        multithread: (matches.occurrences_of("single-thread") == 0),
        verbose: (matches.occurrences_of("verbose") > 0),
        output: output,

        context: &empty,
        handle: None,

        context_start: &empty,
        handle_start: None,

        context_prog: &empty,
        handle_prog: None,

        context_end: &empty,
        handle_end: None
    };

    if let Some(template) = matches.value_of("template") {
        opts.context = &template;
        opts.handle = Some(render_template);
    }

    if let Some(template) = matches.value_of("template-start") {
        opts.context_start = &template;
        opts.handle_start = Some(render_template);
    }

    if let Some(template) = matches.value_of("template-prog") {
        opts.context_prog = &template;
        opts.handle_prog = Some(render_template);
    }

    if let Some(template) = matches.value_of("template-end") {
        opts.context_end = &template;
        opts.handle_end = Some(render_template);
    }

    let fsys = matches.value_of("file-system").unwrap();

    if opts.verbose {
        println!("Running at '{}' (fs={}) multithread:{} verbose:{}", path, fsys, opts.multithread, opts.verbose);
    }
    
    let fs: Arc<dyn FileSystem> = Arc::new(match &fsys.to_lowercase()[..] {
        "std" => fstat::systems::fs::Fs {},
        _ => panic!("no fs match"), // Clap prevents this from ever happening
    });

    run(path, opts, &fs);
}


fn render_template(stats: FileStats, template: &str) {
    let mut tiny_template = TinyTemplate::new();
    tiny_template.add_template("template", template).unwrap();

    let rendered = tiny_template.render("template", &stats).unwrap();
    println!("{}", rendered);
}