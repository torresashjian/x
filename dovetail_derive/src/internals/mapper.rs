// Copyright © 2019. TIBCO Software Inc.
// This file is subject to the license terms contained
// in the license file that is distributed with this file.
extern crate proc_macro2;

use syn::Error;

pub struct Mapper {}

impl Mapper {
    pub fn gen(context: &Context) -> Result<proc_macro2::TokenStream, Vec<Error>> {
    }
}
