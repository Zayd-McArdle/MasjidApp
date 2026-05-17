#[deprecated(note = "Please use `NormalDbProvider` or `InMemoryDbProvider` instead")]
#[derive(Hash, Eq, PartialEq, Clone)]
pub enum DbType {
    InMemory,
    MySql,
}
