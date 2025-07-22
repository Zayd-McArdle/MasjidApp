/*use masjid_app_api::features::user_authentication;
use masjid_app_api::features::user_authentication::UserAccountDTO;
use masjid_app_api_library::app_state::DbType;

pub(crate) async fn add_user_to_main_database(new_user: UserAccountDTO) {
    let repository = user_authentication::new_user_repository(DbType::MySql).await;
    let register_user_result = repository.register_user(new_user.clone()).await;
    assert!(register_user_result.is_ok());

    let login_result = repository
        .login(&new_user.username, &new_user.password)
        .await;
    assert!(login_result.is_ok());
}
*/
