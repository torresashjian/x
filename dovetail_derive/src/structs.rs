// Copyright © 2019. TIBCO Software Inc.
// This file is subject to the license terms contained
// in the license file that is distributed with this file.
use dovetail_core::DataType;
use std::iter::FromIterator;
use syn::{DeriveInput, Data, Fields, Field, Token, FieldsNamed};
use syn::Data::Struct;
use proc_macro2::{Ident, Span};
use dovetail_core::app::types::AllTypes;

use crate::internals::DoveError;

pub fn expand_struct(
    input_struct: &DeriveInput, new_fields: Vec<DataType>
) -> Result<proc_macro2::TokenStream, DoveError> {
    let mut struct_tokens: Vec<proc_macro2::TokenStream> = Vec::new();
    // Iterate through input attrs
    /*for input_attr in &act_config.input {
        let input_type =
            AllTypes::from_string(input_attr.name.to_owned(), input_attr.typ.to_owned(), input_attr.value.to_owned());
        let input_type_name = Ident::new(&input_type.get_name().unwrap(), Span::call_site());
        let input_type_type = Ident::new(&input_type.get_type().unwrap(), Span::call_site());
        attrs_tokens.push(quote! {
                pub #input_type_name: #input_type_type,
        });
    }*/
    /*let mut input_attrs_stream: Vec<proc_macro2::TokenStream> = Vec::new();
    for attr in &input_struct.attrs {
        let input_attr = quote!{
            #attr
        };
        input_attrs_stream.push(input_attr);
    }
    let input_attrs = proc_macro2::TokenStream::from_iter(input_attrs_stream.into_iter());
    let input_vis = &input_struct.vis;
    let input_ident = &input_struct.ident;
    let input_generics = &input_struct.generics;
    let input_data_struct = match &input_struct.data{
        Data::Struct(data_struct) => data_struct,
        _ => {
            return Err(DoveError::from("trying to expand a non struct".to_string()));
        }
    };
    let input_struct_token = &input_data_struct.struct_token;
    let input_fields = &input_data_struct.fields;

    let input_fields_named = match &input_fields{
       Fields::Named(fields_named) => fields_named,
        _ => {
            return Err(DoveError::from("trying to expand a non struct fields".to_string()));
        }
    };


    let mut new_input_fields: Vec<proc_macro2::TokenStream> = Vec::new();

    let input_fields_named_named = &input_fields_named.named;
    for input_field_named in input_fields_named_named {
        let input_field_quote = quote! {
            #input_field_named
        };
        new_input_fields.push(input_field_quote);
    }

    // Add new fields
    for new_field in new_fields {
        let field_type =
            AllTypes::from_string(new_field.name.to_owned(), new_field.typ.to_owned(), new_field.value.to_owned());
        let new_field_name = Ident::new(&field_type.get_name().unwrap(), Span::call_site());
        let new_field_type_type = Ident::new(&field_type.get_type().unwrap(), Span::call_site());
        new_input_fields.push(quote! {
                pub #new_field_name: #new_field_type_type,
        });
    } 

    let new_input_fields = proc_macro2::TokenStream::from_iter(new_input_fields.into_iter());


    let input_semi_token = &input_data_struct.semi_token;

    let my_expanded_struct = quote! {
            #input_attrs
            #input_vis #input_struct_token #input_ident #input_generics {
                #new_input_fields
            }#input_semi_token
    };
    struct_tokens.push(my_expanded_struct);*/


    // Validate that it is a struct
    let input_data_struct = match &input_struct.data {
        Struct(data_struct) => {
            data_struct
        },
        _ => {
            return Err(DoveError::from("trigger_settings should be added to a struct".to_string()).into());
        }
    };

    let mut new_input_fields_vec: Vec<proc_macro2::TokenStream> = Vec::new();

        // Add new fields
    for new_field in new_fields {
        let field_type =
            AllTypes::from_string(new_field.name.to_owned(), new_field.typ.to_owned(), new_field.value.to_owned());
        let new_field_name = Ident::new(&field_type.get_name().unwrap(), Span::call_site());
        let new_field_type_type = Ident::new(&field_type.get_type().unwrap(), Span::call_site());
        new_input_fields_vec.push(quote! {
                pub #new_field_name: #new_field_type_type,
        });
    } 

    let new_input_fields = proc_macro2::TokenStream::from_iter(new_input_fields_vec.into_iter());

    // Add old fields
    let input_fields = &input_data_struct.fields;

    let input_fields_named = match &input_fields{
       Fields::Named(fields_named) => fields_named,
        _ => {
            return Err(DoveError::from("trying to expand a non struct fields".to_string()));
        }
    };


    let mut old_input_fields_vec: Vec<proc_macro2::TokenStream> = Vec::new();

    let input_fields_named_named = &input_fields_named.named;
    let input_fields_named_named_pairs = input_fields_named_named.pairs();
    for input_field_named_pair in input_fields_named_named_pairs {
        let input_field_quote = quote! {
            #input_field_named_pair
        };
        old_input_fields_vec.push(input_field_quote);
    }

    let old_input_fields = proc_macro2::TokenStream::from_iter(old_input_fields_vec.into_iter());

    println!("Old Input Fields {:?}", &old_input_fields.to_string());

    // Create the fields named
    let fields_named: FieldsNamed = parse_quote! {
        {
            #new_input_fields
            #old_input_fields
        }
    };

    let mut input_copy = input_struct.clone();
    input_copy.data = syn::Data::Struct(syn::DataStruct{struct_token: input_data_struct.struct_token, fields: syn::Fields::Named(fields_named), semi_token: input_data_struct.semi_token});

    let input_modified = quote!{
        #input_copy
    };
    struct_tokens.push(input_modified);

    let expanded_struct = proc_macro2::TokenStream::from_iter(struct_tokens.into_iter());
    Ok(expanded_struct)
}

#[cfg(test)]
mod tests {
    use quote::quote;
    use syn::DeriveInput;
    use proc_macro::TokenStream;
    use dovetail_core::DataType;

    use crate::structs::expand_struct;
    
    #[test]
    fn test_expand_struct() {
        let my_test_struct: DeriveInput = parse_quote! {
            #[derive(Debug)]
            pub struct MyTestStruct<T, U> {
                x: T,
                y: U,
            }
        };
        // Create a test data type
        let test_data_type = DataType{name: "my_test_name".to_string(), typ: "string".to_string(), value: "default_val".to_string()};
        // Call expand
        let expand_struct_res = expand_struct(&my_test_struct, vec![test_data_type]);

        let expand_struct = match expand_struct_res {
            Ok(expand_struct) => expand_struct,
            Err(e) => panic!("Unable to expand struct, error: {:?}", e)
        };

        println!("Final Struct {:?}", &expand_struct.to_string());

        /*let my_expanded_struct: DeriveInput = parse_quote! {
            #expand_struct
        };

        // Make sure the attrs are the same
        assert_eq!(my_test_struct.attrs, my_expanded_struct.attrs, "Struct attributes should be the same");
        // Make sure the visibility is the same
        assert_eq!(my_test_struct.vis, my_expanded_struct.vis);
        // Make sure the identifier is the same
        assert_eq!(my_test_struct.ident, my_expanded_struct.ident);
        // Make sure the generics are the same
        assert_eq!(my_test_struct.generics, my_expanded_struct.generics);*/
    }
}
