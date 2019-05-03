extern crate proc_macro;

extern crate serde_json;
extern crate serde;
extern crate syn;

use crate::proc_macro::TokenStream;
use quote::quote;
use std::error::Error;
use std::env;
use std::fs::File;
use std::io::BufReader;
use serde_json::Value;
use serde::{Deserialize, Serialize};

use syn::{
    Lit,
    Meta,
    MetaNameValue,
};

/*#[derive(Deserialize, Debug)]
struct App {
    name: String,
}*/

#[proc_macro_derive(HelloMacro, attributes(path))]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    
    // Construct a representation of Rust code as a syntax tree
    let ast : syn::DeriveInput = syn::parse(input).unwrap();

    let path = get_path(&ast);

    let path_str = path.unwrap();
    
    println!("Florust path attribute: {}", path_str);

    // Load json file
    let file = match  File::open(path_str) {
        Err(why) => panic!("couldn't open {}: {}", path_str, why.description()),
        Ok(file) => file,
    };

    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let app : serde_json::map::Map<String, Value> = match serde_json::from_reader(reader) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't serde_from_reader: {}",  why.description()),
        Ok(app) => app,
    };

    let app_string = &app["name"].as_str().unwrap();
    println!("Outside app str: {}", app_string);
    
    let name = &ast.ident;

    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {} app name: {}", stringify!(#name), stringify!(#app_string));
            }
        }
    };
    gen.into()

    // Build the trait implementation
    //impl_hello_macro(&ast, &app)
}

/*fn impl_hello_macro(ast: &syn::DeriveInput, _app: &App) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {} app name: {}", stringify!(#name), _app.name);
            }
        }
    };
    gen.into()
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
