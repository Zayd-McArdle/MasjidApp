use masjid_app_api_library::shared::services::payment::transaction_status::r#enum::TransactionStatus;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SendDonationResponse {
    #[serde(rename = "transactionStatus")]
    pub transaction_status: String,
}

impl From<TransactionStatus> for SendDonationResponse {
    fn from(value: TransactionStatus) -> Self {
        Self {
            transaction_status: value.to_string(),
        }
    }
}
