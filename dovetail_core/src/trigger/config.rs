// Copyright © 2019. TIBCO Software Inc.
// This file is subject to the license terms contained
// in the license file that is distributed with this file.
use serde_json::value::Value;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub name: String,
    #[serde(rename = "type")]
    pub typ: String,
    #[serde(rename = "ref")]
    pub reference: String,
    pub version: String,
    #[serde(default)]
    pub tittle: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub homepage: String,
    #[serde(default)]
    pub settings: serde_json::map::Map<String, Value>,
    #[serde(default)]
    pub output: Vec<DataType>,
    #[serde(default)]
    pub reply: Vec<DataType>,
    pub handler: Vec<Handler>,
}

#[derive(Deserialize, Debug)]
pub struct DataType {
    pub name: String,
    #[serde(rename = "type")]
    pub typ: String,
    #[serde(default)]
    pub value: String,
}

#[derive(Deserialize, Debug)]
pub struct Handler {
    pub name: String,
    #[serde(rename = "type")]
    pub typ: String,
    pub required: bool,
    #[serde(default)]
    pub allowed: Vec<String>,
}
