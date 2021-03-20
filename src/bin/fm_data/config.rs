use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fs::read_to_string;

#[derive(Serialize, Deserialize, Debug)]
pub struct GoogleConfiguration {
    pub creds_file : String,
    pub token_file : String,
    pub spreadsheet_name : String,
    pub team_sheet : String,
    pub team_perf_sheet : String,
    pub league_perf_sheet : String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InputConfiguration {
    pub data_html : String,
    pub league_perf_html : String,
    pub team_perf_html : String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Configuration {
    pub google : GoogleConfiguration,
    pub input : InputConfiguration
}

// XXX: make config json path configurable
pub fn read_configuration() -> Result<Configuration> {
    let config_file="config.json";
    let data = read_to_string(config_file).unwrap();
    serde_json::from_str(data.as_str())
}