// Copyright © 2019. TIBCO Software Inc.
// This file is subject to the license terms contained
// in the license file that is distributed with this file.
extern crate proc_macro2;

use syn::Error;
use flow::config::Mappings;
use std::iter::FromIterator;

pub struct Mapper<'a> {
    pub mappings: &'a Mappings,
}

impl<'a> Mapper<'a> {
    pub fn new(mappings: &'a Mappings) -> Mapper {
        Mapper{mappings: &mappings}
    }

    pub fn input_mappings(&self) -> proc_macro2::TokenStream {
        let mut tokens: Vec<proc_macro2::TokenStream> = Vec::new();

        for input_mapping in &self.mappings.input {
            
        }
        
        let default_values = quote!{
            ..Default::default()
        };

        tokens.push(default_values);

        let res = proc_macro2::TokenStream::from_iter(tokens.into_iter());
        res
    }
}
