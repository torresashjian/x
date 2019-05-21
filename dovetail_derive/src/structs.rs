// Copyright © 2019. TIBCO Software Inc.
// This file is subject to the license terms contained
// in the license file that is distributed with this file.
use dovetail_core::DataType;
use std::iter::FromIterator;
use syn::{DeriveInput, Data, Fields, Field};

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
    let mut input_attrs_stream: Vec<proc_macro2::TokenStream> = Vec::new();
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

    let input_fields_named_named = &input_fields_named.named;
    for input_field_named in input_fields_named_named {
        println!("Input Field Named: {:?}", input_field_named);
    }

    let input_semi_token = &input_data_struct.semi_token;

    let my_expanded_struct = quote! {
            #input_attrs
            #input_vis #input_struct_token #input_ident #input_generics {

            }#input_semi_token
    };
    struct_tokens.push(my_expanded_struct);

    let expanded_struct = proc_macro2::TokenStream::from_iter(struct_tokens.into_iter());
    Ok(expanded_struct)
}

#[cfg(test)]
mod tests {
    use quote::quote;
    use syn::DeriveInput;
    use proc_macro::TokenStream;

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
        // Call expand
        let expand_struct_res = expand_struct(&my_test_struct, Vec::new());

        let expand_struct = match expand_struct_res {
            Ok(expand_struct) => expand_struct,
            Err(e) => panic!("Unable to expand struct, error: {:?}", e)
        };

        let my_expanded_struct: DeriveInput = parse_quote! {
            #expand_struct
        };

        // Make sure the attrs are the same
        assert_eq!(my_test_struct.attrs, my_expanded_struct.attrs, "Struct attributes should be the same");
        // Make sure the visibility is the same
        assert_eq!(my_test_struct.vis, my_expanded_struct.vis);
        // Make sure the identifier is the same
        assert_eq!(my_test_struct.ident, my_expanded_struct.ident);
        // Make sure the generics are the same
        assert_eq!(my_test_struct.generics, my_expanded_struct.generics);
    }
}
