use rusqlite::Connection;

pub fn init(conn: &Connection) {
    conn.execute(
        "CREATE TABLE TempuratureRecords (
                  id              INTEGER PRIMARY KEY,
                  temp            DECIMAL(10, 5) NOT NULL,
                  humidity            DECIMAL(10, 5) NOT NULL
                  )",
        [],
    )
    .expect("Unable to create TempuratureRecords table");
    conn.execute("INSERT INTO TempuratureRecords(temp, humidity) VALUES (5.18, 31.0)", [])
        .expect("Unable to insert into TempuratureRecords table");
}
