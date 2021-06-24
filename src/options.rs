use std::str::FromStr;

// mod systems;

pub struct Options {
    pub verbose: bool,
    pub output: OutputOption,
    pub multithread: bool,
    pub template: String,
    // pub system: Arc<systems::FileSystem>,
}

pub enum OutputOption {
    Root,
    All
}
impl FromStr for OutputOption {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s.to_lowercase()[..] {
            "root" => Ok(OutputOption::Root),
            "all" => Ok(OutputOption::All),
            _ => Err("no match"),
        }
    }
}