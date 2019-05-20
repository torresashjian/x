// Copyright © 2019. TIBCO Software Inc.
// This file is subject to the license terms contained
// in the license file that is distributed with this file.

use crate::DataType;

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
    pub settings: Vec<DataType>,
    #[serde(default)]
    pub output: Vec<DataType>,
    #[serde(default)]
    pub reply: Vec<DataType>,
    pub handler: Handler,
}

#[derive(Deserialize, Debug)]
pub struct Handler {
    #[serde(default)]
    pub settings: Vec<DataType>,
}
