# 16 — Tables and SQL

<sub>rust_lessons 0.20.0 · source: `src/section_16_*.rs` · run: `cargo run --example 16_databases`</sub>

Section 16 — tables and SQL.

# Subgoals

1. **Declare the shape.** Columns and their types, before any row exists — the DDL is the
   decision, the rows are consequences.
2. **Name the key.** Which column identifies a row is what makes `update` and `delete`
   safe rather than sweeping.
3. **Pick the operation, not the SQL.** Create, read, update, delete — then the statement
   follows, which is why each method here also prints its SQL.
4. **Say what an empty result means.** No rows matching is `Ok(vec![])`, not an error;
   a wrong column name *is* one.

# Worked example — a books table through all four operations

"Track books, and change one when it is reissued." The four moves are the four
decisions, in the order a schema is actually designed.

1. **Declare the shape.** Three columns, named and typed, before a row exists:
   `id`, `title`, `author`. That declaration *is* the table, and it is what the
   printed DDL says back.
2. **Name the key.** `id` identifies a row, so `update_where` and `delete_where`
   take the key column: one row changes, and nothing sweeps the table by
   accident.
3. **Pick the operation, not the SQL.** Insert, select, update, delete — and each
   method also prints the statement it stands for, so the SQL follows the
   decision instead of driving it.
4. **Say what an empty result means.** A select that matches nothing is `Ok(0)`
   rows and stays `Ok`; a column that does not exist is the error, and it says
   which column was asked for:

```text
let mut books = Table::new("books", &["id", "title", "author"]);
println!("{}", books.ddl());
// Output: CREATE TABLE books (id, title, author);
books.insert(&["1", "The Dispossessed", "Le Guin"]).unwrap();
books.insert(&["2", "The Left Hand of Darkness", "Le Guin"]).unwrap();
println!("{}", books.update_sql("id", "title"));
// Output: UPDATE books SET title = ? WHERE id = ?;
println!("{:?}", books.update_where("id", "1", "title", "The Dispossessed (1974)"));
// Output: Ok(1)
println!("{:?}", books.select_where("author", "Nobody").map(|r| r.len()));
// Output: Ok(0)
println!("{:?}", books.select_where("isbn", "1").unwrap_err());
// Output: "books has no column named isbn"
```

# SQL, DDL and DML

The deck's last three slides: relational databases speak SQL, tables are created
with DDL and changed with DML, and applications do four things with them —
**C**reate, **R**ead, **U**pdate, **D**elete, the nickname **CRUD**.

There is no database dependency in this course, so the table is a `Vec` of rows
and every operation is spelled twice: once as the method you call, once as the
SQL it corresponds to. That pairing is the lesson — the SQL is not magic, it is
a notation for these four operations.

One row, in column order.

A table: a name, its columns, and its rows.

The `CREATE TABLE` from the deck's DDL slide, as data.
`CREATE TABLE books (id INTEGER PRIMARY KEY, title TEXT NOT NULL, author TEXT)`

The DDL this table stands for.

Number of rows.

True when nothing has been inserted.

**C**reate: `INSERT INTO t VALUES (...)`.

Refuses a row whose width does not match the columns, which is the mistake a
real database also refuses.

The SQL an insert corresponds to.

**R**ead: every row where `column` equals `value`.
`SELECT * FROM t WHERE column = value`.

**R**ead everything. `SELECT * FROM t`.

The SQL a select corresponds to.

**U**pdate one column of every matching row. `UPDATE t SET col = ? WHERE key = ?`.

The SQL an update corresponds to.

**D**elete every matching row. `DELETE FROM t WHERE col = ?`.

The SQL a delete corresponds to.

Column position, and the friendly error a wrong column name deserves.

The four CRUD operations, named — the deck's closing vocabulary.

