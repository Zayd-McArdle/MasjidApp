use crate::features::donation::errors::donation_history_admin_repository_errors::GetDonationTransactionsError;
use crate::features::donation::errors::grouped_donation_history_error::GroupedDonationHistoryError;
use crate::features::donation::models::donation_dto::DonationHistoryDTO;
use crate::features::donation::models::donation_filter::DonationFilter;
use crate::features::donation::models::donation_grouping::DonationGrouping;
use async_trait::async_trait;
use masjid_app_api_library::features::donation::models::donation_history::DonationHistory;
use masjid_app_api_library::new_repository;
use masjid_app_api_library::shared::data_access::repository_manager::{
    InMemoryRepository, MySqlRepository, RepositoryMode, RepositoryType,
};
use mockall::automock;
use sqlx::mysql::{MySqlArguments, MySqlRow};
use sqlx::query::Query;
use sqlx::{MySql, MySqlPool, Row};
use std::collections::HashMap;
use std::sync::Arc;

#[automock]
#[async_trait]
pub trait DonationHistoryAdminRepository: Send + Sync {
    async fn get_donation_transactions(
        &self,
        donation_filter: &DonationFilter,
    ) -> Result<Vec<DonationHistoryDTO>, GetDonationTransactionsError>;
    async fn get_grouped_donation_transaction_history(
        &self,
        donation_filter: &DonationFilter,
        donation_grouping: &DonationGrouping,
    ) -> Result<HashMap<String, Vec<DonationHistoryDTO>>, GroupedDonationHistoryError>;
    async fn get_grouped_donation_transaction_history_count(
        &self,
        donation_filter: &DonationFilter,
        donation_grouping: &DonationGrouping,
    ) -> Result<HashMap<String, u32>, GroupedDonationHistoryError>;
}
#[async_trait]
impl DonationHistoryAdminRepository for InMemoryRepository {
    async fn get_donation_transactions(
        &self,
        filters: &DonationFilter,
    ) -> Result<Vec<DonationHistoryDTO>, GetDonationTransactionsError> {
        todo!()
    }

    async fn get_grouped_donation_transaction_history(
        &self,
        donation_filter: &DonationFilter,
        donation_grouping: &DonationGrouping,
    ) -> Result<HashMap<String, Vec<DonationHistoryDTO>>, GroupedDonationHistoryError> {
        todo!()
    }

    async fn get_grouped_donation_transaction_history_count(
        &self,
        donation_filter: &DonationFilter,
        donation_grouping: &DonationGrouping,
    ) -> Result<HashMap<String, u32>, GroupedDonationHistoryError> {
        todo!()
    }
}

#[async_trait]
impl DonationHistoryAdminRepository for MySqlRepository {
    async fn get_donation_transactions(
        &self,
        filters: &DonationFilter,
    ) -> Result<Vec<DonationHistoryDTO>, GetDonationTransactionsError> {
        let donation_intention = filters
            .donation_intention
            .and_then(|intention| Some(intention.to_string()));
        let donation_frequency = filters
            .donation_frequency
            .and_then(|frequency| Some(frequency.to_string()));
        let transaction_status = filters
            .transaction_status
            .and_then(|status| Some(status.to_string()));
        const STORED_PROCEDURE: &'static str =
            "CALL get_donation_transactions(?, ?, ?, ?, ?, ?, ?, ?)";
        let db_connection = self.db_connection.clone();
        let donation_transactions = sqlx::query(STORED_PROCEDURE)
            .bind(&filters.donation_cause)
            .bind(&donation_intention)
            .bind(&filters.email)
            .bind(&filters.phone_number)
            .bind(&filters.amount)
            .bind(&filters.is_gift_aid)
            .bind(&donation_frequency)
            .bind(&transaction_status)
            .map(donation_history_from_my_sql_row)
            .fetch_all(&*db_connection)
            .await
            .map_err(|err| {
                if let sqlx::Error::RowNotFound = err {
                    return GetDonationTransactionsError::NotFound;
                }
                tracing::error!(
                    donation_cause = &filters.donation_cause,
                    donation_intention = donation_intention,
                    email = &filters.email,
                    phone_number = &filters.phone_number,
                    amount = &filters.amount,
                    is_gift_aid = &filters.is_gift_aid,
                    donation_frequency = &donation_frequency,
                    transaction_status = &transaction_status,
                    stored_procedure = STORED_PROCEDURE,
                    error = err.to_string(),
                    "unable to fetch donation transaction history from database"
                );
                GetDonationTransactionsError::UnableToFetchDonationTransactions
            })?;
        if donation_transactions.is_empty() {
            return Err(GetDonationTransactionsError::NotFound);
        }
        Ok(donation_transactions
            .into_iter()
            .map(DonationHistoryDTO::from)
            .collect())
    }

    async fn get_grouped_donation_transaction_history(
        &self,
        donation_filter: &DonationFilter,
        donation_grouping: &DonationGrouping,
    ) -> Result<HashMap<String, Vec<DonationHistoryDTO>>, GroupedDonationHistoryError> {
        let donation_transaction = get_donation_transaction_history_common(
            get_stored_procedure(&donation_grouping),
            &donation_filter,
        )
        .fetch_all(&*self.db_connection.clone())
        .await?;
    }

    async fn get_grouped_donation_transaction_history_count(
        &self,
        filters: &DonationFilter,
        donation_grouping: &DonationGrouping,
    ) -> Result<HashMap<String, u32>, GroupedDonationHistoryError> {
        let donation_transaction_history_count = get_donation_transaction_history_common(
            get_stored_procedure(&donation_grouping),
            &filters,
        )
        .fetch_all(&*self.db_connection.clone())
        .await?;
        let mut grouped_donation_history_count = HashMap::new();
        for row in donation_transaction_history_count {
            grouped_donation_history_count.insert(row.get(0), row.get(1));
        }
        Ok(grouped_donation_history_count)
    }
}

fn get_donation_transaction_history_common<'a>(
    query: &'static str,
    filters: &'a DonationFilter,
) -> Query<'a, MySql, MySqlArguments> {
    let donation_intention = filters
        .donation_intention
        .map(|intention| intention.to_string());
    let donation_frequency = filters
        .donation_frequency
        .map(|frequency| frequency.to_string());
    let transaction_status = filters.transaction_status.map(|status| status.to_string());
    sqlx::query(query)
        .bind(&filters.donation_cause)
        .bind(donation_intention)
        .bind(&filters.email)
        .bind(&filters.phone_number)
        .bind(&filters.amount)
        .bind(&filters.is_gift_aid)
        .bind(donation_frequency)
        .bind(transaction_status)
}
pub async fn new_donation_history_admin_repository(
    repository_mode: RepositoryMode,
) -> Arc<dyn DonationHistoryAdminRepository> {
    new_repository!(repository_mode, RepositoryType::Donation)
}

#[inline]
fn get_stored_procedure(donation_grouping: &DonationGrouping) -> &'static str {
    const STORED_PROCEDURE_BY_CAUSE: &'static str =
        "CALL get_donation_transactions_group_by_cause(?, ?, ?, ?, ?, ?, ?, ?);";
    const STORED_PROCEDURE_BY_INTENTION: &'static str =
        "CALL get_donation_transactions_group_by_intention(?, ?, ?, ?, ?, ?, ?, ?);";
    const STORED_PROCEDURE_BY_AMOUNT: &'static str =
        "CALL get_donation_transactions_group_by_amount(?, ?, ?, ?, ?, ?, ?, ?);";
    const STORED_PROCEDURE_BY_GIFT_AID: &'static str =
        "CALL get_donation_transactions_group_by_gift_aid(?, ?, ?, ?, ?, ?, ?, ?);";
    const STORED_PROCEDURE_BY_FREQUENCY: &'static str =
        "CALL get_donation_transactions_group_by_frequency(?, ?, ?, ?, ?, ?, ?, ?);";
    const STORED_PROCEDURE_BY_STATUS: &'static str =
        "CALL get_donation_transactions_group_by_status(?, ?, ?, ?, ?, ?, ?, ?);";
    match donation_grouping {
        DonationGrouping::Cause => STORED_PROCEDURE_BY_CAUSE,
        DonationGrouping::Intention => STORED_PROCEDURE_BY_INTENTION,
        DonationGrouping::Amount => STORED_PROCEDURE_BY_AMOUNT,
        DonationGrouping::GiftAid => STORED_PROCEDURE_BY_GIFT_AID,
        DonationGrouping::Frequency => STORED_PROCEDURE_BY_FREQUENCY,
        DonationGrouping::Status => STORED_PROCEDURE_BY_STATUS,
    }
}

#[inline]
fn get_donation_filter_parameter_strings(
    filters: &DonationFilter,
) -> (Option<String>, Option<String>, Option<String>) {
    let donation_intention = filters
        .donation_intention
        .and_then(|intention| Some(intention.to_string()));
    let donation_frequency = filters
        .donation_frequency
        .and_then(|frequency| Some(frequency.to_string()));
    let transaction_status = filters
        .transaction_status
        .and_then(|status| Some(status.to_string()));
    (donation_intention, donation_frequency, transaction_status)
}

#[inline]
fn donation_history_from_my_sql_row(row: MySqlRow) -> DonationHistory {
    DonationHistory {
        id: row.get(0),
        cause: row.get(1),
        donation_intention: row.get(2),
        donor_full_name: row.get(3),
        donor_title: row.get(4),
        phone_number: row.get(5),
        email: row.get(6),
        address_line_1: row.get(7),
        address_line_2: row.get(8),
        address_city: row.get(9),
        address_region: row.get(10),
        address_country: row.get(11),
        address_postal: row.get(12),
        amount: row.get(13),
        is_gift_aid: row.get(14),
        donation_frequency: row.get(15),
        transaction_status: row.get(16),
    }
}
