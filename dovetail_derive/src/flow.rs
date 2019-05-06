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
use flow::config::{Link, Task};
use petgraph::dot::Config as GraphConfig;
use petgraph::dot::Dot;
use petgraph::Graph;
use petgraph::graph::NodeIndex;
use petgraph::visit::Bfs;
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::iter::FromIterator;

use syn::{parse_quote, Error, Stmt};

use crate::environment;
use crate::internals::{Context, Generator, Module, Mapper};

pub fn expand_flow(
    _attr: TokenStream,
    _input: TokenStream,
) -> Result<proc_macro2::TokenStream, Vec<Error>> {
    let mut _tokens: Vec<proc_macro2::TokenStream> = Vec::new();

    let mut contxt = Context::new();

    println!(
        "Looking for app configuration at '{}'",
        environment::APP_CONFIG_PATH_KEY
    );

    let app_config_path_res = environment::get_app_config_path();

    let app_config_path = match app_config_path_res {
        Ok(app_config) => app_config,
        Err(e) => {
            let mut errors: Vec<syn::Error> = Vec::new();
            errors.push(e);
            return Err(errors);
        }
    };

    println!("App configuration found...");

    // Read app_config from path
    let app_config = read_app_config(&app_config_path);

    // Get only the flow resources
    let flow_resources: Vec<FlowConfig> = get_flow_resources(&app_config);

    // Create the graphs
    let flow_graphs = create_flow_graphs(&flow_resources);

    // Create the flows code logic
    let flows_code_ctxt = add_flows_code(flow_graphs);

    contxt.merge(&flows_code_ctxt);

    // Generate code
    Generator::gen(&contxt)
}

fn read_app_config(app_config_path: &str) -> AppConfig {
    // Load json file
    let file = match File::open(&app_config_path) {
        Err(why) => panic!("couldn't open {}: {:?}", &app_config_path, why),
        Ok(file) => file,
    };

    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `App`.
    let app_config: AppConfig = match serde_json::from_reader(reader) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("Error reading app config file: {:?}", why),
        Ok(app_config) => app_config,
    };

    app_config
}

fn get_flow_resources(app_config: &AppConfig) -> Vec<FlowConfig> {
    let mut flow_resources: Vec<FlowConfig> = Vec::new();
    for resource in &app_config.resources {
        println!("Resource id: {}", resource.id);
        let id_type_res = resource.get_type();
        // ignore not typed ids
        if id_type_res.is_ok() {
            let id_type = id_type_res.unwrap();
            if id_type.eq(environment::RESOURCE_FLOW_TYPE) {
                println!(
                    "Found resource flow type with id : {}",
                    resource.get_id().unwrap()
                );
                // Extract resource data
                let flow_data: FlowData = serde_json::from_value(resource.data.clone())
                    .expect("Error getting resource flow data");
                let flow_config: FlowConfig = FlowConfig::new(
                    resource.get_id().expect("Error getting id for resource"),
                    flow_data,
                );
                println!("Adding flow with id {}", &resource.id);
                flow_resources.push(flow_config);
            }
        }
    }
    flow_resources
}

fn create_flow_graphs<'a>(
    flow_configs: &'a Vec<FlowConfig>,
) -> Vec<(&'a FlowConfig, Option<NodeIndex>, Graph<&'a Task, &'a Link>)> {
    let mut flow_graphs: Vec<(&'a FlowConfig, Option<NodeIndex>, Graph<&'a Task, &'a Link>)> = Vec::new();
    for flow_config in flow_configs {
        flow_graphs.push(create_flow_graph(&flow_config));
    }
    flow_graphs
}

fn create_flow_graph<'a>(
    flow_config: &'a FlowConfig,
) -> (&'a FlowConfig, Option<NodeIndex>, Graph<&'a Task, &'a Link>) {
    let mut flow_graph = Graph::<&Task, &Link>::new();
    let mut taskid_to_nodeidx = HashMap::new();
    let mut first_task : Option<NodeIndex> = None;
    for task in &flow_config.data.tasks {
        let task_id = task.id.to_string().clone();
        let nodeidx = flow_graph.add_node(task);
        if first_task.is_none(){
            first_task = Some(nodeidx);
        }
        taskid_to_nodeidx.insert(task_id, nodeidx);
    }
    for link in &flow_config.data.links {
        let from_idx = taskid_to_nodeidx
            .get(&link.from)
            .expect(&format!("Issue getting taskId from link '{}'", &link.from));
        let to_idx = taskid_to_nodeidx
            .get(&link.to)
            .expect(&format!("Issue getting taskId to link '{}'", &link.to));
        flow_graph.add_edge(*from_idx, *to_idx, link);
    }
    /*println!("taskid_to_nodeidx: {:?}", taskid_to_nodeidx);
    println!(
        "{:?}",
        Dot::with_config(&flow_graph, &[GraphConfig::EdgeNoLabel])
    );*/
    (flow_config, first_task, flow_graph)
}

fn add_flows_code(graphs: Vec<(&FlowConfig, Option<NodeIndex>, Graph<&Task, &Link>)>) -> Context {
    let mut contxt = Context::new();

    for graph in graphs {
        let flow_code_ctxt = add_flow_code(graph);
        contxt.merge(&flow_code_ctxt);
    }

    contxt
}

fn add_flow_code(graph: (&FlowConfig, Option<NodeIndex>, Graph<&Task, &Link>)) -> Context {
    let module_name = get_module_name(&graph.0);
    println!("Module name: {}", module_name);

    let mut contxt = Context::new();

    // Add module
    let mod_contxt = add_flow_module(&module_name);
    contxt.merge(&mod_contxt);

    // Add the flow input struct
    let input_struct_contxt = add_flow_input_struct(&graph.0);
    contxt.merge(&input_struct_contxt);

    // Add the flow output struct
    let output_struct_contxt = add_flow_output_struct(&graph.0);
    contxt.merge(&output_struct_contxt);

    // Add the flow output struct
    let flow_start_fn_contxt = add_flow_start_fn(&module_name, graph);
    contxt.merge(&flow_start_fn_contxt);

    contxt
}

fn get_module_name<'a>(flow_config: &'a FlowConfig) -> &'a str {
    &flow_config.data.name
}

fn add_flow_module(module_name: &str) -> Context {
    let mut contxt = Context::new();

    let module_identi = Ident::new(&module_name, Span::call_site());

    let module = Module {
        is_pub: true,
        module_name: module_identi,
    };

    contxt.module = Some(module);

    contxt
}

// Adds the type that will be received as input when calling the flow
fn add_flow_input_struct(flow_config: &FlowConfig) -> Context {
    let mut contxt = Context::new();

    let input_attrs = create_flow_input_attrs(flow_config);
    let input_struct = parse_quote! {
            pub struct FlowInput {
                #input_attrs
            }
    };

    contxt.structs.insert(input_struct);

    contxt
}

fn create_flow_input_attrs(flow_config: &FlowConfig) -> proc_macro2::TokenStream {
    let mut attrs_tokens: Vec<proc_macro2::TokenStream> = Vec::new();
    // Get metadata
    let meta = &flow_config.data.metadata;
    // Iterate through input attrs
    for input_attr in &meta.input {
        let input_type =
            AllTypes::from_string(input_attr.name.to_owned(), input_attr.typ.to_owned());
        let input_type_name = Ident::new(&input_type.get_name().unwrap(), Span::call_site());
        let input_type_type = Ident::new(&input_type.get_type().unwrap(), Span::call_site());
        attrs_tokens.push(quote! {
                pub #input_type_name: #input_type_type,
        });
    }

    let input_attrs = proc_macro2::TokenStream::from_iter(attrs_tokens.into_iter());
    input_attrs
}

// Adds the type that will be received as output when calling the flow
fn add_flow_output_struct(flow_config: &FlowConfig) -> Context {
    let mut contxt = Context::new();

    let output_attrs = create_flow_output_attrs(flow_config);
    let output_struct = parse_quote! {
            pub struct FlowOutput {
                #output_attrs
            }
    };

    contxt.structs.insert(output_struct);

    contxt
}

fn create_flow_output_attrs(flow_config: &FlowConfig) -> proc_macro2::TokenStream {
    let mut attrs_tokens: Vec<proc_macro2::TokenStream> = Vec::new();
    // Get metadata
    let meta = &flow_config.data.metadata;
    // Iterate through output attrs
    for output_attr in &meta.output {
        let output_type =
            AllTypes::from_string(output_attr.name.to_owned(), output_attr.typ.to_owned());
        let output_type_name = Ident::new(&output_type.get_name().unwrap(), Span::call_site());
        let output_type_type = Ident::new(&output_type.get_type().unwrap(), Span::call_site());
        attrs_tokens.push(quote! {
                pub #output_type_name: #output_type_type,
        });
    }

    let output_attrs = proc_macro2::TokenStream::from_iter(attrs_tokens.into_iter());
    output_attrs
}

// Adds the type that will be received as output when calling the flow
fn add_flow_start_fn(module_name: &str, graph: (&FlowConfig, Option<NodeIndex>, Graph<&Task, &Link>)) -> Context {
    let mut contxt = Context::new();

    let (start_logic, start_logic_contxt) = create_flow_start_logic(module_name, graph);
    contxt.merge(&start_logic_contxt);
    let start_fn_name = format!("start_{}", module_name.to_string());
    let start_ident = Ident::new(&start_fn_name, Span::call_site());

    let start_fn = parse_quote! {
            #[no_mangle]
            pub fn #start_ident(flow_input: &FlowInput) -> Result<FlowOutput, String> {
                #start_logic
            }
    };

    contxt.fns.insert(start_fn);

    contxt
}

fn create_flow_start_logic(
    module_name: &str,
    graph: (&FlowConfig, Option<NodeIndex>, Graph<&Task, &Link>),
) -> (proc_macro2::TokenStream, Context) {
    let mut tasks_tokens: Vec<proc_macro2::TokenStream> = Vec::new();
    let mut contxt = Context::new();
    // Traverse through tasks
    let fist_task = match graph.1{
        Some(task) => task,
        None => {
            contxt.errors.push(Error::new(Span::call_site(), "No First task found"));
            return (proc_macro2::TokenStream::new(), contxt);
        }
    };

    let g = graph.2;
    let mut bfs = Bfs::new(&g, fist_task);
    while let Some(nx) = bfs.next(&g) {
        // Add tasks logic
        let (task_token, task_contxt) = create_flow_task_logic(g[nx]);
        tasks_tokens.push(task_token);
        contxt.merge(&task_contxt);
    }

    let logic = quote! {
        println!("Inside the logic");
        Ok(FlowOutput{test_flow_output: String::from("Logging the message Test Flow Input")})
    };
    tasks_tokens.push(logic);


    let flow_start_logic = proc_macro2::TokenStream::from_iter(tasks_tokens.into_iter());
    (flow_start_logic, contxt)
}

fn create_flow_task_logic(task: &Task) -> (proc_macro2::TokenStream, Context){
    let mut task_tokens: Vec<proc_macro2::TokenStream> = Vec::new();
    let mut contxt = Context::new();
    
    let activity_id = &task.id; 
    let activity_id_ident = Ident::new(&activity_id, Span::call_site());
    let start_fn_name = format!("start_{}", &activity_id);
    let start_fn_name_ident = Ident::new(&start_fn_name, Span::call_site());

    // Add activity uses
    // start_function use
    let start_fn_use = parse_quote!{
        use #activity_id_ident::#start_fn_name_ident;
    };
    contxt.uses.insert(start_fn_use);

    let activity_input_alias_type = format!("{}_ActivityInput", &activity_id);
    let activity_input_alias_type_ident = Ident::new(&activity_input_alias_type, Span::call_site());
    // ActivityInput use
    let activity_input_use = parse_quote!{
        use #activity_id_ident::ActivityInput as #activity_input_alias_type_ident;
    };
    contxt.uses.insert(activity_input_use);

    let mapper = Mapper::new(&task.activity.mappings);

    // Iterate through mapping inputs
    /*for mapping_input in &task.activity.mappings.input {
        println!("Mapping Input {:?}", mapping_input);
        // Prepare ActivityInput with mappings



        //let map_to
        /*let output_type =
            AllTypes::from_string(output_attr.name.to_owned(), output_attr.typ.to_owned());
        let output_type_name = Ident::new(&output_type.get_name().unwrap(), Span::call_site());
        let output_type_type = Ident::new(&output_type.get_type().unwrap(), Span::call_site());
        attrs_tokens.push(quote! {
                pub #output_type_name: #output_type_type,
        });*/
    }*/

    let activity_input_alias = format!("a_{}_ActivityInput", &activity_id);
    let activity_input_alias_ident = Ident::new(&activity_input_alias, Span::call_site());
    let input_mappings = &mapper.input_mappings();

    let activity_input_declaration = quote! {
        let #activity_input_alias_ident = #activity_input_alias_type_ident {
            #input_mappings
        };
    };

    task_tokens.push(activity_input_declaration);


    let flow_task_logic = proc_macro2::TokenStream::from_iter(task_tokens.into_iter());
    (flow_task_logic, contxt)
}
