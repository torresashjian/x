// Copyright © 2019. TIBCO Software Inc.
// This file is subject to the license terms contained
// in the license file that is distributed with this file.
extern crate proc_macro;
extern crate proc_macro2;
extern crate serde;
extern crate serde_json;
extern crate syn;
extern crate app;
extern crate trigger;

use std::collections::HashSet;
use proc_macro::TokenStream;
use std::fs::File;
use std::error::Error;
use std::io::BufReader;
use quote::quote;
use std::iter::FromIterator;
use proc_macro2::{Ident, Span};
use trigger::meta::{Metadata,TypedElement};

use syn::{
    Lit,
    Meta,
    MetaNameValue,
};

use crate::AllTypes::*;

#[derive(Debug)]
enum AllTypes {
    TypeAny(bool),
	TypeString(bool),
	TypeInteger(bool),
	TypeLong(bool),
	TypeDouble(bool),
	TypeBoolean(bool),
	TypeObject(bool),
	TypeComplexObject(bool),
	TypeArray(bool),
	TypeParams(bool),
    TypeUnsupported,
}


#[proc_macro_derive(DovetailTrigger, attributes(path))]
pub fn dovetail_trigger(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    let ast : syn::DeriveInput = syn::parse(input).unwrap();

    let path = get_path(&ast);

    let path_str = path.unwrap();
    
    println!("Generating app with path: {}", &path_str);

    
    /*let (t_crates, t_uses, mut t_code) = get_trigger_metadata();

    let mut tokens : Vec<proc_macro2::TokenStream> = Vec::new();

     // Create all crates import
    for t_crate in &t_crates {
        let identi = Ident::new(&t_crate, Span::call_site());
        println!("Creating crate: {}", identi);
        let token = quote! {
            extern crate #identi;
        };
        tokens.push(token);
    } 

     // Create all uses
    for t_use in &t_uses {
        println!("Creating use: {}", t_use);
        let mut identifiers : Vec<Ident> = Vec::new();
        let s_split : Vec<&str> = t_use.split("::").collect();
        for s in s_split {
            identifiers.push(Ident::new(s, Span::call_site()));
        }
        let token = quote! {
            use #(#identifiers)::*;
        };
        tokens.push(token);
    } 
    // All tokens to generate
    tokens.append(&mut t_code);
    let res = proc_macro2::TokenStream::from_iter(tokens.into_iter());
    println!("Tokens: {}", &res.to_string());

    res.into()*/
    TokenStream::new()
}

// Returns crates, uses and code needed for trigger metadata
fn get_trigger_metadata() -> (HashSet<String>, HashSet<String>, Vec<proc_macro2::TokenStream>){
    let trigger_path = String::from("trigger.json");
    
    println!("Generating trigger metadata for path: trigger.json");

    // Load json file
    let trigger_file = match  File::open(&trigger_path) {
        Err(why) => panic!("couldn't open {}: {}", &trigger_path, why.description()),
        Ok(trigger_file) => trigger_file,
    };

    let reader = BufReader::new(trigger_file);

    // Read the JSON contents of the file as an instance of `trigger::Metadata`.
    let meta : Metadata = match serde_json::from_reader(reader) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't serde_from_reader: {:?}",  why),
        Ok(meta) => meta,
    };

    // All tokens to generate
    let mut tokens : Vec<proc_macro2::TokenStream> = Vec::new();

    let mut t_crates = HashSet::new();

    t_crates.insert("data".to_string());

    let mut t_uses = HashSet::new();  

    // Create all settings types
    for setting in &meta.settings {
        println!("Creating setting type: {:?}", setting);
        // Getting data type
        let data_type = get_data_type(setting);
        println!("DataType: {:#?}", data_type);
        println!("Creating setting type: {:?}", setting);
        // Add crates
        t_uses.insert(format!("data::types::{}", setting.typ));

        // Add token code
        let s_name = format!("{}_{}", "settings", setting.name);
        let identi = Ident::new(&s_name, Span::call_site());
        let token = quote! {
            pub struct #identi {
            }
        };
        tokens.push(token);
    } 
    
    // Create all output types
    for out in &meta.output {
        println!("Creating output type: {:?}", out);
        // Add crates
        t_uses.insert(format!("data::types::{}", out.typ));

        // Add token code
        let o_name = format!("{}_{}", "output", out.name);
        let identi = Ident::new(&o_name, Span::call_site());
        let token = quote! {
            pub struct #identi {
            }
        };
        tokens.push(token);
    } 

    // Create all reply types
    for rep in &meta.reply {
        println!("Creating reply type: {:?}", rep);
        // Add crates
        t_uses.insert(format!("data::types::{}", rep.typ));

        // Add token code
        let m_name = format!("{}_{}", "reply", rep.name);
        let identi = Ident::new(&m_name, Span::call_site());
        let token = quote! {
            pub struct #identi {
            }
        };
        tokens.push(token);
    }    

    (t_crates, t_uses, tokens)
}

fn get_data_type(typed_element: &TypedElement) -> AllTypes {
    let typ = &typed_element.typ;
    // TODO: change this to correctly done
    let any = String::from("any");
    let string = String::from("string");
    let integer = String::from("integer");
    let long = String::from("long");
    let double = String::from("double");
    let boolean = String::from("boolean");
    let object = String::from("object");
    let complexObject = String::from("complexObject");
    let array = String::from("array");
    let params = String::from("params");
    match typ {
        any => TypeAny(false),
        string => TypeString(true),
        integer => TypeInteger(true),
        long => TypeLong(true),
        double => TypeUnsupported,
        boolean => TypeBoolean(true),
        object => TypeObject(false),
        complexObject => TypeComplexObject(false),
        array => TypeArray(false),
        params => TypeParams(false),
        _ => TypeUnsupported,
    }
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

    // Read the JSON contents of the file as an instance of `User`.
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
}*/

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

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
