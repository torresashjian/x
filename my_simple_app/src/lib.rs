// Generated Code
extern crate my_simple_trigger;

// Generated Code
use my_simple_trigger::{start_my_simple_trigger, TriggerInput};


#[cfg(test)]
mod tests {

    //extern crate my_simple_trigger;
    use my_simple_trigger::{start_my_simple_trigger, TriggerInput};
    //extern crate data;

    //use data::types::complex_object;

    #[test]
    fn test_my_simple_trigger() {
        // Call WASM flow
        println!("Calling wasm flow ...");
        //let output_body : complex_object = complex_object{};
        let trigger_input = TriggerInput{name : "john".to_string()};
        let reply  = start_my_simple_trigger(&trigger_input);
        println!("Wasm trigger finished succesfully reply: {:?}", reply);
        assert_eq!(reply.is_ok(), true);
    }
}