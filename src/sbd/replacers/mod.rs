pub mod abbreviation;
pub mod list_item;
pub mod numbers;
pub trait Process {
    fn process(&self, text: String) -> String;
}
