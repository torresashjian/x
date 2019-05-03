// Generated Code
extern crate my_simple_trigger;

// Generated Code
use my_simple_trigger::Wasm;


#[cfg(test)]
mod tests {

    //extern crate my_simple_trigger;
    use my_simple_trigger::Wasm;
    //extern crate data;

    //use data::types::complex_object;

    #[test]
    fn test_my_simple_trigger() {
        // Call WASM flow
        println!("Calling wasm flow ...");
        //let output_body : complex_object = complex_object{};
        let reply  = Wasm("{\"name\":\"john\"}".to_string());
        println!("Wasm trigger finished succesfully reply: {}", reply);
        assert_eq!("Wasm trigger finished succesfully".to_string(), reply);
    }
}