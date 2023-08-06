use rusqlite::{Connection, Result};

pub fn init(conn: &Connection) {
    conn.execute(
        "CREATE TABLE TempuratureRecords (
                  id              INTEGER PRIMARY KEY,
                  temp            DECIMAL(10, 5) NOT NULL
                  )",
        [],
    )
    .expect("Unable to create TempuratureRecords table");
    conn.execute("INSERT INTO TempuratureRecords(temp) VALUES (5.18)", [])
        .expect("Unable to insert into TempuratureRecords table");
}
