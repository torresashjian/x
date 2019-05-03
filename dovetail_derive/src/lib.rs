// Copyright © 2019. TIBCO Software Inc.
// This file is subject to the license terms contained
// in the license file that is distributed with this file.
extern crate proc_macro;
extern crate proc_macro2;

#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
/*use darling_core::FromMeta;
extern crate serde;
extern crate serde_json;
extern crate syn;
extern crate app;
extern crate dovetail_trigger_macro_derive;

use dovetail_trigger_macro_derive::DovetailTriggers;
use quote::quote;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use app::App;
use app::id::IdParser;
use proc_macro2::{Ident, Span};
*/

mod activity;

use syn::{parse_macro_input, DeriveInput, Error};

#[proc_macro_attribute]
pub fn activity(attr: TokenStream, input: TokenStream) -> TokenStream {
    //let attr = parse_macro_input!(attr as DeriveInput);
    //let input = parse_macro_input!(input as DeriveInput);
    activity::expand_activity(attr, input)
        .unwrap_or_else(to_compile_errors)
        .into()
    

    /*
    let attr_args = syn::parse_macro_input!(attr as syn::AttributeArgs);
    let mut tp : Option<String> = None;
    for attr_arg in attr_args {
        println!("attr_arg: {:?}", attr_arg);
        match attr_arg {
            NestedMeta::Meta(Meta::NameValue(MetaNameValue{ref ident, ref lit, ..})) if ident == "tp" => {
                if let Lit::Str(lit) = lit {
                    tp = Some(lit.value());
                }
            },
            _ => {}
        }
    }
    println!("tp: {}", tp.unwrap());
    */
    //let _input = parse_macro_input!(item as ItemFn);

    //input
}

fn to_compile_errors(errors: Vec<syn::Error>) -> proc_macro2::TokenStream {
    let compile_errors = errors.iter().map(syn::Error::to_compile_error);
    quote!(#(#compile_errors)*)
}

/*#[proc_macro_derive(JsonToDovetail, attributes(path))]
pub fn json_to_dovetail(input: TokenStream) -> TokenStream {

    // Construct a representation of Rust code as a syntax tree
    let ast : syn::DeriveInput = syn::parse(input).unwrap();

    let path = get_path(&ast);

    let path_str = path.unwrap();

    println!("Generating app with path: {}", &path_str);

    // Load json file
    let file = match  File::open(&path_str) {
        Err(why) => panic!("couldn't open {}: {}", &path_str, why.description()),
        Ok(file) => file,
    };

    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `App`.
    let app : App = match serde_json::from_reader(reader) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't serde_from_reader: {:?}",  why),
        Ok(app) => app,
    };

    // All tokens to generate
    let mut tokens : Vec<proc_macro2::TokenStream> = Vec::new();

    let name = &ast.ident;

    // Create triggers
    let triggers_token = quote! {
            #[derive(DovetailTriggers)]
            struct __DovetailTriggers;
        };
    tokens.push(triggers_token);


    // Iterate through resources
    for resource in &app.resources {
        // Load id
        let id = match resource.get_id() {
            Err(why) => panic!("Error getting resource id for resource '{:?}', error: {}", &resource, why),
            Ok(id) => id,
        };
        let identi = Ident::new(&id, Span::call_site());
        let token = quote! {

            impl HelloMacro for #name {
                fn hello_macro() {
                    println!("Hello, Macro! My name is {}", stringify!(#name));
                }
            }

            pub fn #identi() {
                println!("Inside function {}", stringify!(#id));
            }
        };
        tokens.push(token);
    };

    let res = proc_macro2::TokenStream::from_iter(tokens.into_iter());
    println!("{}", &res.to_string());
    res.into()
}

// get_path instrospects the metadata of the derive element and extracts the path out of it
fn get_path(ast : &syn::DeriveInput) -> Option<String> {
    let mut path : Option<String> = None;

    for option in ast.attrs.iter() {
        let option = option.parse_meta().unwrap();
        match option {
            // Match `#[ident = lit]` attributes.  Match guard makes it `#[path = lit]`
            Meta::NameValue(MetaNameValue{ref ident, ref lit, ..}) if ident == "path" => {
                if let Lit::Str(lit) = lit {
                    path = Some(lit.value());
                }
            },
            _ => {}
        }
    }

    path

}*/

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
