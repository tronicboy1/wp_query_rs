use wp_query_rs::*;

#[cfg(feature = "query_sync")]
#[test]
fn can_get_user_by_id() {
    let user = WpUser::get_user_by_id(1).unwrap().unwrap();

    assert_eq!(user.id, 1);
}
