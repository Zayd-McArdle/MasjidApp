use crate::shared::data_access::db_type::DbType;
use std::collections::HashMap;

#[derive(Default, Clone)]
pub struct AppState<T> {
    //This field allows you to write data to any database
    pub repository_map: HashMap<DbType, T>,
}
