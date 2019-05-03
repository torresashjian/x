// Developer Code
//extern crate dovetail_derive;
// Developer Code
use dovetail_derive::activity;

/*
// Developer Code
#[dovetail_activity(tp = "input")]
pub struct ActivityInput {
  pub message: String,
}

//#[dovetail_activity("output")]
pub struct ActivityOutput{
  pub message: String,
}
*/

// Developer Code
#[activity()]
pub fn start_my_simple_activity(activity_input: ActivityInput) -> Result<ActivityOutput, String> {
    println!("Inside my_simple_activity");
    let msg = format!("Logging the message {}", &activity_input.message);
    println!("{}", msg);
    Ok(ActivityOutput { message: msg })
}

#[cfg(test)]
mod tests {
    use crate::start_my_simple_activity;
    #[test]
    fn test_my_simple_activity() {
        // Preparing mock Activity Input
        let activity_input = ActivityInput {
            message: String::from("Test Activity Input"),
        };
        let res = start_my_simple_activity(activity_input);
        assert_eq!(
            "Logging the message Test Activity Input",
            res.unwrap().message
        );
    }
}
