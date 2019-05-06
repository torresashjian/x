// Copyright © 2019. TIBCO Software Inc.
// This file is subject to the license terms contained
// in the license file that is distributed with this file.

extern crate proc_macro2;

use crate::internals::Context;
use quote::quote;
use syn::Error;

pub struct Generator {}

impl Generator {
    pub fn gen(context: &Context) -> Result<proc_macro2::TokenStream, Vec<Error>> {
        // First check for errors
        match context.check() {
            Ok(()) => (),
            Err(e) => {
                return Err(e);
            }
        };
        // Generate crates
        let crates = &context.crates;

        let crates_gen = quote! {
            #(#crates)*
        };
        // Generate uses
        let uses = &context.uses;

        let uses_gen = quote! {
            #(#uses)*
        };
        // Generate structs
        let structs = &context.structs;

        let structs_gen = quote! {
            #(#structs)*
        };
        // Generate fns
        let fns = &context.fns;

        let fns_gen = quote! {
            #(#fns)*
        };

        // Add module if it exists
        let all_gen = match &context.module {
            Some(module) => {
                let module_name = &module.module_name;
                if module.is_pub {
                    println!("Public module found...");
                    let all = quote! {
                        pub mod #module_name {
                            #crates_gen
                            #uses_gen
                            #structs_gen
                            #fns_gen
                        }
                    };
                    all
                } else {
                    println!("Private module found...");
                    let all = quote! {
                        mod #module_name {
                            #crates_gen
                            #uses_gen
                            #structs_gen
                            #fns_gen
                        }
                    };
                    all
                }
            }
            None => {
                println!("No module found...");
                let all = quote! {
                    #crates_gen
                    #uses_gen
                    #structs_gen
                    #fns_gen
                };
                all
            }
        };

        println!("Generating Code: {}", &all_gen.to_string());

        Ok(all_gen)
    }
}
