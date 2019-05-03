// Copyright © 2019. TIBCO Software Inc.
// This file is subject to the license terms contained
// in the license file that is distributed with this file.
extern crate serde_json;

use app::types::AllTypes;
use proc_macro::TokenStream;
use std::iter::FromIterator;
use quote::quote;
use proc_macro2::{Ident, Span};
use serde_json::map::Map;
use serde_json::Value;
use std::env;
use std::fs::File;
use std::io::BufReader;
use activity::config::Config;


use syn::{DeriveInput, Error};

static ACTIVITY_CONFIG_PATH: &str = "activity.json";

pub fn expand_activity(
    attr: TokenStream,
    input: TokenStream,
) -> Result<proc_macro2::TokenStream, Vec<syn::Error>> {
    let mut tokens: Vec<proc_macro2::TokenStream> = Vec::new();

    println!("Looking for activity configuration at '{}'", &ACTIVITY_CONFIG_PATH);

    let act_config_res = read_act_config(&ACTIVITY_CONFIG_PATH);
    
    let act_config = match act_config_res {
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

    let start_fn : proc_macro2::TokenStream = proc_macro2::TokenStream::from(input);
    tokens.push(start_fn);
    let res = proc_macro2::TokenStream::from_iter(tokens.into_iter());
    println!("Final Activity Code: {}", &res.to_string());
    Ok(res)
}

fn read_act_config(act_config_path: &str) -> Result<Config, Vec<syn::Error>> {
    // Load json file
    let file = match  File::open(&act_config_path) {
        Err(why) => {
            let mut errors: Vec<syn::Error> = Vec::new();
            errors.push(syn::Error::new(Span::call_site(), format!("couldn't open {}: {:?}", &act_config_path, why)));
            return Err(errors);
        },
        Ok(file) => file,
    };

    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `Activity Config`.
    let act_config : Config = match serde_json::from_reader(reader) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("Error reading activity config file: {:?}",  why),
        Ok(act_config) => act_config,
    };

    Ok(act_config)
}

fn generate_act_input(act_config: &Config) -> Result<proc_macro2::TokenStream, Vec<syn::Error>>{
    println!("Generating activity input data...");
    let act_attrs = generate_act_input_attrs(&act_config);
    let act_input = quote!{
        pub struct ActivityInput {
            #act_attrs
        }
    };
    Ok(act_input)
}

fn generate_act_input_attrs(act_config: &Config) -> proc_macro2::TokenStream {
    let mut attrs_tokens: Vec<proc_macro2::TokenStream> = Vec::new();
    // Iterate through input attrs
    for input_attr in &act_config.input {
        let input_type = AllTypes::from_string(input_attr.name.to_owned(), input_attr.typ.to_owned());
        let input_type_name = Ident::new(&input_type.get_name().unwrap(), Span::call_site());
        let input_type_type = Ident::new(&input_type.get_type().unwrap(), Span::call_site());
        attrs_tokens.push(quote! {
                pub #input_type_name: #input_type_type,
        });
    }

    let res = proc_macro2::TokenStream::from_iter(attrs_tokens.into_iter());
    let input_attrs = quote! {
            #res
    };
    input_attrs
}