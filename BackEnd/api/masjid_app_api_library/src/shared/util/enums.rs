use crate::shared::traits::value_retriever::ValueRetriever;
use std::fmt::Display;

fn get_names<T>() -> Vec<String>
where
    T: ValueRetriever + ToString + Display,
{
    T::get_values().iter().map(T::to_string).collect()
}
