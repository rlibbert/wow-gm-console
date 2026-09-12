//! Live integration test against a real AzerothCore world database.
//! Not run as part of normal `cargo test`/CI since it depends on external
//! server state and real credentials -- run explicitly with:
//!
//!   WOW_GM_TEST_DB_HOST=127.0.0.1 WOW_GM_TEST_DB_PORT=3307 \
//!   WOW_GM_TEST_DB_DATABASE=acore_world WOW_GM_TEST_DB_USERNAME=gmconsole_ro \
//!   WOW_GM_TEST_DB_PASSWORD=your-password \
//!   cargo test --test live_db_test -- --ignored

use std::env;

use sqlx::mysql::MySqlPoolOptions;
use wow_gm_console_lib::db::{search_items, search_teleports, DbError, ItemFilter};

struct TestDb {
    host: String,
    port: u16,
    database: String,
    username: String,
    password: String,
}

fn test_db() -> TestDb {
    TestDb {
        host: env::var("WOW_GM_TEST_DB_HOST").expect("set WOW_GM_TEST_DB_HOST"),
        port: env::var("WOW_GM_TEST_DB_PORT")
            .expect("set WOW_GM_TEST_DB_PORT")
            .parse()
            .expect("WOW_GM_TEST_DB_PORT must be a valid port number"),
        database: env::var("WOW_GM_TEST_DB_DATABASE").expect("set WOW_GM_TEST_DB_DATABASE"),
        username: env::var("WOW_GM_TEST_DB_USERNAME").expect("set WOW_GM_TEST_DB_USERNAME"),
        password: env::var("WOW_GM_TEST_DB_PASSWORD").expect("set WOW_GM_TEST_DB_PASSWORD"),
    }
}

async fn connect(db: &TestDb) -> sqlx::MySqlPool {
    let url = format!(
        "mysql://{}:{}@{}:{}/{}",
        db.username, db.password, db.host, db.port, db.database
    );
    MySqlPoolOptions::new()
        .max_connections(2)
        .connect(&url)
        .await
        .expect("should connect to the live world database")
}

#[tokio::test]
#[ignore]
async fn search_teleports_returns_real_rows() {
    let db = test_db();
    let pool = connect(&db).await;

    let results = search_teleports(&pool, "stormwind", 20)
        .await
        .expect("query should succeed");

    assert!(!results.is_empty(), "expected at least one teleport matching 'stormwind'");
    assert!(results.iter().all(|t| t.name.to_lowercase().contains("stormwind")));
}

#[tokio::test]
#[ignore]
async fn search_items_filters_by_class_and_level() {
    let db = test_db();
    let pool = connect(&db).await;

    let filter = ItemFilter {
        name_substring: None,
        class: Some(2),    // Weapon
        subclass: Some(2), // Bow
        quality_min: None,
        required_level_max: Some(40),
    };
    let results = search_items(&pool, &filter, 50)
        .await
        .expect("query should succeed");

    assert!(!results.is_empty(), "expected at least one bow with required level <= 40");
    assert!(results.iter().all(|i| i.class == 2 && i.subclass == 2 && i.required_level <= 40));
}

#[tokio::test]
#[ignore]
async fn bad_password_maps_to_auth_failed() {
    let db = test_db();
    let bad_url = format!(
        "mysql://{}:definitely-not-the-password@{}:{}/{}",
        db.username, db.host, db.port, db.database
    );

    let err = MySqlPoolOptions::new()
        .connect(&bad_url)
        .await
        .expect_err("wrong password should be rejected");

    assert!(matches!(DbError::from(err), DbError::AuthFailed));
}

#[tokio::test]
#[ignore]
async fn readonly_account_cannot_write() {
    let db = test_db();
    let pool = connect(&db).await;

    let result = sqlx::query("DELETE FROM game_tele WHERE id = 999999999")
        .execute(&pool)
        .await;

    assert!(
        result.is_err(),
        "the read-only test account should not be able to run a DELETE"
    );
}
