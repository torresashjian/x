// Copyright © 2019. TIBCO Software Inc.
// This file is subject to the license terms contained
// in the license file that is distributed with this file.
extern crate serde;
extern crate serde_json;
extern crate proc_macro2;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

use syn::{Field, quote};

pub mod activity;
pub mod app;
pub mod flow;
pub mod trigger;

#[derive(Deserialize, Debug)]
pub struct DataType {
    pub name: String,
    #[serde(rename = "type")]
    pub typ: String,
    #[serde(default)]
    pub value: String,
}
 
impl From<DataType> for Field {
    fn from(inner: DataType) -> Field {
        let my_field = quote! {
            my_field: String,
        };
        let input = parse_macro_input!(my_field as Field);
        return input;
    }
}
