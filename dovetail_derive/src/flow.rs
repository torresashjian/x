// Copyright © 2019. TIBCO Software Inc.
// This file is subject to the license terms contained
// in the license file that is distributed with this file.
extern crate serde_json;

/*
use app::types::AllTypes;
use std::iter::FromIterator;
use quote::quote;
use proc_macro2::{Ident, Span};
use std::fs::File;
use std::io::BufReader;
*/

use proc_macro::TokenStream;
use syn::{Error};

use crate::environment;

pub fn expand_flow(
    _attr: TokenStream,
    _input: TokenStream,
) -> Result<proc_macro2::TokenStream, Vec<Error>> {
    let mut _tokens: Vec<proc_macro2::TokenStream> = Vec::new();

    println!("Looking for app configuration at '{}'", environment::APP_CONFIG_PATH_KEY);

    let _act_config_res = environment::get_app_config_path();
    
    /*let act_config = match act_config_res {
        Ok(act_config) => act_config,
        Err(e) => {
            return Err(e);
        }
    };
     
    println!("Activity configuration found...");

    let act_input_res = generate_act_input(&act_config);

    match act_input_res {
        Ok(act_input) => {
            tokens.push(act_input);
        },
        Err(why) => {
            let mut errors: Vec<syn::Error> = Vec::new();
            errors.push(syn::Error::new(Span::call_site(), format!("couldn't generate activity input data : {:?}", why)));
            return Err(errors);
        }
    }

    println!("Activity input data generated succesfully...");

    let act_output_res = generate_act_output(&act_config);

    match act_output_res {
        Ok(act_output) => {
            tokens.push(act_output);
        },
        Err(why) => {
            let mut errors: Vec<syn::Error> = Vec::new();
            errors.push(syn::Error::new(Span::call_site(), format!("couldn't generate activity output data : {:?}", why)));
            return Err(errors);
        }
    }

    println!("Activity output data generated succesfully...");

    let start_fn : proc_macro2::TokenStream = proc_macro2::TokenStream::from(input);
    tokens.push(start_fn);
    let res = proc_macro2::TokenStream::from_iter(tokens.into_iter());
    println!("Final Activity Code: {}", &res.to_string());
    Ok(res)*/
    Ok(proc_macro2::TokenStream::new())
}