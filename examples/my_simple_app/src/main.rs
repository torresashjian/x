// Generated Code
extern crate my_simple_trigger;

// Generated Code
use my_simple_trigger::{start_my_simple_trigger};


fn main() {
    println!("Starting my_simple_trigger ...");
    start_my_simple_trigger();
}


#[cfg(test)]
mod tests {

    extern crate my_simple_trigger;
    use my_simple_trigger::{start_my_simple_trigger};

    #[test]
    fn test_my_simple_trigger() {
        println!("Starting test_my_simple_trigger ...");
        start_my_simple_trigger();
        /*let reply  = start_my_simple_trigger();
        println!("my_simple_trigger trigger finished succesfully reply: {:?}", reply);
        assert_eq!(reply.is_ok(), true);*/
    }
}