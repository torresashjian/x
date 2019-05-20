// Developer Code
use dovetail_derive::trigger_settings;

#[trigger_settings()]
struct Settings {
  setting_foo: String,
  setting_bar: String,
}

impl Settings {
  fn from_app() -> Settings {
      Settings{..Default::default()}
  }
}

impl Default for Settings {
    fn default() -> Self { 
      Settings{
        setting_foo: "default setting name here!!".to_string(),
        setting_bar: Default::default(),
      }
    }
}

pub fn start_my_simple_trigger() {
    println!("Started trigger my_simple_trigger...");
    let settings = Settings::from_app();
    assert_eq!("default setting name here!!".to_string(), settings.setting_foo);
}
/*
// Generated Code
use my_simple_flow::Wasm::{start_Wasm, FlowInput};


// Developer Code
#[derive(DovetailTrigger)]
#[path = "src/WASM_fromDovetail.json"]
struct MyTrigger;


// Generated Code
extern crate data;

// Generated Code
use data::types::complex_object;
// Generated Code
use data::types::any;

// Generated Code
pub struct OutputBody {
    complex_object: complex_object,
}
// Generated Code
pub struct ReplyData {
    any: any,
}



// Generated Code ENPOINT OPTION 1
pub trait MyTriggerTrait {
    fn Wasm(outputBody: OutputBody) -> ReplyData;
}

// Generated Code
impl MyTriggerImpl for MyTriggerTrait {
    fn Wasm(outputBody: OutputBody) -> ReplyData {
      // TODO figure out implementation here
    }
}



// Developer code
#[derive(Debug)]
pub struct TriggerInput{
    pub name: String,
}

// Generated code
#[derive(Debug)]
pub struct TriggerReply {

}

#[trigger_settings()]
pub struct settings {
    my_settings_field: String,
}

// Generated Code ENPOINT OPTION 2
#[no_mangle]
//#[triggger()]
pub fn start_my_simple_trigger() -> Result<TriggerReply, String>  {
  let trigger_input: TriggerInput = TriggerInput{name: "My Name".to_string()};
  println!("Started trigger...");
  println!("Trigger Input : {:?}", trigger_input);
  // Construct FlowInput
  let flow_input = FlowInput{test_flow_input: trigger_input.name.to_owned()};
  // Start flow tasks
  //let _res = start_Wasm(flow_input);
  let res = start_Wasm(&flow_input);
  println!("Wasm trigger finished succesfully result: {:#?}...", res);
  Ok(TriggerReply{})
}

*/