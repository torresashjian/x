// Copyright © 2019. TIBCO Software Inc.
// This file is subject to the license terms contained
// in the license file that is distributed with this file.
use dovetail_core::app::types::AllTypes;
use dovetail_core::DataType;
use proc_macro2::{Ident, Span};
use serde_json::value::Value;
use std::iter::FromIterator;
use syn::Data::Struct;
use syn::{Data, DeriveInput, Field, Fields, FieldsNamed, Token};

use crate::internals::DoveError;

pub fn expand_fields(
    input_struct: &DeriveInput,
    new_fields: &Vec<DataType>,
) -> Result<proc_macro2::TokenStream, DoveError> {
    let mut struct_tokens: Vec<proc_macro2::TokenStream> = Vec::new();

    // Validate that it is a struct
    let input_data_struct = match &input_struct.data {
        Struct(data_struct) => data_struct,
        _ => {
            return Err(
                DoveError::from("expand_struct should be done to a struct".to_string()).into(),
            );
        }
    };

    let mut new_input_fields_vec: Vec<proc_macro2::TokenStream> = Vec::new();

    // Add new fields
    for new_field in new_fields {
        let field_type = AllTypes::from_string(
            new_field.name.to_owned(),
            new_field.typ.to_owned(),
            new_field.value.to_owned(),
        );
        let new_field_name = Ident::new(&field_type.get_name().unwrap(), Span::call_site());
        let new_field_type_type = Ident::new(&field_type.get_type().unwrap(), Span::call_site());
        new_input_fields_vec.push(quote! {
                pub #new_field_name: #new_field_type_type,
        });
    }

    let new_input_fields = proc_macro2::TokenStream::from_iter(new_input_fields_vec.into_iter());

    // Add old fields
    let input_fields = &input_data_struct.fields;

    let input_fields_named = match &input_fields {
        Fields::Named(fields_named) => fields_named,
        _ => {
            return Err(DoveError::from(
                "trying to expand a non struct fields".to_string(),
            ));
        }
    };

    let mut old_input_fields_vec: Vec<proc_macro2::TokenStream> = Vec::new();

    let input_fields_named_pairs = input_fields_named.named.pairs();
    for input_field_named in input_fields_named_pairs {
        let input_field_quote = quote! {
            #input_field_named
        };
        old_input_fields_vec.push(input_field_quote);
    }

    let old_input_fields = proc_macro2::TokenStream::from_iter(old_input_fields_vec.into_iter());

    // Create the fields named
    let fields_named: FieldsNamed = parse_quote! {
        {
            #new_input_fields
            #old_input_fields
        }
    };

    let mut input_copy = input_struct.clone();
    input_copy.data = syn::Data::Struct(syn::DataStruct {
        struct_token: input_data_struct.struct_token,
        fields: syn::Fields::Named(fields_named),
        semi_token: input_data_struct.semi_token,
    });

    let input_modified = quote! {
        #input_copy
    };
    struct_tokens.push(input_modified);

    let expanded_struct = proc_macro2::TokenStream::from_iter(struct_tokens.into_iter());
    Ok(expanded_struct)
}

pub fn expand_default(
    input_struct: &DeriveInput,
    default_fields: &Vec<DataType>,
) -> Result<proc_macro2::TokenStream, DoveError> {
    let mut struct_tokens: Vec<proc_macro2::TokenStream> = Vec::new();

    // Get Struct identifier
    let struct_ident = &input_struct.ident;

    let mut default_fields_vec: Vec<proc_macro2::TokenStream> = Vec::new();

    // Add default fields
    for default_field in default_fields {
        let field_type = AllTypes::from_string(
            default_field.name.to_owned(),
            default_field.typ.to_owned(),
            default_field.value.to_owned(),
        );
        let default_field_name = Ident::new(&field_type.get_name().unwrap(), Span::call_site());
        let default_field_value = &field_type.get_value().unwrap();
        default_fields_vec.push(quote! {
                #default_field_name: #default_field_value,
        });
    }

    let new_default_fields = proc_macro2::TokenStream::from_iter(default_fields_vec.into_iter());

    let struct_default = quote! {
        impl Default for #struct_ident {
            fn default() -> Self {
                #struct_ident {
                    #new_default_fields
                }
            }
        }
    };
    struct_tokens.push(struct_default);

    let expanded_struct = proc_macro2::TokenStream::from_iter(struct_tokens.into_iter());
    Ok(expanded_struct)
}

pub fn expand_from_app(
    input_struct: &DeriveInput,
    app_fields: &serde_json::map::Map<String, Value>,
    config_fields: &Vec<DataType>,
) -> Result<proc_macro2::TokenStream, DoveError> {
    let mut struct_tokens: Vec<proc_macro2::TokenStream> = Vec::new();

    // Get Struct identifier
    let struct_ident = &input_struct.ident;

    let mut from_app_fields_vec: Vec<proc_macro2::TokenStream> = Vec::new();

    // Iterate over config_fields.
    for config_field in config_fields {
        let field_type = AllTypes::from_string(
            config_field.name.to_owned(),
            config_field.typ.to_owned(),
            config_field.value.to_owned(),
        );
        // get if from app fields
        match app_fields.get(&field_type.get_name().unwrap()) {
            Some(app_field_value) => {
                // Override the value
                let overriden_field_type = AllTypes::from_string(
                    config_field.name.to_owned(),
                    config_field.typ.to_owned(),
                    app_field_value.as_str().unwrap().to_owned(),
                );
                let overriden_field_name =
                    Ident::new(&overriden_field_type.get_name().unwrap(), Span::call_site());
                let overriden_field_value = &overriden_field_type.get_value().unwrap();
                let from_app_field = quote! {
                    #overriden_field_name: #overriden_field_value,
                };
                from_app_fields_vec.push(from_app_field);
            }
            None => (),
        }
    }

    let from_app_fields = proc_macro2::TokenStream::from_iter(from_app_fields_vec.into_iter());

    let struct_default = quote! {
        impl #struct_ident {
            pub fn from_app() -> Self {
                #struct_ident {
                    #from_app_fields
                    ..Default::default()
                }
            }
        }
    };
    struct_tokens.push(struct_default);

    let expanded_struct = proc_macro2::TokenStream::from_iter(struct_tokens.into_iter());
    Ok(expanded_struct)
}

#[cfg(test)]
mod tests {
    use dovetail_core::DataType;
    use proc_macro::TokenStream;
    use quote::quote;
    use syn::Data::Struct;
    use syn::{Data, DeriveInput, Field, Fields, FieldsNamed, Token};

    use crate::structs::add_struct_fields;

    #[test]
    fn test_add_struct_fields() {
        let my_test_struct: DeriveInput = parse_quote! {
            #[derive(Debug)]
            pub struct MyTestStruct<T, U> {
                x: T,
                y: U,
            }
        };
        // Create a test data type
        let test_data_type = DataType {
            name: "my_test_name".to_string(),
            typ: "string".to_string(),
            value: "default_val".to_string(),
        };
        // Call expand
        let expand_struct_res = expand_fields(&my_test_struct, vec![test_data_type]);

        let expand_struct = match expand_struct_res {
            Ok(expand_struct) => expand_struct,
            Err(e) => panic!("Unable to expand struct, error: {:?}", e),
        };

        let my_expanded_struct: DeriveInput = parse_quote! {
            #expand_struct
        };

        // Make sure the attrs are the same
        assert_eq!(
            my_test_struct.attrs, my_expanded_struct.attrs,
            "Struct attributes should be the same"
        );
        // Make sure the visibility is the same
        assert_eq!(
            my_test_struct.vis, my_expanded_struct.vis,
            "Struct visibility should be the same"
        );
        // Make sure the identifier is the same
        assert_eq!(
            my_test_struct.ident, my_expanded_struct.ident,
            "Struct identifier should be the same"
        );
        // Make sure the generics are the same
        assert_eq!(
            my_test_struct.generics, my_expanded_struct.generics,
            "Struct generics should be the same"
        );

        // Get the data_strcut
        let input_data_struct = match &my_expanded_struct.data {
            Struct(data_struct) => data_struct,
            _ => {
                panic!("trigger_settings should be added to a struct".to_string());
            }
        };

        let input_fields = &input_data_struct.fields;

        let input_fields_named = match &input_fields {
            Fields::Named(fields_named) => fields_named,
            _ => {
                panic!("trying to expand a non struct fields".to_string());
            }
        };

        // Make sure the data fields number are correct
        assert_eq!(
            &3usize,
            &input_fields_named.named.pairs().count(),
            "Struct data fields number should be correct"
        );
    }
}
