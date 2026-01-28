use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError, ValidationErrors};

#[derive(Debug, Deserialize, Serialize)]
pub struct PostalCode(String);
impl Validate for PostalCode {
    fn validate(&self) -> Result<(), ValidationErrors> {
        let mut errors = ValidationErrors::new();
        let postal_code = self.0.as_str().trim();
        if postal_code.is_empty() {
            errors.add(
                "postalCode",
                ValidationError::new("Postal code cannot be empty"),
            );
            return Err(errors);
        }
        if is_us_postal_code(postal_code) || is_uk_postal_code(postal_code)
        //TODO - add other postal codes
        {
            Ok(())
        } else {
            errors.add("postalCode", ValidationError::new("Invalid postal code"));
            Err(errors)
        }
    }
}
fn is_us_postal_code(postal_code: &str) -> bool {
    postal_code.chars().all(|c| c.is_ascii_digit())
        && (postal_code.len() == 5 || postal_code.len() == 10)
}

fn is_uk_postal_code(postal_code: &str) -> bool {
    if postal_code.len() >= 5 && postal_code.len() <= 8 {
        const UK_POSTAL_CODE_PATTERN: &'static str = r"^[A-Z]{1,2}[0-9][A-Z0-9]? ?[0-9][A-Z]{2}$";
        return regex::Regex::new(UK_POSTAL_CODE_PATTERN)
            .unwrap()
            .is_match(postal_code.to_uppercase().as_str());
    }
    false
}
