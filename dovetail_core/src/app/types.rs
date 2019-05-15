// Copyright © 2019. TIBCO Software Inc.
// This file is subject to the license terms contained
// in the license file that is distributed with this file.
use proc_macro2::{Ident, Span};
use quote::quote;

#[derive(Debug)]
pub enum AllTypes {
    TypeAny(String, String),
    TypeString(String, String),
    TypeInteger(String, String),
    TypeLong(String, String),
    TypeDouble(String, String),
    TypeBoolean(String, String),
    TypeObject(String, String),
    TypeComplexObject(String, String),
    TypeArray(String, String),
    TypeParams(String, String),
    TypeUnsupported,
}

impl AllTypes {
    // TODO add the rest of the types
    pub fn get_type(&self) -> Result<String, String> {
        match self {
            &AllTypes::TypeAny(_, _) => Err("Unsupported type".to_string()),
            &AllTypes::TypeString(_, _) => Ok(String::from("String")),
            &AllTypes::TypeInteger(_, _) => Ok(String::from("u32")),
            &AllTypes::TypeBoolean(_, _) => Ok(String::from("bool")),
            _ => Err("Unsupported type".to_string()),
        }
    }

    pub fn get_name(&self) -> Result<String, String> {
        match &self {
            &AllTypes::TypeAny(name, value) => Err("Unsupported type".to_string()),
            &AllTypes::TypeString(name, value) => Ok(name.to_string()),
            &AllTypes::TypeInteger(name, value) => Ok(name.to_string()),
            &AllTypes::TypeBoolean(name, value) => Ok(name.to_string()),
            _ => Err("Unsupported type".to_string()),
        }
    }

    pub fn get_value(&self) -> Result<proc_macro2::TokenStream, String> {
        match &self {
            &AllTypes::TypeAny(name, value) => Err("Unsupported type".to_string()),
            &AllTypes::TypeString(name, value) => {
							if value.is_empty(){
								let value_return = quote!{
									Default::default()
								};
								return Ok(value_return);
							} else {
								let value_return = quote!{
									#value.to_string()
								};
								return Ok(value_return);
							}
						},
            &AllTypes::TypeInteger(name, value) => {
							if value.is_empty(){
								let value_return = quote!{
									Default::default()
								};
								return Ok(value_return);
							} else {
								let value_integer = value.parse::<u32>().unwrap();
								let value_return = quote!{
									#value_integer
								};
								return Ok(value_return);
							}
						},
            &AllTypes::TypeBoolean(name, value) => {
							if value.is_empty(){
								let value_return = quote!{
									Default::default()
								};
								return Ok(value_return);
							} else {
								let value_bool = value.parse::<bool>().unwrap();
								let value_return = quote!{
									#value_bool
								};
								return Ok(value_return);
							}
						},
            _ => Err("Unsupported type".to_string()),
        }
    }

    pub fn from_string(name: String, type_name: String, value: String) -> AllTypes {
        match type_name.as_ref() {
            "string" => AllTypes::TypeString(name, value),
            "integer" => AllTypes::TypeInteger(name, value),
            "boolean" => AllTypes::TypeBoolean(name, value),
            _ => AllTypes::TypeUnsupported,
        }
    }
}
