// Developer Code
use serde_json::value::Value;
use serde_json::map::Map;


// Developer Code
//#[dovetail_activity("input")]
pub struct ActivityInput {
  pub message: String,
  pub status: i64,
}

//#[dovetail_activity("output")]
pub struct ActivityOutput{
}

// Developer Code
//#[dovetail_activity("callback")]
pub fn start_my_simple_activity_return(activity_input: ActivityInput) -> Result<ActivityOutput, String>{
  println!("Inside start_my_simple_activity_return");
  let msg = format!("Logging the message {}", &activity_input.message);
  println!("{}", msg);
  Ok(ActivityOutput{message: msg})
}

#[cfg(test)]
mod tests {
  use crate::*;
    #[test]
    fn test_my_simple_activity() {
      // Preparing mock Activity Input
      let activity_input = ActivityInput{message: String::from("Test Activity Input")};
      let res = start_my_simple_activity(activity_input);
      assert_eq!("Logging the message Test Activity Input", res.unwrap().message);
    }
}