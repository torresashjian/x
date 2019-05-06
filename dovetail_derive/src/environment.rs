// Copyright © 2019. TIBCO Software Inc.
// This file is subject to the license terms contained
// in the license file that is distributed with this file.

use proc_macro2::Span;
use std::env;
use syn::Error;

pub static APP_CONFIG_PATH_KEY: &str = "APP_CONFIG_PATH";
pub static RESOURCE_FLOW_TYPE: &str = "flow";

pub fn get_app_config_path() -> Result<String, Error> {
    // Get app config path from environment
    let path_res = env::var(APP_CONFIG_PATH_KEY);
    match path_res {
        Ok(config_path) => {
            return Ok(config_path);
        }
        Err(why) => {
            return Err(Error::new(
                Span::call_site(),
                format!(
                    "Error getting app config path from environment key {} : {:?}",
                    APP_CONFIG_PATH_KEY, why
                ),
            ));
        }
    }
}
