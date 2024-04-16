use function_name::named;

use itertools::Itertools;
use rwa_api::api::{self, ApiContract};

use serial_test::serial;

use super::common::*;

#[tokio::test]
#[serial]
#[named]
async fn test_get_rwa_accounts_by_mint() {
    let name = trim_test_name(function_name!());
    let setup = TestSetup::new(name.clone()).await;

    let seeds: Vec<SeedEvent> = vec![seed_token_mint(
        "Ea1yrC1xRXd6tWcHL4yhGRB31j6jTwdeqd3e9LHaYUwj",
    )];
    apply_migrations_and_delete_data(setup.db.clone()).await;
    index_seed_events(&setup, seeds.iter().collect_vec()).await;
    let request: api::GetRwaAccountsByMint = serde_json::from_str(
        r#"{
        "id": "Ea1yrC1xRXd6tWcHL4yhGRB31j6jTwdeqd3e9LHaYUwj"
    }"#,
    )
    .unwrap();
    let response = setup
        .rwa_api
        .get_rwa_accounts_by_mint(request)
        .await
        .unwrap();
    insta::assert_json_snapshot!(setup.name.clone(), response);
}

#[tokio::test]
#[serial]
#[named]
async fn test_get_rwa_accounts_by_authority() {
    let name = trim_test_name(function_name!());
    let setup = TestSetup::new(name.clone()).await;

    let seeds: Vec<SeedEvent> = vec![seed_token_mint(
        "dqci6TGXWGTki8DTQ1QcJ3qg4fPbxUzj6MwJBGjuhHX",
    )];
    apply_migrations_and_delete_data(setup.db.clone()).await;
    index_seed_events(&setup, seeds.iter().collect_vec()).await;
    let request: api::GetRwaAccountsByMint = serde_json::from_str(
        r#"{
        "id": "dqci6TGXWGTki8DTQ1QcJ3qg4fPbxUzj6MwJBGjuhHX"
    }"#,
    )
    .unwrap();
    let response = setup
        .rwa_api
        .get_rwa_accounts_by_authority(request)
        .await
        .unwrap();
    insta::assert_json_snapshot!(setup.name.clone(), response);
}

#[tokio::test]
#[serial]
#[named]
async fn test_get_rwa_accounts_by_delegate() {
    let name = trim_test_name(function_name!());
    let setup = TestSetup::new(name.clone()).await;

    let seeds: Vec<SeedEvent> = vec![seed_token_mint(
        "8tAP4FDpFehdQVmwTqCoYtDehG863TJqnTKZYz6UPR6m",
    )];
    apply_migrations_and_delete_data(setup.db.clone()).await;
    index_seed_events(&setup, seeds.iter().collect_vec()).await;
    let request: api::GetRwaAccountsByMint = serde_json::from_str(
        r#"{
        "id": "8tAP4FDpFehdQVmwTqCoYtDehG863TJqnTKZYz6UPR6m"
    }"#,
    )
    .unwrap();
    let response = setup
        .rwa_api
        .get_rwa_accounts_by_delegate(request)
        .await
        .unwrap();
    insta::assert_json_snapshot!(setup.name.clone(), response);
}

// get_rwa_accounts_by_delegate
