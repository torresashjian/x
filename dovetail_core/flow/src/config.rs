// Copyright © 2019. TIBCO Software Inc.
// This file is subject to the license terms contained
// in the license file that is distributed with this file.
use serde_json::value::Value;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub id: String,
    pub data: Data,
}
impl Config {
    pub fn new(id: String, data: Data) -> Config {
        Config { id: id, data: data }
    }
}
#[derive(Deserialize, Debug)]
pub struct Data {
    pub name: String,
    pub description: String,
    pub tasks: Vec<Task>,
    pub links: Vec<Link>,
    pub metadata: Metadata,
}

#[derive(Deserialize, Debug)]
pub struct Task {
    pub id: String,
    pub name: String,
    pub activity: Activity,
}

#[derive(Deserialize, Debug)]
pub struct Link {
    pub id: i8,
    pub from: String,
    pub to: String,
    #[serde(rename = "type")]
    pub typ: String,
    #[serde(default)]
    pub value: String,
}

// TODO finish this structure
#[derive(Deserialize, Debug)]
pub struct Metadata {
    #[serde(default)]
    pub input: Vec<DataType>,
    #[serde(default)]
    pub output: Vec<DataType>,
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
pub struct Activity {
    #[serde(rename = "ref")]
    pub reference: String,
    #[serde(default)]
    pub input: serde_json::map::Map<String, Value>,
    #[serde(default)]
    pub output: serde_json::map::Map<String, Value>,
    #[serde(default)]
    pub mappings: Mappings,
}

#[derive(Default, Deserialize, Debug)]
pub struct Mappings {
    #[serde(default)]
    pub input: Vec<Mapping>,
    #[serde(default)]
    pub output: Vec<Mapping>,
}

#[derive(Deserialize, Debug)]
pub struct Mapping {
    #[serde(rename = "mapTo")]
    pub map_to: String,
    #[serde(rename = "type")]
    pub typ: String,
    pub value: String,
}
