// Copyright © 2019. TIBCO Software Inc.
// This file is subject to the license terms contained
// in the license file that is distributed with this file.
extern crate serde_json;

use dovetail_core::app::config::Config as AppConfig;
use dovetail_core::app::config::Trigger as TriggerAppConfig;
use dovetail_core::trigger::config::Config as TriggerConfig;
use proc_macro::TokenStream;
use quote::quote;
use std::fs::File;
use std::io::BufReader;
use std::iter::FromIterator;
use syn::{DeriveInput, Error};

use crate::app::get_app_config;
use crate::environment;
use crate::internals::DoveError;

pub fn expand_trigger_settings(
    _attr: TokenStream,
    input: DeriveInput,
) -> Result<proc_macro2::TokenStream, Vec<Error>> {
    let mut tokens: Vec<proc_macro2::TokenStream> = Vec::new();

    // Get trigger config
    let trigger_config_res = get_trigger_config();

    let trigger_config = match trigger_config_res {
        Ok(trigger_config) => trigger_config,
        Err(e) => {
            return Err(e.into());
        }
    };

    // Get app config
    let app_config_res = get_app_config();

    let app_config = match app_config_res {
        Ok(app_config) => app_config,
        Err(e) => {
            return Err(e.into());
        }
    };

    // Find trigger in app config
    let trigger_app_config_res = get_trigger_app_config(&trigger_config.reference, &app_config);

    let trigger_app_config = match trigger_app_config_res {
        Ok(trigger_app_config) => trigger_app_config,
        Err(e) => {
            return Err(e.into());
        }
    };
     
    println!("Found trigger app config: {:?}", &trigger_app_config);
    /*let fields_named_res = syn::parse_str::<FieldsNamed>("{my_field : String,}");
    println!("fields_named Result: {:?}", fields_named_res);

    let fields_named = match fields_named_res {
        Ok(fields_named) => {
            fields_named
        },
        Err(e) => {
            return Err(DoveError::from(format!("Error parsing trigger settings fields: {:?}", e)).into());
        }
    };

    // Validate that it is a struct
    let input_data_struct = match &input.data {
        Struct(data_struct) => {
            data_struct
        },
        _ => {
            return Err(DoveError::from("trigger_settings should be added to a struct".to_string()).into());
        }
    };

    let mut input_copy = input.clone();
    input_copy.data = syn::Data::Struct(syn::DataStruct{struct_token: input_data_struct.struct_token, fields: syn::Fields::Named(fields_named), semi_token: input_data_struct.semi_token});

    let input_modified = quote!{
        #input_copy
    };
    tokens.push(input_modified);

    /*println!(
        "Found trigger settings data '{:?}'",
        input.data
    );*/
    /*println!(
        "Looking for activity configuration at '{}'",
        &ACTIVITY_CONFIG_PATH
    );

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
            let mut errors: Vec<Error> = Vec::new();
            errors.push(Error::new(
                Span::call_site(),
                format!("couldn't generate activity input data : {:?}", why),
            ));
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
            let mut errors: Vec<Error> = Vec::new();
            errors.push(Error::new(
                Span::call_site(),
                format!("couldn't generate activity output data : {:?}", why),
            ));
            return Err(errors);
        }
    }
    */
    println!("Trigger output data generated succesfully...");*/

    let trigger_settings = quote! {
        #input
    };
    tokens.push(trigger_settings);

    let res = proc_macro2::TokenStream::from_iter(tokens.into_iter());
    println!("Final Trigger Code: {}", &res.to_string());
    Ok(res)
}

pub fn get_trigger_config() -> Result<TriggerConfig, DoveError> {
    let trigger_config_path_res = environment::get_trigger_config_path();

    let trigger_config_path = match trigger_config_path_res {
        Ok(trigger_config_path) => trigger_config_path,
        Err(e) => {
            return Err(e);
        }
    };

    // Load json file
    let file = match File::open(&trigger_config_path) {
        Ok(file) => file,
        Err(e) => {
            return Err(DoveError::from(format!(
                "couldn't open {}: {:?}",
                &trigger_config_path, e
            )));
        }
    };

    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `Config`.
    match serde_json::from_reader(reader) {
        Ok(trigger_config) => {
            println!("Trigger configuration found at {}", &trigger_config_path);
            return Ok(trigger_config);
        }
        Err(e) => {
            return Err(DoveError::from(format!(
                "Error reading trigger config file from {}: {:?}",
                &trigger_config_path, e
            )));
        }
    };
}

fn get_trigger_app_config<'a>(
    trigger_ref: &str,
    app_config: &'a AppConfig,
) -> Result<&'a TriggerAppConfig, DoveError> {
    if trigger_ref.is_empty() {
        return Err(DoveError::from(
            "Found empty trigger ref in trigger configuration".to_string(),
        ));
    }
    // Iterate through triggers in app_config
    for trigger in &app_config.triggers {
        if trigger_ref == trigger.reference {
            return Ok(trigger);
        }
    }
    Err(DoveError::from(format!(
        "No trigger with ref '{}' found in your app configuration file",
        trigger_ref
    )))
}

/*fn read_act_config(act_config_path: &str) -> Result<Config, Vec<Error>> {
    // Load json file
    let file = match File::open(&act_config_path) {
        Err(why) => {
            let mut errors: Vec<Error> = Vec::new();
            errors.push(Error::new(
                Span::call_site(),
                format!("couldn't open {}: {:?}", &act_config_path, why),
            ));
            return Err(errors);
        }
        Ok(file) => file,
    };

    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `Activity Config`.
    let act_config: Config = match serde_json::from_reader(reader) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("Error reading activity config file: {:?}", why),
        Ok(act_config) => act_config,
    };

    Ok(act_config)
}

fn generate_act_input(act_config: &Config) -> Result<proc_macro2::TokenStream, Vec<Error>> {
    println!("Generating activity input data...");
    let act_attrs = generate_act_input_attrs(&act_config);
    let default_act_attrs = generate_default_act_attrs(&act_config.input);
    let act_input = quote! {
        pub struct ActivityInput {
            #act_attrs
        }

        impl Default for ActivityInput {
            fn default() -> ActivityInput {
                ActivityInput {
                    #default_act_attrs
                }
            }
        }
    };
    Ok(act_input)
}

fn generate_act_input_attrs(act_config: &Config) -> proc_macro2::TokenStream {
    let mut attrs_tokens: Vec<proc_macro2::TokenStream> = Vec::new();
    // Iterate through input attrs
    for input_attr in &act_config.input {
        let input_type =
            AllTypes::from_string(input_attr.name.to_owned(), input_attr.typ.to_owned(), input_attr.value.to_owned());
        let input_type_name = Ident::new(&input_type.get_name().unwrap(), Span::call_site());
        let input_type_type = Ident::new(&input_type.get_type().unwrap(), Span::call_site());
        attrs_tokens.push(quote! {
                pub #input_type_name: #input_type_type,
        });
    }

    let input_attrs = proc_macro2::TokenStream::from_iter(attrs_tokens.into_iter());
    input_attrs
}

fn generate_act_output(act_config: &Config) -> Result<proc_macro2::TokenStream, Vec<Error>> {
    println!("Generating activity output data...");
    let act_attrs = generate_act_output_attrs(&act_config);
    let default_act_attrs = generate_default_act_attrs(&act_config.output);
    let act_output = quote! {
        pub struct ActivityOutput {
            #act_attrs
        }

        impl Default for ActivityOutput {
            fn default() -> ActivityOutput {
                ActivityOutput {
                    #default_act_attrs
                }
            }
        }
    };
    Ok(act_output)
}

fn generate_act_output_attrs(act_config: &Config) -> proc_macro2::TokenStream {
    let mut attrs_tokens: Vec<proc_macro2::TokenStream> = Vec::new();
    // Iterate through attrs
    for attr in &act_config.output {
        let typ = AllTypes::from_string(attr.name.to_owned(), attr.typ.to_owned(), attr.value.to_owned());
        let type_name = Ident::new(&typ.get_name().unwrap(), Span::call_site());
        let type_type = Ident::new(&typ.get_type().unwrap(), Span::call_site());
        attrs_tokens.push(quote! {
                pub #type_name: #type_type,
        });
    }

    let attrs = proc_macro2::TokenStream::from_iter(attrs_tokens.into_iter());
    attrs
}

fn generate_default_act_attrs(attrs: &Vec<DataType>) -> proc_macro2::TokenStream {
    let mut attrs_tokens: Vec<proc_macro2::TokenStream> = Vec::new();
    // Iterate through attrs
    for attr in attrs {
        let typ = AllTypes::from_string(attr.name.to_owned(), attr.typ.to_owned(), attr.value.to_owned());
        let type_name = Ident::new(&typ.get_name().unwrap(), Span::call_site());
        let default_value = &typ.get_value().unwrap();
        attrs_tokens.push(quote! {
                #type_name: #default_value,
        });
    }

    let attrs = proc_macro2::TokenStream::from_iter(attrs_tokens.into_iter());
    attrs
}
*/
