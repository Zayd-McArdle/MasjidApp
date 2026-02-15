use crate::features::donation::errors::donation_history_admin_repository_errors::GetDonationTransactionsError;
use crate::features::donation::errors::grouped_donation_history_error::GroupedDonationHistoryError;
use crate::features::donation::models::donation_dto::DonationHistoryDTO;
use crate::features::donation::models::donation_filter::DonationFilter;
use crate::features::donation::models::donation_filter_with_grouping::DonationFilterWithGrouping;
use crate::features::donation::models::donation_grouping::DonationGrouping;
use crate::features::donation::repositories::donation_history_admin_repository::util::{
    get_stored_procedure_internal, group_donation_transaction_history_dtos,
};
use crate::features::donation::repositories::donation_history_admin_repository::DonationHistoryAdminRepository;
use async_trait::async_trait;
use masjid_app_api_library::features::donation::models::donation_history::DonationHistory;
use masjid_app_api_library::shared::data_access::db_retrieval_mode::DBRetrievalMode;
use masjid_app_api_library::shared::data_access::repository_manager::MySqlRepository;
use sqlx::mysql::{MySqlArguments, MySqlRow};
use sqlx::query::Query;
use sqlx::{Error, MySql, Row};
use std::collections::HashMap;

#[async_trait]
impl DonationHistoryAdminRepository for MySqlRepository {
    async fn get_donation_transactions(
        &self,
        filters: &DonationFilter,
    ) -> Result<Vec<DonationHistoryDTO>, GetDonationTransactionsError> {
        const STORED_PROCEDURE: &'static str =
            "CALL get_donation_transactions(?, ?, ?, ?, ?, ?, ?, ?)";
        let db_connection = self.db_connection.clone();
        let donation_transactions =
            get_donation_transaction_history_common(STORED_PROCEDURE, &filters)
                .map(donation_history_from_my_sql_row)
                .fetch_all(&*db_connection)
                .await
                .map_err(|err| {
                    tracing::error!(
                        stored_procedure = STORED_PROCEDURE,
                        error = err.to_string(),
                        "unable to fetch donation transaction history from database"
                    );
                    GetDonationTransactionsError::UnableToFetchDonationTransactions
                })?;
        Ok(donation_transactions
            .into_iter()
            .map(DonationHistoryDTO::from)
            .collect())
    }

    async fn get_grouped_donation_transaction_history(
        &self,
        donation_filter: &DonationFilterWithGrouping,
    ) -> Result<HashMap<String, Vec<DonationHistoryDTO>>, GroupedDonationHistoryError> {
        let stored_procedure = get_mysql_stored_procedure(
            &donation_filter.donation_grouping,
            DBRetrievalMode::Records,
        );
        let donation_transaction_dtos: Vec<DonationHistoryDTO> =
            get_donation_transaction_history_common(stored_procedure, &donation_filter.filter)
                .map(donation_history_from_my_sql_row)
                .fetch_all(&*self.db_connection.clone())
                .await
                .map_err(|err| {
                    map_sqlx_error_to_grouped_donation_history_error(
                        stored_procedure,
                        donation_filter,
                        err,
                    )
                })?
                .into_iter()
                .map(DonationHistoryDTO::from)
                .collect();

        Ok(group_donation_transaction_history_dtos(
            donation_transaction_dtos,
            &donation_filter.donation_grouping,
        ))
    }

    async fn get_grouped_donation_transaction_history_count(
        &self,
        donation_filter: &DonationFilterWithGrouping,
    ) -> Result<HashMap<String, i64>, GroupedDonationHistoryError> {
        let stored_procedure =
            get_mysql_stored_procedure(&donation_filter.donation_grouping, DBRetrievalMode::Count);
        let donation_transaction_history_count =
            get_donation_transaction_history_common(stored_procedure, &donation_filter.filter)
                .fetch_all(&*self.db_connection.clone())
                .await
                .map_err(|err| {
                    map_sqlx_error_to_grouped_donation_history_error(
                        stored_procedure,
                        donation_filter,
                        err,
                    )
                })?;
        let mut grouped_donation_history_count = HashMap::new();

        for row in donation_transaction_history_count {
            let grouping_key = match donation_filter.donation_grouping {
                // Index 0 is the grouped field name
                DonationGrouping::Amount => row.get::<'_, u32, usize>(0).to_string(),
                DonationGrouping::GiftAid => row.get::<'_, bool, usize>(0).to_string(),
                DonationGrouping::Cause
                | DonationGrouping::Intention
                | DonationGrouping::Frequency
                | DonationGrouping::TransactionStatus => row.get(0),
            };
            // Index 1 contains the row count
            grouped_donation_history_count.insert(grouping_key, row.get(1));
        }
        Ok(grouped_donation_history_count)
    }
}

#[inline]
fn map_sqlx_error_to_grouped_donation_history_error(
    stored_procedure: &'static str,
    donation_filter: &DonationFilterWithGrouping,
    err: Error,
) -> GroupedDonationHistoryError {
    tracing::error!(
        stored_procedure = stored_procedure,
        error = err.to_string(),
        grouping = donation_filter.donation_grouping.to_string(),
        "unable to fetch donation transaction history grouping from database"
    );
    GroupedDonationHistoryError::UnableToGetGroupedDonationHistory
}

#[inline]
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

    tracing::debug!(
        donation_cause = &filters.donation_cause,
        donation_intention = donation_intention,
        email = &filters.email,
        phone_number = &filters.phone_number,
        amount = &filters.amount,
        is_gift_aid = &filters.is_gift_aid,
        donation_frequency = &donation_frequency,
        transaction_status = &transaction_status,
        query = query,
        "retrieving data from mysql database"
    );
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

/// Returns a MySQL stored procedure based on particular field grouping and retrieval mode
///
/// # Arguments
///
/// * `donation_grouping`: Determines what database field to group or order by
/// * `db_retrieval_mode`: Determines whether the database is returning records or record counts
///
/// returns: &str
///
/// # Examples
///
/// ```
///
/// ```
#[inline]
const fn get_mysql_stored_procedure(
    donation_grouping: &DonationGrouping,
    db_retrieval_mode: DBRetrievalMode,
) -> &'static str {
    // Filter by cause
    const GROUP_BY_CAUSE: &'static str =
        "CALL get_donation_transactions_group_by_cause(?, ?, ?, ?, ?, ?, ?, ?);";
    const ORDER_BY_CAUSE: &'static str =
        "CALL get_donation_transactions_order_by_cause(?, ?, ?, ?, ?, ?, ?, ?);";

    // Filter by intention
    const GROUP_BY_INTENTION: &'static str =
        "CALL get_donation_transactions_group_by_intention(?, ?, ?, ?, ?, ?, ?, ?);";
    const ORDER_BY_INTENTION: &'static str =
        "CALL get_donation_transactions_order_by_intention(?, ?, ?, ?, ?, ?, ?, ?);";

    // Filter by amount
    const GROUP_BY_AMOUNT: &'static str =
        "CALL get_donation_transactions_group_by_amount(?, ?, ?, ?, ?, ?, ?, ?);";
    const ORDER_BY_AMOUNT: &'static str =
        "CALL get_donation_transactions_order_by_amount(?, ?, ?, ?, ?, ?, ?, ?);";

    // Filter by gift aid
    const GROUP_BY_GIFT_AID: &'static str =
        "CALL get_donation_transactions_group_by_gift_aid(?, ?, ?, ?, ?, ?, ?, ?);";
    const ORDER_BY_GIFT_AID: &'static str =
        "CALL get_donation_transactions_order_by_gift_aid(?, ?, ?, ?, ?, ?, ?, ?);";

    // Filter by frequency
    const GROUP_BY_FREQUENCY: &'static str =
        "CALL get_donation_transactions_group_by_frequency(?, ?, ?, ?, ?, ?, ?, ?);";
    const ORDER_BY_FREQUENCY: &'static str =
        "CALL get_donation_transactions_order_by_frequency(?, ?, ?, ?, ?, ?, ?, ?);";

    // Filter by status
    const GROUP_BY_STATUS: &'static str =
        "CALL get_donation_transactions_group_by_transaction_status(?, ?, ?, ?, ?, ?, ?, ?);";
    const ORDER_BY_STATUS: &'static str =
        "CALL get_donation_transactions_order_by_transaction_status(?, ?, ?, ?, ?, ?, ?, ?);";

    match donation_grouping {
        DonationGrouping::Cause => {
            get_stored_procedure_internal(db_retrieval_mode, GROUP_BY_CAUSE, ORDER_BY_CAUSE)
        }
        DonationGrouping::Intention => {
            get_stored_procedure_internal(db_retrieval_mode, GROUP_BY_INTENTION, ORDER_BY_INTENTION)
        }
        DonationGrouping::Amount => {
            get_stored_procedure_internal(db_retrieval_mode, GROUP_BY_AMOUNT, ORDER_BY_AMOUNT)
        }
        DonationGrouping::GiftAid => {
            get_stored_procedure_internal(db_retrieval_mode, GROUP_BY_GIFT_AID, ORDER_BY_GIFT_AID)
        }
        DonationGrouping::Frequency => {
            get_stored_procedure_internal(db_retrieval_mode, GROUP_BY_FREQUENCY, ORDER_BY_FREQUENCY)
        }
        DonationGrouping::TransactionStatus => {
            get_stored_procedure_internal(db_retrieval_mode, GROUP_BY_STATUS, ORDER_BY_STATUS)
        }
    }
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
