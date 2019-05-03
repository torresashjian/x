// Copyright © 2019. TIBCO Software Inc.
// This file is subject to the license terms contained
// in the license file that is distributed with this file.

#[derive(Debug)]
pub enum AllTypes {
    TypeAny(String),
	TypeString(String),
	TypeInteger(String),
	TypeLong(String),
	TypeDouble(String),
	TypeBoolean(String),
	TypeObject(String),
	TypeComplexObject(String),
	TypeArray(String),
	TypeParams(String),
    TypeUnsupported,
}


impl AllTypes {
	// TODO add the rest of the types
    pub fn get_type(&self) -> Result<String, String> {
		match self {
			&AllTypes::TypeAny(_) => Err("Unsupported type".to_string()),
			&AllTypes::TypeString(_) => Ok(String::from("String")),
			&AllTypes::TypeInteger(_) => Ok(String::from("u64")),
			&AllTypes::TypeBoolean(_) => Ok(String::from("bool")),
			_ => Err("Unsupported type".to_string()),
		}
	}

    pub fn get_name(&self) -> Result<String, String> {
		match &self {
			&AllTypes::TypeAny(name) => Err("Unsupported type".to_string()),
			&AllTypes::TypeString(name) => Ok(name.to_string()),
			&AllTypes::TypeInteger(name) => Ok(name.to_string()),
			&AllTypes::TypeBoolean(name) => Ok(name.to_string()),
			_ => Err("Unsupported type".to_string()),
		}
	}


	pub fn from_string(name: String, type_name: String) -> AllTypes {
		match type_name.as_ref() {
			"string" => AllTypes::TypeString(name),
			"integer" => AllTypes::TypeInteger(name),
			"boolean" => AllTypes::TypeBoolean(name),
			_ => AllTypes::TypeUnsupported,
		}
	}
}

