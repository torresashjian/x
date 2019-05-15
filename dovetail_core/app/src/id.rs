// Copyright © 2019. TIBCO Software Inc.
// This file is subject to the license terms contained
// in the license file that is distributed with this file.
pub struct CompositeId {
    pub raw: String,
    pub typ: String,
    pub id: String,
}

impl CompositeId {
    pub fn new(raw: String, typ: String, id: String) -> CompositeId {
        CompositeId {
            raw: raw,
            typ: typ,
            id: id,
        }
    }
}

pub trait IdParser {
    fn get_raw(&self) -> Result<String, String>;
    fn get_type(&self) -> Result<String, String>;
    fn get_id(&self) -> Result<String, String>;
}

pub fn parse_id(id: &String) -> Result<CompositeId, String> {
    let split = id.split(":");
    let vec: Vec<&str> = split.collect();
    let len = vec.len();
    match len {
        1 => {
            let split_id_part = vec.get(0);
            if None == split_id_part {
                return Err(format!("No Id part found for element '{}'", id));
            }
            // This case has no type
            return Ok(CompositeId::new(
                id.to_string(),
                "".to_string(),
                vec.get(0).unwrap().to_string(),
            ));
        }
        2 => {
            if None == vec.get(0) {
                return Err(format!("No type part found for element '{}'", id));
            }

            if None == vec.get(1) {
                return Err(format!("No Id part found for element '{}'", id));
            }
            // This case has type and id
            return Ok(CompositeId::new(
                id.to_string(),
                vec.get(0).unwrap().to_string(),
                vec.get(1).unwrap().to_string(),
            ));
        }
        _ => {
            // Unsupported id format
            return Err(format!(
                "Unsupported id format with {} reparation tokens",
                len
            ));
        }
    }
}
