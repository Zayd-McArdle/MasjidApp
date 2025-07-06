use std::collections::HashMap;

#[derive(Default, Clone)]
pub struct AppState<T> {
    //This field allows you to write data to any database
    pub repository_map: HashMap<DbType, T>,
}
#[derive(Hash, Eq, PartialEq, Clone)]
pub enum DbType {
    InMemory,
    MySql,
}
