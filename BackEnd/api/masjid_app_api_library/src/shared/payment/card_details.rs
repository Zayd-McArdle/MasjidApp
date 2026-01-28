use chrono::Datelike;
use serde::Deserialize;
use validator::{Validate, ValidationError, ValidationErrors};
#[derive(Debug, Deserialize, Validate)]
pub struct CardDetails {
    #[serde(rename = "cardholderName")]
    #[validate(length(min = 3, max = 100))]
    pub cardholder_name: String,

    #[serde(rename = "cardNumber")]
    #[validate(credit_card)]
    pub card_number: String,

    #[validate(length(min = 3, max = 4))]
    pub cvv: String,

    #[serde(rename = "expiryDate")]
    #[validate(nested)]
    pub expiry_date: ExpiryDate,
}

#[derive(Debug, Deserialize)]
pub struct ExpiryDate {
    pub month: u8,
    pub year: u8,
}
impl Validate for ExpiryDate {
    fn validate(&self) -> Result<(), ValidationErrors> {
        let mut errors = ValidationErrors::new();
        if self.month == 0 || self.month > 12 {
            errors.add("month", ValidationError::new("month is invalid"));
        }
        if self.year > 99 {
            errors.add("year", ValidationError::new("year is invalid"));
        }
        if !errors.is_empty() {
            return Err(errors);
        }
        let today = chrono::Local::now();
        // check if card expired
        if self.year < (today.year() % 100) as u8 && self.month < today.month() as u8 {
            errors.add("expiryDate", ValidationError::new("card expired"));
            return Err(errors);
        }
        Ok(())
    }
}
