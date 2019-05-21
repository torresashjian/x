/*
 * Copyright © 2019. TIBCO Software Inc.
 * This file is subject to the license terms contained
 * in the license file that is distributed with this file.
 */
extern crate dovetail_derive;
extern crate cbor;
#[macro_use]
extern crate clap;
extern crate crypto;
#[macro_use]
extern crate log;
extern crate log4rs;
extern crate sawtooth_sdk;

mod handler;

use dovetail_derive::trigger_settings;
use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;

use std::process;

use sawtooth_sdk::processor::TransactionProcessor;

use handler::TriggerTransactionHandler;

//Generated code from app.json
pub const settings_validator_url: &str = "tcp://localhost:4004";
pub const settings_family_name: &str = "intkey";

#[trigger_settings()]
pub struct Settings{}


pub fn start_my_simple_sawtooth_trigger() {
    let settings = Settings::from_app();
    let matches = clap_app!(myapp =>
        (version: crate_version!())
        (about: "my_simple_sawtooth_trigger Transaction Processor (Rust)")
        (@arg connect: -C --connect +takes_value
         "connection endpoint for validator")
        (@arg verbose: -v --verbose +multiple
         "increase output verbosity"))
    .get_matches();

    let endpoint = matches
        .value_of("connect")
        .unwrap_or(settings.validator_url);

    let console_log_level;
    match matches.occurrences_of("verbose") {
        0 => console_log_level = LevelFilter::Warn,
        1 => console_log_level = LevelFilter::Info,
        2 => console_log_level = LevelFilter::Debug,
        3 | _ => console_log_level = LevelFilter::Trace,
    }

    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{h({l:5.5})} | {({M}:{L}):20.20} | {m}{n}",
        )))
        .build();

    let config = match Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(Root::builder().appender("stdout").build(console_log_level))
    {
        Ok(x) => x,
        Err(e) => {
            for err in e.errors().iter() {
                info!("Configuration error: {}", err.to_string());
            }
            process::exit(1);
        }
    };

    match log4rs::init_config(config) {
        Ok(_) => (),
        Err(e) => {
            info!("Configuration error: {}", e.to_string());
            process::exit(1);
        }
    }

    let handler = TriggerTransactionHandler::new(&settings);
    let mut processor = TransactionProcessor::new(endpoint);

    info!("Console logging level: {}", console_log_level);

    processor.add_handler(&handler);
    processor.start();
}
