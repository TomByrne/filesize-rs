use std::str::FromStr;

// #[derive(Debug)]
pub struct Options<'a> {
    pub verbose: bool,
    pub output: OutputOption,
    pub multithread: bool,

    pub context:&'a str,
    pub handle: HandlerOption,

    pub context_start:&'a str,
    pub handle_start: HandlerOption,

    pub context_prog:&'a str,
    pub handle_prog: HandlerOption,

    pub context_end:&'a str,
    pub handle_end: HandlerOption,
}

pub type HandlerOption = Option<fn(f:FileStats, c:&str)->()>;
// pub type HandlerOption = Option<&'static (dyn Fn(FileStats) -> ())>;
// pub type HandlerOption = &'static dyn Fn(FileStats) -> ();


#[derive(Serialize)]
#[derive(Clone)]
pub struct FileStats {
    pub path: String,
    pub name: String,
    pub is_dir: bool,
    pub has_children: bool,
    
    pub depth: u32,
    pub index: u32,
    pub total: u32,
    pub first: bool,
    pub last: bool,
    pub parents_last: Vec<bool>,

    pub time_s: u64,

    pub size_mb: u64,
    pub size_b: u64,
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