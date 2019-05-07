// Copyright © 2019. TIBCO Software Inc.
// This file is subject to the license terms contained
// in the license file that is distributed with this file.
extern crate serde;
extern crate serde_json;

use serde_json::value::Value;
use trigger;

use crate::id::{parse_id, IdParser};

#[derive(Deserialize, Debug)]
pub struct Config {
    pub name: String,
    #[serde(rename = "type")]
    pub typ: String,
    pub version: String,
    pub description: String,
    pub triggers: Vec<trigger::config::Config>,
    pub resources: Vec<Resource>,
}

#[derive(Deserialize, Debug)]
pub struct Resource {
    pub id: String,
    pub data: Value,
}

impl IdParser for Resource {
    fn get_raw(&self) -> Result<String, String> {
        return Ok(self.id.to_owned());
    }

    fn get_type(&self) -> Result<String, String> {
        let parsed_res = parse_id(&self.id);
        match parsed_res {
            Ok(composite_id) => Ok(composite_id.typ),
            Err(err_msg) => Err(err_msg),
        }
    }

    fn get_id(&self) -> Result<String, String> {
        let parsed_res = parse_id(&self.id);
        match parsed_res {
            Ok(composite_id) => Ok(composite_id.id),
            Err(err_msg) => Err(err_msg),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
