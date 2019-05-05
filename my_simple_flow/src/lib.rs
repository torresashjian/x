// Generated Code
//extern crate dovetail_flow_macro_derive;
// Generated Code
use dovetail_derive::flow;

// Generated Code
// This will generate all flows
#[flow()]
fn start() {
}

//ALL Generated Code
pub mod Wasm2 {
    pub struct FlowInput {
        pub test_flow_input: String,
    }

    pub struct FlowOutput{
        pub test_flow_output: String,
    }

    //my_simple_activity
    use my_simple_activity::{start_my_simple_activity};
    use my_simple_activity::ActivityInput as my_simple_activity_ActivityInput;
    //use my_simple_activity::ActivityOutput as my_simple_activity_ActivityOutput;
    
    //my_simple_activity_return
    //use my_simple_activity_return::{start_my_simple_activity_return};
    use my_simple_activity_return::ActivityInput as my_simple_activity_return_ActivityInput;
    //use my_simple_activity_return::ActivityOutput as my_simple_activity_return_ActivityOutput;

    #[no_mangle]
    pub fn start_Wasm2(flow_input: &FlowInput) -> Result<FlowOutput, String> {

        println!("Inside start_Wasm flow next task: {}", "my_simple_activity");
        // Prepare the mappings
        let mut my_simple_activity_activityInput = my_simple_activity_ActivityInput{message: "".to_string()};
        // Assignment
        my_simple_activity_activityInput.message = flow_input.test_flow_input.to_owned();
        // Callback
        let my_simple_activity_result = start_my_simple_activity(&my_simple_activity_activityInput);
        let my_simple_activity_activityOutput = match my_simple_activity_result {
            Err(why) => {
                return Err(why);
            },
            Ok(flow_output) =>flow_output,
        };
        println!("start_my_simple_activity returned with result {}, flow next task: {}",my_simple_activity_activityOutput.message, "my_simple_activity_return");

        // Prepare the mappings

        /*
        let my_simple_activity_return_activityInput = my_simple_activity_return_ActivityInput{message: "".to_string(), status: 200};
        // Assignment
        my_simple_activity_activityInput.message = flow_input.test_flow_input;
        // Callback
        let my_simple_activity_return_result = start_my_simple_activity_return(&my_simple_activity_return_activityInput);
        let my_simple_activity_return_activityOutput = match my_simple_activity_return_result {
            Err(why) => {
                return Err(why);
            },
            Ok(flow_output) =>my_simple_activity_return_activityOutput,
        };
        println!("start_my_simple_activity returned with result {}, flow next task: {}",my_simple_activity_return_activityOutput.message, "my_simple_activity_return");
        */

        // Prepare metadata
        //let mut wasm_metadata : Map<String, Value> = Map::new();
        //wasm_metadata.insert("logLevel".to_string(), Value::String("INFO".to_string()));

        // Prepare mappings
        // This is a generated assign in mapper flow.body = input.message
        //let input_message = output_body;
        //start_my_simple_activity(input_message, wasm_metadata);
        println!("start_my_simple_activity returned flow next task: {}", "my_simple_activity");



        //start_my_simple_activity_return();
        println!("my_simple_activity_return returned flow finished returning: {}", "todo_add_returned_here");

        //Hello();
        //start();
        Ok(FlowOutput{test_flow_output: String::from("Logging the message Test Flow Input")})
    }


   /*#[cfg(test)]
    mod tests {
        use crate::Wasm::*;
        #[test]
        fn test_Wasm() {
            // Preparing mock Flow Input
            let flow_input = FlowInput{test_flow_input: String::from("Test Flow Input")};
            let res = start_Wasm(&flow_input);
            assert_eq!("Logging the message Test Flow Input", res.unwrap().test_flow_output);
        }
    }*/
}

#[cfg(test)]
mod tests {
    use crate::Wasm::*;
    #[test]
    fn test_Wasm() {
        // Preparing mock Flow Input
        let flow_input = FlowInput{test_flow_input: String::from("Test Flow Input")};
        let res = start_Wasm(&flow_input);
        assert_eq!("Logging the message Test Flow Input", res.unwrap().test_flow_output);
    }
}



