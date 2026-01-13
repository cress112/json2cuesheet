use std::path::PathBuf;

pub trait ArgsGetter {
    fn load(&self) -> Result<Args, String>;
}

pub struct Args {
    pub input_file_path: PathBuf,
    pub target_file_path: PathBuf,
}
