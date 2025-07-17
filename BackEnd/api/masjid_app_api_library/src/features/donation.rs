use chrono::{DateTime, Utc};

pub enum PostalCode{
    PostCode(String),
    ZipCode(String),
}
pub struct Address {
    pub line_1: String,
    pub line_2: Option<String>,
    pub postal_code: PostalCode,
}
pub enum DonationMethod {
    Now,
    Deferred
}
pub struct StandingOrder {
    account_holder: String,
    bank_name: String,
    account_number: String,
    sort_code: String,
    frequency: Option<String>,
    amount_per_donation: Option<u16>,
    start_date: Option<DateTime<Utc>>,
    end_date: Option<DateTime<Utc>>,
}
pub struct DonationDTO {
    cause: String,
    is_gift_aid: bool,
    title: String,
    first_name: String,
    last_name: String,
    address: Address,
    phone_number: String,
    amount: u16,
    standing_order: Option<StandingOrder>,
    date_donated: DateTime<Utc>,
}