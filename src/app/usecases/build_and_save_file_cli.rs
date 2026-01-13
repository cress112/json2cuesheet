use crate::app::interfaces::ArgsGetter;
use crate::domain::aggregates::common::json::Json as JsonCommon;
use crate::domain::aggregates::v1::json::Json as JsonV1;
use crate::domain::services::v1::Json2CueV1;
use std::fs;
use std::path::PathBuf;

pub fn build_and_save_file<A: ArgsGetter>(args_getter: A) -> Result<(), String> {
    // ! 書き出し実装をinfra-layerに掃き出し
    let args = args_getter.load()?;
    let version = parse_version(&args.input_file_path)?;
    if Json2CueV1::is_parsable_version(version) {
        let built_cue_string = build_json_v1(&args.input_file_path)?;
        return save_file(&args.target_file_path, built_cue_string);
    }
    Err(format!("unknown version: {}", version))
}

fn parse_version(input_file_path: &PathBuf) -> Result<u8, String> {
    let json_string = match fs::read_to_string(input_file_path) {
        Ok(res) => res,
        Err(res) => {
            return Err(format!(
                "failed to read json file: {:?}\n{}",
                input_file_path, res
            ));
        }
    };
    let json: JsonCommon = match serde_json::from_str(&json_string) {
        Ok(res) => res,
        Err(res) => {
            return Err(format!(
                "json file({:?}) is not parsable as v1\n{}",
                input_file_path, res
            ));
        }
    };
    Ok(json.version)
}

fn build_json_v1(input_file_path: &PathBuf) -> Result<String, String> {
    let json_string = match fs::read_to_string(input_file_path) {
        Ok(res) => res,
        Err(res) => {
            return Err(format!(
                "failed to read json file: {:?}\n{}",
                input_file_path, res
            ));
        }
    };
    let json: JsonV1 = match serde_json::from_str(&json_string) {
        Ok(res) => res,
        Err(res) => {
            return Err(format!(
                "json file({:?}) is not parsable as v1\n{}",
                input_file_path, res
            ));
        }
    };
    let builder = Json2CueV1::new(json);
    builder.build()
}

fn save_file(target_file_path: &PathBuf, data: String) -> Result<(), String> {
    match fs::write(target_file_path, data) {
        Ok(res) => Ok(res),
        Err(res) => Err(format!(
            "failed to save data to {:?}\n{}",
            target_file_path, res
        )),
    }
}
