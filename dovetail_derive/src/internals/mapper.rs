// Copyright © 2019. TIBCO Software Inc.
// This file is subject to the license terms contained
// in the license file that is distributed with this file.
extern crate proc_macro2;

use dovetail_core::flow::config::Mappings;
use proc_macro2::{Ident, Span};
use std::iter::FromIterator;
use syn::Error;

pub struct Mapper<'a> {
    pub mappings: &'a Mappings,
}

impl<'a> Mapper<'a> {
    pub fn new(mappings: &'a Mappings) -> Mapper {
        Mapper {
            mappings: &mappings,
        }
    }

    pub fn input_mappings(&self) -> Result<proc_macro2::TokenStream, Error> {
        let mut tokens: Vec<proc_macro2::TokenStream> = Vec::new();

        for input_mapping in &self.mappings.input {
            // Check the type of mapping
            let mapping_code = match input_mapping.typ.as_str() {
                "assign" | "expression" => {
                    // Get map to value
                    let map_to = &input_mapping.map_to;
                    if map_to.starts_with("$INPUT") {
                        let map_to_name =
                            map_to.trim_start_matches("$INPUT['").trim_end_matches("']");
                        let map_to_name_ident = Ident::new(&map_to_name, Span::call_site());
                        let map_to_token = quote! {
                            #map_to_name_ident:
                        };

                        tokens.push(map_to_token);
                    } else {
                        return Err(Error::new(
                            Span::call_site(),
                            format!("Unsupported mapping mapTo {}", &map_to),
                        ));
                    }
                    // Get map from value
                    let map_from = &input_mapping.value;
                    if map_from.starts_with("$flow") {
                        let map_from_name = map_from.trim_start_matches("$flow.");
                        let map_from_name_ident = Ident::new(&map_from_name, Span::call_site());
                        let map_from_token = quote! {
                            flow_input.#map_from_name_ident.to_owned(),
                        };

                        tokens.push(map_from_token);
                    } else if map_from.starts_with("$activity") {
                        let map_from_trimmed = map_from.trim_start_matches("$activity[");
                        let mut activity_id = String::new();
                        let mut map_from_name = String::new();
                        let mut in_activity = true;
                        let mut in_map_from = false;
                        for c in map_from_trimmed.chars() {
                            if !in_activity && !in_map_from {
                                in_map_from = true;
                                continue;
                            }
                            if c == ']' {
                                in_activity = false;
                                continue;
                            }
                            if in_activity {
                                activity_id.push(c);
                            }
                            if in_map_from {
                                map_from_name.push(c);
                            }
                        }
                        println!(
                            "Map From Name activity: {} and map_from {}",
                            &activity_id, &map_from_name
                        );
                        // TODO we need access to activity alias and flows, so should we move this somewhere else????
                        // Adding default for now, but this is wrong!! SHOULD CHANGE
                        let map_from_token = quote! {
                            Default::default(),
                        };

                        tokens.push(map_from_token);
                    /*let map_from_name_ident = Ident::new(&map_from_name, Span::call_site());
                    let map_from_token = quote!{
                            flow_input.#map_from_name_ident.to_owned(),
                        };

                    tokens.push(map_from_token);*/
                    } else {
                        return Err(Error::new(
                            Span::call_site(),
                            format!("Unsupported mapping value {}", &map_from),
                        ));
                    }
                }
                _ => {
                    return Err(Error::new(
                        Span::call_site(),
                        format!("Unsupported mapping type {}", &input_mapping.typ),
                    ));
                }
            };
        }

        let default_values = quote! {
            ..Default::default()
        };

        tokens.push(default_values);

        let res = proc_macro2::TokenStream::from_iter(tokens.into_iter());
        Ok(res)
    }
}
