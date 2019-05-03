// Copyright © 2019. TIBCO Software Inc.
// This file is subject to the license terms contained
// in the license file that is distributed with this file.
pub mod config;
pub mod id;
pub mod types;

#[macro_use]
extern crate serde_derive;

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

impl AllTypes {

}