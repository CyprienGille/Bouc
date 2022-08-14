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
    title: String,
    author: String,
    century: String,
    year: i32,
    genre: String,
    theme: String,
    place: String,
    difficulty: i8,
    read: bool,
    copies: i32,
    meta_book: bool,
    fluff: String,
}

fn get_century_from_year(year: i64) -> String {
    "XVIIeme".to_string()
}

async fn init_db() -> Pool<Sqlite> {
    let pool: Pool<Sqlite> = sqlite::SqlitePoolOptions::new()
        .max_connections(5)
        .connect(r#"sqlite://../bouc.db?mode=rwc"#)
        .await
        .expect("connection init failed");
    pool
}

async fn add(book: Book) -> anyhow::Result<()> {
    let pool = init_db().await;
    let mut tx = pool.begin().await.expect("begin tx");

    sqlx::query(r#"INSERT INTO biblio(title,author,century,year,genre,theme,place,difficulty,read,copies,meta_book,fluff) VALUES($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12)"#)
        .bind(book.title)
        .bind(book.author)
        .bind(book.century)
        .bind(book.year)
        .bind(book.genre)
        .bind(book.theme)
        .bind(book.place)
        .bind(book.difficulty)
        .bind(book.read)
        .bind(book.copies)
        .bind(book.meta_book)
        .bind(book.fluff)
        .execute(&mut tx)
        .await
        .unwrap_or_else(|_| panic!("insert test in tx failed"));

    tx.commit().await.expect("tx commit failed");

    Ok(())
}

#[tauri::command]
async fn fetch_all() -> Vec<Book> {
    let pool = init_db().await;
    let v = sqlx::query_as::<_, Book>("SELECT * FROM biblio")
        .fetch_all(&pool)
        .await
        .expect("query all failed");
    v
}

#[tauri::command]
async fn fetch_one_book(id: String) -> Vec<Book> {
    let pool = init_db().await;
    let q_string: String = "SELECT * FROM biblio WHERE id=".to_string();
    let v = sqlx::query_as::<_, Book>(&(q_string + &id))
        .fetch_all(&pool)
        .await
        .expect("query one book by id failed");
    v
}

async fn fetch_if_contains(field_name: &str, substring: &str) -> Vec<Book> {
    let pool = init_db().await;
    let q_string: String = "SELECT * FROM biblio WHERE ".to_owned();
    let v = sqlx::query_as::<_, Book>(&(q_string + field_name + " LIKE '%" + substring + "%'"))
        .fetch_all(&pool)
        .await
        .expect("dynamic query result failed");
    v
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let pool: Pool<Sqlite> = init_db().await;

    sqlx::query(
        r#"CREATE TABLE IF NOT EXISTS biblio  (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    title TEXT,
                    author TEXT,
                    century TEXT,
                    year INTEGER,
                    genre TEXT,
                    theme TEXT,
                    place TEXT,
                    difficulty INTEGER,
                    read INTEGER,
                    copies INTEGER,
                    meta_book INTEGER,
                    fluff TEXT
               )
            "#,
    )
    .execute(&pool)
    .await
    .expect("create table");

    let les_mis = Book {
        id: 30,
        title: "Les Misérables".to_string(),
        author: "Victor Hugo".to_string(),
        year: 1876,
        century: get_century_from_year(1876),
        genre: "Romantisme".to_string(),
        theme: "Divers".to_string(),
        place: "Mur de gauche".to_string(),
        difficulty: 4,
        read: true,
        meta_book: false,
        copies: 3,
        fluff: "".to_string(),
    };

    add(les_mis).await?;

    // let field_name = "title";
    // let substring = "Miser";
    // let v = fetch_if_contains(field_name, substring).await;

    // dbg!(v);

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            test_command,
            fetch_all,
            fetch_one_book
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
