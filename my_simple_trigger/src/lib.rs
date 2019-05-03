
// Developer Code
extern crate dovetail_trigger_macro_derive;
// Developer Code
use dovetail_trigger_macro_derive::DovetailTrigger;

// Generated Code
use my_simple_flow::Wasm::{start_Wasm, FlowInput, FlowOutput};

/*
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
*/

// Generated Code ENPOINT OPTION 2
#[no_mangle]
pub fn Wasm(body: String) -> String {
  println!("Started trigger...");
  println!("Input param outputBody found: {}", body);
  // Construct FlowInput
  let flow_input = FlowInput{test_flow_input:body};
  // Start flow tasks
  //let _res = start_Wasm(flow_input);
  let _res = start_Wasm_manual(flow_input);
  "Wasm trigger finished succesfully".to_string()
}