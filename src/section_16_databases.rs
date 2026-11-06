//! Section 16 — tables and SQL.
//!
//! # Subgoals
//!
//! 1. **Declare the shape.** Columns and their types, before any row exists — the DDL is the
//!    decision, the rows are consequences.
//! 2. **Name the key.** Which column identifies a row is what makes `update` and `delete`
//!    safe rather than sweeping.
//! 3. **Pick the operation, not the SQL.** Create, read, update, delete — then the statement
//!    follows, which is why each method here also prints its SQL.
//! 4. **Say what an empty result means.** No rows matching is `Ok(vec![])`, not an error;
//!    a wrong column name *is* one.
//! The deck's last three slides: relational databases speak SQL, tables are created
//! with DDL and changed with DML, and applications do four things with them —
//! **C**reate, **R**ead, **U**pdate, **D**elete, the nickname **CRUD**.
//!
//! There is no database dependency in this course, so the table is a `Vec` of rows
//! and every operation is spelled twice: once as the method you call, once as the
//! SQL it corresponds to. That pairing is the lesson — the SQL is not magic, it is
//! a notation for these four operations.

/// One row, in column order.
pub type Row = Vec<String>;

/// A table: a name, its columns, and its rows.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Table {
    pub name: String,
    pub columns: Vec<String>,
    rows: Vec<Row>,
}

impl Table {
    /// The `CREATE TABLE` from the deck's DDL slide, as data.
    /// `CREATE TABLE books (id INTEGER PRIMARY KEY, title TEXT NOT NULL, author TEXT)`
    pub fn new(name: &str, columns: &[&str]) -> Self {
        Table {
            name: name.to_string(),
            columns: columns.iter().map(|c| c.to_string()).collect(),
            rows: Vec::new(),
        }
    }

    /// The DDL this table stands for.
    pub fn ddl(&self) -> String {
        format!(
            "CREATE TABLE {} ({});",
            self.name,
            self.columns.join(", ")
        )
    }

    /// Number of rows.
    pub fn len(&self) -> usize {
        self.rows.len()
    }

    /// True when nothing has been inserted.
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    /// **C**reate: `INSERT INTO t VALUES (...)`.
    ///
    /// Refuses a row whose width does not match the columns, which is the mistake a
    /// real database also refuses.
    pub fn insert(&mut self, row: &[&str]) -> Result<usize, String> {
        if row.len() != self.columns.len() {
            return Err(format!(
                "{} has {} columns, row has {}",
                self.name,
                self.columns.len(),
                row.len()
            ));
        }
        self.rows.push(row.iter().map(|v| v.to_string()).collect());
        Ok(self.rows.len())
    }

    /// The SQL an insert corresponds to.
    pub fn insert_sql(&self) -> String {
        format!(
            "INSERT INTO {} ({}) VALUES ({});",
            self.name,
            self.columns.join(", "),
            vec!["?"; self.columns.len()].join(", ")
        )
    }

    /// **R**ead: every row where `column` equals `value`.
    /// `SELECT * FROM t WHERE column = value`.
    pub fn select_where(&self, column: &str, value: &str) -> Result<Vec<&Row>, String> {
        let idx = self.column_index(column)?;
        Ok(self.rows.iter().filter(|r| r[idx] == value).collect())
    }

    /// **R**ead everything. `SELECT * FROM t`.
    pub fn select_all(&self) -> &[Row] {
        &self.rows
    }

    /// The SQL a select corresponds to.
    pub fn select_sql(&self, column: Option<&str>) -> String {
        match column {
            Some(c) => format!("SELECT * FROM {} WHERE {} = ?;", self.name, c),
            None => format!("SELECT * FROM {};", self.name),
        }
    }

    /// **U**pdate one column of every matching row. `UPDATE t SET col = ? WHERE key = ?`.
    pub fn update_where(
        &mut self,
        key_column: &str,
        key_value: &str,
        set_column: &str,
        new_value: &str,
    ) -> Result<usize, String> {
        let k = self.column_index(key_column)?;
        let s = self.column_index(set_column)?;
        let mut changed = 0;
        for row in self.rows.iter_mut() {
            if row[k] == key_value {
                row[s] = new_value.to_string();
                changed += 1;
            }
        }
        Ok(changed)
    }

    /// The SQL an update corresponds to.
    pub fn update_sql(&self, key_column: &str, set_column: &str) -> String {
        format!(
            "UPDATE {} SET {} = ? WHERE {} = ?;",
            self.name, set_column, key_column
        )
    }

    /// **D**elete every matching row. `DELETE FROM t WHERE col = ?`.
    pub fn delete_where(&mut self, column: &str, value: &str) -> Result<usize, String> {
        let idx = self.column_index(column)?;
        let before = self.rows.len();
        self.rows.retain(|r| r[idx] != value);
        Ok(before - self.rows.len())
    }

    /// The SQL a delete corresponds to.
    pub fn delete_sql(&self, column: &str) -> String {
        format!("DELETE FROM {} WHERE {} = ?;", self.name, column)
    }

    /// Column position, and the friendly error a wrong column name deserves.
    fn column_index(&self, column: &str) -> Result<usize, String> {
        self.columns
            .iter()
            .position(|c| c == column)
            .ok_or_else(|| format!("{} has no column named {column}", self.name))
    }
}

/// The four CRUD operations, named — the deck's closing vocabulary.
pub fn crud() -> [(&'static str, &'static str, &'static str); 4] {
    [
        ("Create", "INSERT", "add a row"),
        ("Read", "SELECT", "fetch rows"),
        ("Update", "UPDATE", "change a row"),
        ("Delete", "DELETE", "remove a row"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn books() -> Table {
        let mut t = Table::new("books", &["id", "title", "author"]);
        t.insert(&["1", "Dune", "Herbert"]).unwrap();
        t.insert(&["2", "Emma", "Austen"]).unwrap();
        t.insert(&["3", "Dune Messiah", "Herbert"]).unwrap();
        t
    }

    #[test]
    fn create_then_read() {
        let t = books();
        assert_eq!(t.len(), 3);
        assert_eq!(t.select_where("author", "Herbert").unwrap().len(), 2);
        assert_eq!(t.select_where("title", "Emma").unwrap().len(), 1);
        assert!(t.select_where("nope", "x").is_err(), "unknown column is an error");
        assert_eq!(t.ddl(), "CREATE TABLE books (id, title, author);");
    }

    #[test]
    fn width_mismatch_is_refused_like_a_real_database() {
        let mut t = books();
        assert!(t.insert(&["4", "OnlyTwo"]).is_err());
        assert_eq!(t.len(), 3);
    }

    #[test]
    fn update_changes_only_matching_rows() {
        let mut t = books();
        assert_eq!(t.update_where("id", "2", "title", "Emma (1815)").unwrap(), 1);
        let row = t.select_where("id", "2").unwrap()[0].clone();
        assert_eq!(row[1], "Emma (1815)");
        assert_eq!(t.update_where("id", "99", "title", "x").unwrap(), 0);
    }

    #[test]
    fn delete_removes_matching_rows() {
        let mut t = books();
        assert_eq!(t.delete_where("author", "Herbert").unwrap(), 2);
        assert_eq!(t.len(), 1);
        assert_eq!(t.select_all()[0][1], "Emma");
        assert!(t.delete_where("nope", "x").is_err());
    }

    #[test]
    fn every_operation_has_its_sql() {
        let t = books();
        assert_eq!(
            t.insert_sql(),
            "INSERT INTO books (id, title, author) VALUES (?, ?, ?);"
        );
        assert_eq!(t.select_sql(None), "SELECT * FROM books;");
        assert_eq!(t.select_sql(Some("id")), "SELECT * FROM books WHERE id = ?;");
        assert_eq!(t.update_sql("id", "title"), "UPDATE books SET title = ? WHERE id = ?;");
        assert_eq!(t.delete_sql("id"), "DELETE FROM books WHERE id = ?;");
        assert_eq!(crud().len(), 4);
    }
}
