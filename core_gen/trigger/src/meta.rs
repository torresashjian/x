// Copyright © 2019. TIBCO Software Inc.
// This file is subject to the license terms contained
// in the license file that is distributed with this file.
#[derive(Deserialize, Debug)]
pub struct Metadata {
    pub name: String,
    #[serde(rename = "type")]
    pub typ: String, 
    #[serde(rename = "ref")]
    pub reference: String,
    pub version: String,
    pub title: String,
    pub description: String,
    #[serde(default)]
    pub settings: Vec<TypedElement>,
    #[serde(default)]
    pub output: Vec<TypedElement>,
    #[serde(default)]
    pub reply: Vec<TypedElement>,
    pub handler: Handler,
}

#[derive(Deserialize, Debug)]
pub struct TypedElement {
    pub name: String,
    #[serde(rename = "type")]
    pub typ: String,
    #[serde(default)]
    pub required: bool,
    #[serde(default)]
    pub allowed: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct Handler {
    #[serde(default)]
    pub settings: Vec<TypedElement>,
}