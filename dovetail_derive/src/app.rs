// Copyright © 2019. TIBCO Software Inc.
// This file is subject to the license terms contained
// in the license file that is distributed with this file.
use std::env;
use std::fs;
use std::path::Path;
use std::iter::FromIterator;
use syn::Error;
use proc_macro::TokenStream;
use proc_macro2::{Span};

use crate::environment;

static APP_CONFIG_PATH: &str = "app.json";

pub fn expand_app(
    _attr: TokenStream,
    input: TokenStream,
) -> Result<proc_macro2::TokenStream, Vec<Error>> {
    let mut tokens: Vec<proc_macro2::TokenStream> = Vec::new();
    let mut errs: Vec<Error> = Vec::new();

    /*println!(
        "Looking for app configuration at '{}'",
        &APP_CONFIG_PATH
    );

    let app_config_path = get_app_path(&APP_CONFIG_PATH);

    let app_config_path_string = match app_config_path {
        Ok(app_config_path_string) => app_config_path_string,
        Err(e) => {
            errs.push(e);
            return Err(errs);
        }
    };

    println!(
        "Found '{}' at path {}",
        &APP_CONFIG_PATH, &app_config_path_string
    );

    //Add app path to environment
    env::set_var(environment::APP_CONFIG_PATH_KEY, &app_config_path_string);

    println!(
        "Variable  '{}' set with value {}",
        environment::APP_CONFIG_PATH_KEY, &app_config_path_string
    );*/

    tokens.push(proc_macro2::TokenStream::from(input));
    let res = proc_macro2::TokenStream::from_iter(tokens.into_iter());
    println!("Final App Code: {}", &res.to_string());
    Ok(res)
}

/*fn get_app_path(app_config_path: &str) -> Result<String, Error> {
    let app_config_path = Path::new(&app_config_path);
    if !app_config_path.exists() {
        return Err(Error::new(
                Span::call_site(),
                format!("couldn't find {:?}", &app_config_path),
            ));
    }

    let absolute_path = fs::canonicalize(app_config_path);

    let os_app_config_path = match absolute_path {
        Ok(absolute_pathbuf) => {
            absolute_pathbuf.into_os_string()
        },
        Err(e) => {
            return Err(Error::new(
                Span::call_site(),
                format!("couldn't canonicalize path {:?}, err: {:?}", &app_config_path, e),
            ));
        }
    };


    match os_app_config_path.into_string() {
        Ok(app_config_path_string) => {
            return Ok(app_config_path_string); 
        },
        Err(e) => {
            return Err(Error::new(
                Span::call_site(),
                format!("couldn't convert path to osstring {:?}, err: {:?}", &app_config_path, e),
            ));
        }
    };
}*/