use json2cuesheet::app::usecases::build_and_save_file;
use json2cuesheet::infra::app::CliParser;

fn main() -> Result<(), String> {
    let args_getter = CliParser::new();
    build_and_save_file(args_getter)
}
