#[macro_export]
macro_rules! get_from_repositories_common {
    ($self:expr, $($get_call:tt)*) => {
        match $self.in_memory_repository.$($get_call)*.await {
            Ok(records) => Ok(records),
            Err(_) => Ok($self.repository.$($get_call)*.await?),
        }
    };
}
