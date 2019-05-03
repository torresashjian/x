// Copyright © 2019. TIBCO Software Inc.
// This file is subject to the license terms contained
// in the license file that is distributed with this file.
extern crate app;
extern crate proc_macro;
extern crate proc_macro2;
extern crate serde_json;

use app::config::Config as AppConfig;
use app::id::IdParser;
use app::types::AllTypes;
use flow::config::Config as FlowConfig;
use flow::config::Data as FlowData;
use flow::config::{Task, Link};
use petgraph::Graph;
use petgraph::dot::Config as GraphConfig;
use petgraph::dot::{Dot};
use proc_macro::TokenStream;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::BufReader;
use quote::quote;
use std::iter::FromIterator;
use proc_macro2::{Ident, Span};

// TODO: Should this go somewhere else??
static APP_CONFIG_PATH_KEY: &str = "APP_CONFIG_PATH";
static RESOURCE_FLOW_TYPE: &str = "flow";

#[proc_macro_attribute]
pub fn dovetail_flow (attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("Inside dovetail_flow"); 
    let app_conf_msg = format!("No {} env variable found, please set the path of your app configuration.", APP_CONFIG_PATH_KEY);
    // Get app config path from environment
    let app_config_path = env::var(APP_CONFIG_PATH_KEY).expect(&app_conf_msg);
    assert_eq!(app_config_path.is_empty(), false, "{}",app_conf_msg);
    println!("Looking for dovetail config path on '{}'", app_config_path);
    
    // Read app_config from path
    let app_config = read_app_config(&app_config_path);
    
    // Get only the flow resources
    let flow_resources: Vec<FlowConfig> = get_flow_resources(&app_config);

    // Create the graphs
    let flow_graphs = create_flow_graphs(&flow_resources);

    // Generate the final code
    generate_flows_code(flow_graphs)
}

fn read_app_config(app_config_path: &str) -> AppConfig {
    // Load json file
    let file = match  File::open(&app_config_path) {
        Err(why) => panic!("couldn't open {}: {:?}", &app_config_path, why),
        Ok(file) => file,
    };

    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `App`.
    let app_config : AppConfig = match serde_json::from_reader(reader) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("Error reading app config file: {:?}",  why),
        Ok(app_config) => app_config,
    };

    app_config
}

fn get_flow_resources(app_config: &AppConfig) -> Vec<FlowConfig>{
    let mut flow_resources : Vec<FlowConfig> = Vec::new();
    for resource in &app_config.resources {
        println!("Resource id: {}", resource.id);
        let id_type_res = resource.get_type();
        // ignore not typed ids
        if id_type_res.is_ok() {
            let id_type = id_type_res.unwrap();
            if id_type.eq(RESOURCE_FLOW_TYPE){
                println!("Found resource flow type with id : {}", resource.get_id().unwrap());
                // Extract resource data
                let flow_data : FlowData= serde_json::from_value(resource.data.clone()).expect("Error getting resource flow data");
                let flow_config : FlowConfig = FlowConfig::new(resource.get_id().expect("Error getting id for resource"), flow_data);
                println!("Adding flow with id {}", &resource.id);
                flow_resources.push(flow_config);
            }
        }
    }
    flow_resources
}

fn create_flow_graphs<'a>(flow_configs: &'a Vec<FlowConfig>) -> Vec<(&'a FlowConfig, Graph<&'a Task, &'a Link>)> {
    let mut flow_graphs: Vec<(&'a FlowConfig, Graph<&'a Task, &'a Link>)> = Vec::new();
    for flow_config in flow_configs {
        flow_graphs.push(create_flow_graph(&flow_config));
    }
    flow_graphs
}

fn create_flow_graph<'a>(flow_config: &'a FlowConfig) -> (&'a FlowConfig, Graph<&'a Task, &'a Link>){
    let mut flow_graph = Graph::<&Task, &Link>::new();
    let mut taskid_to_nodeidx = HashMap::new();
    for task in &flow_config.data.tasks {
        let task_id =  task.id.to_string().clone();
        let nodeidx = flow_graph.add_node(task);
        taskid_to_nodeidx.insert(task_id, nodeidx);
    }
    for link in &flow_config.data.links {
        let from_idx=  taskid_to_nodeidx.get(&link.from).expect(&format!("Issue getting taskId from link '{}'",&link.from));
        let to_idx =  taskid_to_nodeidx.get(&link.to).expect(&format!("Issue getting taskId to link '{}'",&link.to));
        flow_graph.add_edge(*from_idx, *to_idx, link);
    }
    println!("taskid_to_nodeidx: {:?}", taskid_to_nodeidx);
    println!("{:?}", Dot::with_config(&flow_graph, &[GraphConfig::EdgeNoLabel]));
    (flow_config, flow_graph)
}

fn generate_flows_code<'a>(graphs: Vec<(&'a FlowConfig, Graph<&'a Task, &'a Link>)>) -> TokenStream {
    let mut flows_tokens : Vec<proc_macro2::TokenStream> = Vec::new();
    for graph in graphs {
        let flow_code = generate_flow_code(graph);
        flows_tokens.push(flow_code);
    }
    let res = proc_macro2::TokenStream::from_iter(flows_tokens.into_iter());
    println!("Final Flows Code: {}", &res.to_string());
    res.into()
}

fn generate_flow_code<'a>(graph: (&'a FlowConfig, Graph<&'a Task, &'a Link>)) -> proc_macro2::TokenStream {
    let module_name = get_module_name(&graph.0);
    println!("Module name: {}", module_name);

    let mut tokens : Vec<proc_macro2::TokenStream> = Vec::new();

    // Generate the flow input struct
    tokens.push(generate_flow_input_struct(&graph.0));
    // Generate the flow output struct
    tokens.push(generate_flow_output_struct(&graph.0));
    // Generate the start flow fn
    tokens.push(generate_flow_start_fn(&module_name, graph));

    // Generate the flow module
    generate_flow_module(&module_name, tokens)
}

fn generate_flow_module(module_name: &str, tokens: Vec<proc_macro2::TokenStream>) -> proc_macro2::TokenStream{
    let module_identi = Ident::new(&module_name, Span::call_site());
    let res = proc_macro2::TokenStream::from_iter(tokens.into_iter());
    let module = quote! {
            pub mod #module_identi {
                #res
            }
    };
    module
}

fn get_module_name<'a> (flow_config: &'a FlowConfig) -> &'a str {
    &flow_config.data.name
}

// Generates the type that will be received as input when calling the flow
fn generate_flow_input_struct(flow_config: &FlowConfig) -> proc_macro2::TokenStream {
    let input_attrs = generate_flow_input_attrs(flow_config);
    let input_struct = quote! {
            pub struct FlowInput {
                #input_attrs
            }
    };
    input_struct
}

fn generate_flow_input_attrs(flow_config: &FlowConfig) -> proc_macro2::TokenStream {
    let mut attrs_tokens: Vec<proc_macro2::TokenStream> = Vec::new();
    // Get metadata
    let meta = &flow_config.data.metadata;
    // Iterate through input attrs
    for input_attr in &meta.input {
        let input_type = AllTypes::from_string(input_attr.name.to_owned(), input_attr.typ.to_owned());
        let input_type_name = Ident::new(&input_type.get_name().unwrap(), Span::call_site());
        let input_type_type = Ident::new(&input_type.get_type().unwrap(), Span::call_site());
        attrs_tokens.push(quote! {
                pub #input_type_name: #input_type_type,
        });
    }

    let res = proc_macro2::TokenStream::from_iter(attrs_tokens.into_iter());
    let input_attrs = quote! {
            #res
    };
    input_attrs
}

// Generates the type that will be returned as output when calling the flow
fn generate_flow_output_struct(flow_config: &FlowConfig) -> proc_macro2::TokenStream {
    let output_attrs = generate_flow_output_attrs(flow_config);
    let output_struct = quote! {
            pub struct FlowOutput {
                #output_attrs
            }
    };
    output_struct
}

fn generate_flow_output_attrs(flow_config: &FlowConfig) -> proc_macro2::TokenStream {
    let mut attrs_tokens: Vec<proc_macro2::TokenStream> = Vec::new();
    // Get metadata
    let meta = &flow_config.data.metadata;
    // Iterate through output attrs
    for output_attr in &meta.output {
        let output_type = AllTypes::from_string(output_attr.name.to_owned(), output_attr.typ.to_owned());
        let output_type_name = Ident::new(&output_type.get_name().unwrap(), Span::call_site());
        let output_type_type = Ident::new(&output_type.get_type().unwrap(), Span::call_site());
        attrs_tokens.push(quote! {
                pub #output_type_name: #output_type_type,
        });
    }

    let res = proc_macro2::TokenStream::from_iter(attrs_tokens.into_iter());
    let output_attrs = quote! {
            #res
    };
    output_attrs
}

// Generates the flow start function
fn generate_flow_start_fn<'a>(module_name: &str, graph: (&'a FlowConfig, Graph<&'a Task, &'a Link>)) -> proc_macro2::TokenStream {
    let start_logic = generate_flow_start_logic();
    let start_fn_name = format!("start_{}", module_name.to_string());
    let start_ident = Ident::new(&start_fn_name, Span::call_site());
    let start_fn = quote! {
            #[no_mangle]
            pub fn #start_ident(flow_input: FlowInput) -> Result<FlowOutput, String> {
                #start_logic
            }
    };
    start_fn
}

fn generate_flow_start_logic() -> proc_macro2::TokenStream {
    let logic = quote! {
        println!("Inside the logic");
        Err("Something".to_string()) 
    };
    logic
}