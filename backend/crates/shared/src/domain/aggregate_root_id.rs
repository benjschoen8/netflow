pub trait AggregateRootId: Send + Sync {
    fn to_string(&self) -> String;
}