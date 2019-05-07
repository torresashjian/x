// Copyright © 2019. TIBCO Software Inc.
// This file is subject to the license terms contained
// in the license file that is distributed with this file.
extern crate proc_macro2;

use syn::Error;
use flow::config::Mappings;
use std::iter::FromIterator;
use proc_macro2::{Ident, Span};

pub struct Mapper<'a> {
    pub mappings: &'a Mappings,
}

impl<'a> Mapper<'a> {
    pub fn new(mappings: &'a Mappings) -> Mapper {
        Mapper{mappings: &mappings}
    }

    pub fn input_mappings(&self) -> Result<proc_macro2::TokenStream, Error> {
        let mut tokens: Vec<proc_macro2::TokenStream> = Vec::new();

        for input_mapping in &self.mappings.input {
            // Check the type of mapping
            let mapping_code = match input_mapping.typ.as_str() {
                "assign" => {
                        // Get map to value
                        let map_to = &input_mapping.map_to;
                        if map_to.starts_with("$INPUT") {
                            let map_to_name = map_to.trim_start_matches("$INPUT['").trim_end_matches("']");
                            println!("Map To Name: {}", &map_to_name);
                            //return Ok(proc_macro2::TokenStream::new());
                        } else {
                            return Err(Error::new(Span::call_site(),format!("Unsupported mapping mapTo {}", &map_to)));
                        }
                        // Get map from value
                        let map_from = &input_mapping.value;
                        if map_from.starts_with("$flow") {
                            let map_from_name = map_from.trim_start_matches("$flow.");
                            println!("Map From Name: {}", &map_from_name);
                            //return Ok(proc_macro2::TokenStream::new());
                        } else {
                            return Err(Error::new(Span::call_site(),format!("Unsupported mapping value {}", &map_from)));
                        }
                },
                _ => {
                    return Err(Error::new(Span::call_site(),format!("Unsupported mapping type {}", &input_mapping.typ)));
                }
            };
        }
        
        let default_values = quote!{
            ..Default::default()
        };

        tokens.push(default_values);

        let res = proc_macro2::TokenStream::from_iter(tokens.into_iter());
        Ok(res)
    }
}
