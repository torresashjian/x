// Copyright © 2019. TIBCO Software Inc.
// This file is subject to the license terms contained
// in the license file that is distributed with this file.

use crate::internals::DoveError;
use std::env;
use std::path::Path;

static APP_CONFIG_NAME: &str = "app.json";
static TRIGGER_CONFIG_NAME: &str = "trigger.json";

pub static RESOURCE_FLOW_TYPE: &str = "flow";

pub fn get_app_config_path() -> Result<String, DoveError> {
    // Get app config path from environment
    let pwd_path_res = env::var("PWD");
    let pwd_path = match pwd_path_res {
        Ok(pwd_path) => pwd_path,
        Err(e) => {
            return Err(DoveError::from(format!(
                "Error getting app config path from environment key PWD : {:?}",
                e
            )));
        }
    };

    let config_path_buf = Path::new(&pwd_path).join(APP_CONFIG_NAME);
    match config_path_buf.to_str() {
        Some(config_path) => Ok(config_path.to_owned()),
        None => {
            return Err(DoveError::from(format!(
                "Error getting app config path from location :{:?}",
                &config_path_buf
            )));
        }
    }
}

pub fn get_trigger_config_path() -> Result<String, DoveError> {
    // Get trigger config path from environment
    return Ok(TRIGGER_CONFIG_NAME.to_owned());
}
