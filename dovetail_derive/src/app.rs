// Copyright © 2019. TIBCO Software Inc.
// This file is subject to the license terms contained
// in the license file that is distributed with this file.
use dovetail_core::app::config::Config;
use std::fs::File;
use std::io::BufReader;

use crate::environment;
use crate::internals::DoveError;

/*pub fn expand_app(
    _attr: TokenStream,
    input: TokenStream,
) -> Result<proc_macro2::TokenStream, Vec<Error>> {
    let mut tokens: Vec<proc_macro2::TokenStream> = Vec::new();
    let mut errs: Vec<Error> = Vec::new();

    tokens.push(proc_macro2::TokenStream::from(input));
    let res = proc_macro2::TokenStream::from_iter(tokens.into_iter());
    println!("Final App Code: {}", &res.to_string());
    Ok(res)
}*/

pub fn get_app_config() -> Result<Config, DoveError> {
    let app_config_path_res = environment::get_app_config_path();

    let app_config_path = match app_config_path_res {
        Ok(app_config_path) => app_config_path,
        Err(e) => {
            return Err(e);
        }
    };

    // Load json file
    let file = match File::open(&app_config_path) {
        Ok(file) => file,
        Err(e) => {
            return Err(DoveError::from(format!(
                "couldn't open {}: {:?}",
                &app_config_path, e
            )));
        }
    };

    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `Config`.
    match serde_json::from_reader(reader) {
        Ok(app_config) => {
            println!("App configuration found at {}", &app_config_path);
            return Ok(app_config);
        }
        Err(e) => {
            return Err(DoveError::from(format!(
                "Error reading app config file from {}: {:?}",
                &app_config_path, e
            )));
        }
    };
}
