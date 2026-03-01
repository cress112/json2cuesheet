use std::process::ExitCode;

use json2cuesheet::app::usecases::build_and_save_file;
use json2cuesheet::infra::app::CliParser;

fn main() -> ExitCode {
    let args_getter = CliParser::new();
    match build_and_save_file(args_getter) {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            println!("{}", e);
            return ExitCode::FAILURE;
        }
    }
}
