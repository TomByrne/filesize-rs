use std::str::FromStr;

#[derive(Debug)]
pub struct Options<'a> {
    pub verbose: bool,
    pub output: OutputOption,
    pub multithread: bool,

    pub template: Option<&'a str>,
    pub template_start: Option<&'a str>,
}

#[derive(Debug)]
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