
pub trait Engine {
    fn start(&self) -> Result<(), &str>;
    fn stop(&self) -> Result<(), &str>;
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
