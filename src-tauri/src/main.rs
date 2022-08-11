#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use sqlx::{sqlite, Pool, Sqlite};

#[tauri::command]
fn test_command() {
    println!("I was called by the frontend!");
}

#[derive(Debug, sqlx::FromRow, serde::Serialize, serde::Deserialize)]
struct Book {
    id: i64,
    title: Option<String>,
    year: i32,
}

async fn init_db() -> Pool<Sqlite> {
    let pool: Pool<Sqlite> = sqlite::SqlitePoolOptions::new()
        .max_connections(5)
        .connect(r#"sqlite://../bouc.db?mode=rwc"#)
        .await
        .expect("?");
    pool
}

async fn add(title: String, year: i64) -> anyhow::Result<()> {
    let pool = init_db().await;
    let mut tx = pool.begin().await.expect("begin tx");

    sqlx::query(r#"INSERT INTO biblio(title,year) VALUES($1,$2)"#)
        .bind(title)
        .bind(year)
        .execute(&mut tx)
        .await
        .unwrap_or_else(|_| panic!("insert test in tx"));

    tx.commit().await.expect("tx commit");

    Ok(())
}

#[tauri::command]
async fn fetch_all() -> Vec<Book> {
    let pool = init_db().await;
    let v = sqlx::query_as::<_, Book>("SELECT * FROM biblio")
        .fetch_all(&pool)
        .await
        .expect("query all");
    v
}

async fn fetch_if_contains(field_name: &str, substring: &str) -> Vec<Book> {
    let pool = init_db().await;
    let q_string: String = "SELECT * FROM biblio WHERE ".to_owned();
    let v = sqlx::query_as::<_, Book>(&(q_string + field_name + " LIKE '%" + substring + "%'"))
        .fetch_all(&pool)
        .await
        .expect("dynamic query result");
    v
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let pool: Pool<Sqlite> = init_db().await;

    sqlx::query(
        r#"CREATE TABLE IF NOT EXISTS biblio  (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    title TEXT,
                    year INTEGER
               )
            "#,
    )
    .execute(&pool)
    .await
    .expect("create table");

    add(String::from("Les Miserables"), 1879).await?;
    add(String::from("Les Miserables 2"), 1882).await?;

    // let field_name = "title";
    // let substring = "Miser";
    // let v = fetch_if_contains(field_name, substring).await;

    // dbg!(v);

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![test_command, fetch_all])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
