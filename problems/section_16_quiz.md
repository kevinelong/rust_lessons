# Retrieval check — section 16: Tables and SQL

Answer from memory before looking. Each question comes from a sentence in the lesson, with one term removed.

1. There is no database dependency in this course, so the table is a **____** of rows and every operation is spelled twice: once as the method you call, once as the SQL it corresponds to.

2. The `CREATE TABLE` from the deck's DDL slide, as data. **____**

3. Read: every row where `column` equals `value`. **____**.

4. Update one column of every matching row. **____**.

5. Delete every matching row. **____**.

<details>
<summary>Answers</summary>

1. `Vec`
2. `CREATE TABLE books (id INTEGER PRIMARY KEY, title TEXT NOT NULL, author TEXT)`
3. `SELECT * FROM t WHERE column = value`
4. `UPDATE t SET col = ? WHERE key = ?`
5. `DELETE FROM t WHERE col = ?`

</details>

<sub>Generated from `src/section_16_*.rs` by `cargo run --bin quiz` — rust_lessons 0.16.0</sub>
