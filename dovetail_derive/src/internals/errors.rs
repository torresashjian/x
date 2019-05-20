// Copyright © 2019. TIBCO Software Inc.
// This file is subject to the license terms contained
// in the license file that is distributed with this file.
use proc_macro2::Span;
use syn::Error;

pub enum DoveError {
    StringError(String),
    SynError(Error),
}

impl From<String> for DoveError {
    fn from(inner: String) -> DoveError {
        DoveError::StringError(inner)
    }
}

impl From<Error> for DoveError {
    fn from(inner: Error) -> DoveError {
        DoveError::SynError(inner)
    }
}

impl From<DoveError> for Vec<Error> {
    fn from(inner: DoveError) -> Vec<Error> {
        match inner {
            DoveError::StringError(message) => {
                return vec![Error::new(Span::call_site(), message)];
            }
            DoveError::SynError(e) => {
                return vec![e];
            }
        }
    }
}
