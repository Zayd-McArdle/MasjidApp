pub trait ValueRetriever {
    fn get_values() -> Vec<Self>
    where
        Self: Sized;
}
