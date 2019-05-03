
// Anything that is actionable
pub trait Actionable {
    fn run(&self) -> Result<String, String> ;
}
