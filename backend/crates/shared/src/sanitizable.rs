pub trait Sanitizable {
    fn safe_message(&self) -> String;
}