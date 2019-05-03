// Copyright © 2019. TIBCO Software Inc.
// This file is subject to the license terms contained
// in the license file that is distributed with this file.
use serde_json::value::Value;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub id:  String,
    #[serde(rename = "ref")]
    pub reference: String,
    #[serde(default)]
    pub settings: serde_json::map::Map<String, Value>,
    pub handlers: Vec<Handler>,
}

#[derive(Deserialize, Debug)]
pub struct Handler {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub settings: serde_json::map::Map<String, Value>,
    #[serde(default)]
    pub actions: Vec<Action>,
    pub action: Action,
}

#[derive(Deserialize, Debug)]
pub struct Action {
    #[serde(default)]
    pub id:  String,
    #[serde(rename = "ref")]
    pub reference: String,
    #[serde(default)]
    pub settings: serde_json::map::Map<String, Value>,
    #[serde(rename = "if")]
    #[serde(default)]
    pub iff:  String,
    #[serde(default)]
    pub input: serde_json::map::Map<String, Value>,
    #[serde(default)]
    pub output: serde_json::map::Map<String, Value>,
}