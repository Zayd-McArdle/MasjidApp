use crate::features::events::repositories::EventsRepository;
use crate::shared::common_service_impl::CommonServiceImpl;

pub struct EventServiceImpl<R: EventsRepository + ?Sized> {
    pub common: CommonServiceImpl<R>,
}
