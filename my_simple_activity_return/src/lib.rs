// Developer Code
use dovetail_derive::activity;


/*// Generated Code
pub struct ActivityInput {
  pub message: String,
  pub status: i64,
}

// Generated Code
pub struct ActivityOutput{
}
*/

// Developer Code
#[activity()]
pub fn start_my_simple_activity_return(activity_input: ActivityInput) -> Result<ActivityOutput, String>{
  println!("Inside start_my_simple_activity_return");
  let msg = format!("Logging the message {} and status {}", &activity_input.message, &activity_input.status);
  Ok(ActivityOutput{})
}

#[cfg(test)]
mod tests {
  use crate::*;
    #[test]
    fn test_my_simple_activity() {
      // Preparing mock Activity Input
      let activity_input = ActivityInput{message: String::from("Test Activity Input"), status: 200};
      let res = start_my_simple_activity_return(activity_input);
      assert_eq!(res.is_ok(), true);
    }
}