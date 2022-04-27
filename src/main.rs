use clap::{app_from_crate, Arg};
use fstat::run;
use fstat::options::{ Options, FileStats, Handlers };
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

    do_matches(matches);
}

fn do_matches(matches: clap::ArgMatches) {

    // println!("{:?}", matches);

    let path = matches.value_of("path").unwrap();
    let output = matches.value_of_t("output").unwrap_or_else(|e| e.exit());

    let opts = Options {
        multithread: (matches.occurrences_of("single-thread") == 0),
        verbose: (matches.occurrences_of("verbose") > 0),
        output: output
    };

    let mut handlers: Handlers<TemplateData> = Handlers {
        post: None,
        start: None,
        prog: None,
        end: None
    };

    let empty = "";

    let mut data = TemplateData {
        post: &empty,
        start: &empty,
        prog: &empty,
        end: &empty,
    };

    if let Some(template) = matches.value_of("template") {
        data.post = &template;
        handlers.post = Some(render_template_post);
    }

    if let Some(template) = matches.value_of("template-start") {
        data.start = &template;
        handlers.start = Some(render_template_start);
    }

    if let Some(template) = matches.value_of("template-prog") {
        data.prog = &template;
        handlers.prog = Some(render_template_prog);
    }

    if let Some(template) = matches.value_of("template-end") {
        data.end = &template;
        handlers.end = Some(render_template_end);
    }

    let fsys = matches.value_of("file-system").unwrap();

    if opts.verbose {
        println!("Running at '{}' (fs={}) multithread:{} verbose:{}", path, fsys, opts.multithread, opts.verbose);
    }
    
    let fs: Arc<dyn FileSystem> = Arc::new(match &fsys.to_lowercase()[..] {
        "std" => fstat::systems::fs::Fs {},
        _ => panic!("no fs match"), // Clap prevents this from ever happening
    });

    run(path, opts, handlers, &data, &fs);
}

struct TemplateData<'a> {
    post: &'a str,
    start: &'a str,
    prog: &'a str,
    end: &'a str,
}


fn render_template_post(stats: FileStats, data: &TemplateData) {
    render_template(stats, data.post);
}
fn render_template_start(stats: FileStats, data: &TemplateData) {
    render_template(stats, data.start);
}
fn render_template_prog(stats: FileStats, data: &TemplateData) {
    render_template(stats, data.prog);
}
fn render_template_end(stats: FileStats, data: &TemplateData) {
    render_template(stats, data.end);
}

fn render_template(stats: FileStats, template: &str) {
    let mut tiny_template = TinyTemplate::new();
    tiny_template.add_template("template", template).unwrap();

    let rendered = tiny_template.render("template", &stats).unwrap();
    println!("{}", rendered);
}