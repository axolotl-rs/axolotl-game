pub trait Nameable {
    fn get_name(&self) -> &str;

    fn set_name(&mut self, name: impl Into<String>);
}
