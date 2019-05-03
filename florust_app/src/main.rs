extern crate dovetail_macro_derive;

use dovetail_macro_derive::JsonToDovetail;

#[derive(JsonToDovetail)]
#[path = "src/flogo.json"]
struct MyFlorustApp;

pub trait HelloMacro {
    fn hello_macro();
}

fn main() {
    MyFlorustApp::hello_macro();
}

#[cfg(test)]
mod tests {

    //use hello_macro::HelloMacro;
    use dovetail_macro_derive::JsonToDovetail;

    #[derive(JsonToDovetail)]
    #[path = "src/flogo.json"]
    struct MyFlorustApp;

    pub trait HelloMacro {
        fn hello_macro();
    }

    #[test]
    fn it_works() {
        MyFlorustApp::hello_macro();
        assert_eq!(2 + 2, 4);
    }
}
