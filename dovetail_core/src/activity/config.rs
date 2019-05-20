// Copyright © 2019. TIBCO Software Inc.
// This file is subject to the license terms contained
// in the license file that is distributed with this file.
extern crate serde;
extern crate serde_json;

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
    pub description: String,
    pub homepage: String,
    #[serde(default)]
    pub reply: bool,
    #[serde(default, rename = "return")]
    pub retrn: bool,
    pub input: Vec<DataType>,
    #[serde(default)]
    pub output: Vec<DataType>,
}
