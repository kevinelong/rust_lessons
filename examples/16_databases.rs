//! Section 16, running: a table, CRUD, and the SQL for each operation.
//!
//! `cargo run --example 16_databases`

use rust_lessons::section_16_databases::{crud, Table};

fn main() {
    println!("rust_lessons {} — section 16", rust_lessons::VERSION);

    for (name, sql, meaning) in crud() {
        println!("  {name:<7} {sql:<7} {meaning}");
    }

    println!();
    let mut books = Table::new("books", &["id", "title", "author"]);
    println!("  DDL: {}", books.ddl());
    println!("  C:   {}", books.insert_sql());

    books.insert(&["1", "Dune", "Herbert"]).unwrap();
    books.insert(&["2", "Emma", "Austen"]).unwrap();
    books.insert(&["3", "Dune Messiah", "Herbert"]).unwrap();
    println!("  after three inserts: {} rows", books.len());

    println!("  R:   {}", books.select_sql(Some("author")));
    for row in books.select_where("author", "Herbert").unwrap() {
        println!("       {row:?}");
    }

    println!("  U:   {}", books.update_sql("id", "title"));
    println!("       changed {} row(s)", books.update_where("id", "2", "title", "Emma (1815)").unwrap());

    println!("  D:   {}", books.delete_sql("author"));
    println!("       removed {} row(s)", books.delete_where("author", "Herbert").unwrap());

    println!();
    println!("  final table:");
    for row in books.select_all() {
        println!("    {row:?}");
    }
}
