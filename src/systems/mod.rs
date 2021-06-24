pub mod fs;

use crate::options::Options;

pub trait FileSystem: Send + Sync
{
    fn is_valid(&self, path: &str, opts: &Options) -> bool;
    fn is_parent(&self, path: &str, opts: &Options) -> bool;
    fn get_children(&self, path: &str, opts: &Options) -> Option<Vec<String>>;
    fn get_size(&self, path: &str, opts: &Options) -> Option<u64>;
    fn get_name(&self, path: &str, opts: &Options) -> String;
}

