// Copyright © 2019. TIBCO Software Inc.
// This file is subject to the license terms contained
// in the license file that is distributed with this file.
use proc_macro2::{Ident, Span};
use std::collections::HashSet;
use syn::{Error, Stmt};

#[derive(Debug)]
pub struct Context {
    pub module: Option<Module>,
    pub crates: HashSet<Stmt>,
    pub uses: HashSet<Stmt>,
    pub structs: HashSet<Stmt>,
    pub fns: HashSet<Stmt>,
    pub errors: Vec<Error>,
}

#[derive(Debug, Clone)]
pub struct Module {
    pub is_pub: bool,
    pub module_name: Ident,
}

impl Context {
    pub fn new() -> Context {
        Context {
            module: None,
            crates: HashSet::new(),
            uses: HashSet::new(),
            structs: HashSet::new(),
            fns: HashSet::new(),
            errors: Vec::new(),
        }
    }

    pub fn merge(&mut self, new_context: &Context) -> Result<(), Error> {
        // Merge module
        if new_context.module.is_some() {
            if self.module.is_some() {
                // There is a problem
                return Err(Error::new(
                    Span::call_site(),
                    "Context can only have one module, trying to add another one...",
                ));
            }
            self.module = new_context.module.clone();
        }
        // Merge crates
        for a_crate in &new_context.crates {
            self.crates.insert(a_crate.clone());
        }
        // Merge uses
        for a_use in &new_context.uses {
            self.uses.insert(a_use.clone());
        }
        // Merge structs
        for a_struct in &new_context.structs {
            self.structs.insert(a_struct.clone());
        }
        // Merge fns
        for a_fn in &new_context.fns {
            self.fns.insert(a_fn.clone());
        }
        Ok(())
    }

    pub fn check(&self) -> Result<(), Vec<Error>> {
        if !self.errors.is_empty() {
            return Err(self.errors.clone());
        }
        Ok(())
    }
}
