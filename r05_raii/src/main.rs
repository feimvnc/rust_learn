use rusqlite::{Connection, Result};

struct DatabaseConnection {
    // You can prefix unused struct fields with an underscore as well if needed
    _connection: Connection,
}

impl DatabaseConnection {
    fn new(db_name: &str) -> Result<DatabaseConnection> {
        let connection = Connection::open(db_name)?;
        Ok(DatabaseConnection {
            _connection: connection,
        })
    }
}

fn main() -> Result<()> {
    // Prefixing with `_` suppresses the unused variable warning
    let _connection = DatabaseConnection::new("my_database.db")?;

    // Perform database operations...

    Ok(())
}
