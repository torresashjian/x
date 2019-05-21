// Generated Code
extern crate my_simple_trigger;

// Generated Code
use my_simple_trigger::start_my_simple_trigger;

fn main() {
    println!("Starting my_simple_trigger ...");
    start_my_simple_trigger();
}

#[cfg(test)]
mod tests {

    extern crate my_simple_trigger;
    use my_simple_trigger::{start_my_simple_trigger, Settings};

    #[test]
    fn test_my_simple_trigger() {
        start_my_simple_trigger();
    }

    #[test]
    fn test_my_simple_trigger_settings() {
        // Test default values
        let settings: Settings = Default::default();
        assert_eq!(
            "default setting name here!!".to_string(),
            settings.setting_foo
        );
        assert_eq!("".to_string(), settings.setting_bar);

        // Test from app values
        let settings_from_app: Settings = Settings::from_app();
        assert_eq!(
            "setting foo changed".to_string(),
            settings_from_app.setting_foo
        );
        assert_eq!(
            "setting bar also changed".to_string(),
            settings_from_app.setting_bar
        );
    }
}
