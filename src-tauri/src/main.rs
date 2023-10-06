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

async fn init_db() -> Pool<Sqlite> {
    let pool: Pool<Sqlite> = sqlite::SqlitePoolOptions::new()
        .max_connections(5)
        .connect(r#"sqlite://../bouc.db?mode=rwc"#)
        .await
        .expect("connection init failed");
    pool
}

#[tauri::command]
async fn init_book_and_add(
    title: String,
    author: String,
    year: i32,
    genre: String,
    theme: String,
    place: String,
    difficulty: i8,
    read: bool,
    copies: i32,
    meta_book: bool,
    fluff: String,
) -> String {
    let book = Book {
        id: 0,
        title,
        author,
        year,
        genre,
        theme,
        place,
        difficulty,
        read,
        copies,
        meta_book,
        fluff,
    };

    add(book)
        .await
        .expect("Book Insertion failed from outside the add function");

    String::from("Successfully added book to database.")
}

async fn add(book: Book) -> anyhow::Result<()> {
    let pool = init_db().await;
    let mut tx = pool.begin().await.expect("begin tx");

    sqlx::query(r#"INSERT INTO biblio(title,author,year,genre,theme,place,difficulty,read,copies,meta_book,fluff) VALUES($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11)"#)
    .bind(book.title)
    .bind(book.author)
    .bind(book.year)
    .bind(book.genre)
    .bind(book.theme)
    .bind(book.place)
    .bind(book.difficulty)
    .bind(book.read)
    .bind(book.copies)
    .bind(book.meta_book)
    .bind(book.fluff)
    .execute(&mut *tx)
    .await
    .unwrap_or_else(|_| panic!("insertion tx failed"));

    tx.commit().await.expect("tx commit failed");

    Ok(())
}

#[tauri::command]
async fn delete(id: i64) -> String {
    let pool = init_db().await;
    let mut tx = pool.begin().await.expect("begin tx");

    sqlx::query(r#"DELETE FROM biblio WHERE id=$1"#)
        .bind(id)
        .execute(&mut *tx)
        .await
        .unwrap_or_else(|_| panic!("removal tx failed"));

    tx.commit().await.expect("tx commit failed");

    String::from("Successfully removed book from database.")
}

#[tauri::command]
async fn modify(id: i64, field_name: String, new_value: String) -> String {
    let pool = init_db().await;
    let mut tx = pool.begin().await.expect("begin tx");
    let mut q_string: String = "UPDATE biblio SET ".to_string();
    q_string += &field_name;
    q_string += "='";
    q_string += &new_value;
    q_string += "' WHERE id=";
    q_string += &id.to_string();

    // dbg!(&q_string);

    sqlx::query(&q_string)
        .execute(&mut *tx)
        .await
        .unwrap_or_else(|_| panic!("update tx failed"));

    tx.commit().await.expect("tx commit failed");
    String::from("Successfully modified book in database.")
}

#[tauri::command]
async fn fetch_all() -> Vec<Book> {
    let pool = init_db().await;
    sqlx::query_as::<_, Book>("SELECT * FROM biblio")
        .fetch_all(&pool)
        .await
        .expect("query all failed")
}

#[tauri::command]
async fn fetch_one_book(id: String) -> Vec<Book> {
    let pool = init_db().await;
    let q_string: String = "SELECT * FROM biblio WHERE id=".to_string();
    sqlx::query_as::<_, Book>(&(q_string + &id))
        .fetch_all(&pool)
        .await
        .expect("query one book by id failed")
}

#[tauri::command]
async fn fetch_if_contains(field_name: String, substring: String) -> Vec<Book> {
    let pool = init_db().await;
    let q_string: String = "SELECT * FROM biblio WHERE ".to_string();
    sqlx::query_as::<_, Book>(&(q_string + &field_name + " LIKE '%" + &substring + "%'"))
        .fetch_all(&pool)
        .await
        .expect("dynamic query result failed")
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let pool: Pool<Sqlite> = init_db().await;

    sqlx::query(
        r#"CREATE TABLE IF NOT EXISTS biblio  (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    title TEXT,
                    author TEXT,
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

    // let les_mis = Book {
    //     id: 30,
    //     title: "Les Misérables".to_string(),
    //     author: "Victor Hugo".to_string(),
    //     year: 1876,
    //     genre: "Romantisme".to_string(),
    //     theme: "Divers".to_string(),
    //     place: "Mur de gauche".to_string(),
    //     difficulty: 4,
    //     read: true,
    //     meta_book: false,
    //     copies: 3,
    //     fluff: "".to_string(),
    // };

    // add(les_mis).await?;

    // let field_name = "title";
    // let substring = "Miser";
    // let v = fetch_if_contains(field_name, substring).await;

    // dbg!(v);

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            test_command,
            fetch_all,
            fetch_one_book,
            fetch_if_contains,
            init_book_and_add,
            modify,
            delete
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
