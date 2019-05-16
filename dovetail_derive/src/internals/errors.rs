// Copyright © 2019. TIBCO Software Inc.
// This file is subject to the license terms contained
// in the license file that is distributed with this file.
use syn::Error;
use proc_macro2::Span;

pub struct ErrorFactory {}

impl ErrorFactory {
    pub fn create(message: &str) -> Vec<Error> {
        return vec![Error::new(
            Span::call_site(),
            message,
        )];
    }
}
