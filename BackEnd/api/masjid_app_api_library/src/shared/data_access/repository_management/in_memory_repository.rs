use crate::shared::data_access::repository_management::repository_type::RepositoryType;

pub struct InMemoryRepository {}

impl InMemoryRepository {
    pub async fn new(repository_type: RepositoryType) -> Self {
        InMemoryRepository {}
    }
}
