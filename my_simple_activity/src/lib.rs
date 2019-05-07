// Developer Code
use dovetail_derive::activity;

/*
// Generated Code
pub struct ActivityInput {
  pub message: String,
}

// Generated Code
pub struct ActivityOutput{
  pub message: String,
}
*/

// Developer Code
#[activity()]
pub fn start_my_simple_activity(activity_input: &ActivityInput) -> Result<ActivityOutput, String> {
    println!("Inside my_simple_activity");
    let msg = format!("Logging the message {}", &activity_input.message);
    println!("{}", msg);
    Ok(ActivityOutput { message: msg })
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_my_simple_activity() {
        // Preparing mock Activity Input
        let activity_input = ActivityInput {
            message: String::from("Test Activity Input"),
            ..Default::default()
        };
        let res = start_my_simple_activity(&activity_input);
        assert_eq!(
            "Logging the message Test Activity Input",
            res.unwrap().message
        );
    }
}
