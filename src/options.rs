use std::str::FromStr;
// use std::marker::PhantomData;

// #[derive(Debug)]
pub struct Options {
    pub verbose: bool,
    pub output: OutputOption,
    pub multithread: bool,
}

// #[derive(Debug)]
pub struct Handlers<'a, T : 'a + Sized + Sync> {
    pub post: HandlerOption<&'a T>,
    pub start: HandlerOption<&'a T>,
    pub prog: HandlerOption<&'a T>,
    pub end: HandlerOption<&'a T>,

    // phantom: PhantomData<&'a T>,
}

pub type HandlerOption<T> = Option<fn(f:FileStats, c:T)->()>;
// pub type HandlerOption = Option<&'static (dyn Fn(FileStats) -> ())>;
// pub type HandlerOption = &'static dyn Fn(FileStats) -> ();


#[derive(Serialize)]
#[derive(Clone)]
pub struct FileStats {
    pub path: String,
    pub name: String,
    pub is_dir: bool,
    pub child_count: usize,
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