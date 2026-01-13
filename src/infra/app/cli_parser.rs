use crate::app::interfaces::{Args, ArgsGetter};
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
struct CliArgs {
    input_file_path: PathBuf,
    target_file_path: PathBuf,
}

pub struct CliParser {
    parser: CliArgs,
}

impl CliParser {
    pub fn new() -> CliParser {
        CliParser {
            parser: CliArgs::parse(),
        }
    }
}

impl ArgsGetter for CliParser {
    fn load(&self) -> Result<Args, String> {
        Ok(Args {
            input_file_path: self.parser.input_file_path.clone(),
            target_file_path: self.parser.target_file_path.clone(),
        })
    }
}
