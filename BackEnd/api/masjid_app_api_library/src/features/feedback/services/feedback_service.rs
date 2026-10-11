use crate::shared::common_service_impl::CommonServiceImpl;

pub struct FeedbackServiceImpl<R, S>
where
    R: Send + Sync + ?Sized,
    S: Send + Sync + ?Sized,
{
    common: CommonServiceImpl<R>,
    image_: S,
}
