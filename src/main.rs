use clap::{app_from_crate};

fn main() {
    let matches = app_from_crate!("myapp")
        .arg("<output> 'Sets an optional output file'")
        .get_matches();

    // You can check the value provided by positional arguments, or option arguments
    if let Some(o) = matches.value_of("output") {
        println!("Value for output: {}", o);
    }
}
