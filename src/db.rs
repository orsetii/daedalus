use rusqlite::Connection;

pub fn init(conn: &Connection) {
    conn.execute(
        "CREATE TABLE InternalTemperatureRecords (
                  id              INTEGER PRIMARY KEY,
                  temp            DECIMAL(10, 5) NOT NULL,
                  humidity            DECIMAL(10, 5) NOT NULL
                  )",
        [],
    )
    .expect("Unable to create InternalTemperatureRecords table");
    conn.execute("INSERT INTO InternalTemperatureRecords(temp, humidity) VALUES (0.00, 0.00)", [])
        .expect("Unable to insert into InternalTemperatureRecords table");


    conn.execute(
        "CREATE TABLE ExternalTemperatureRecords (
                  id              INTEGER PRIMARY KEY,
                  temp            DECIMAL(10, 5) NOT NULL,
                  humidity            DECIMAL(10, 5) NOT NULL,
                  pressure            DECIMAL(10, 5) NOT NULL
                  )",
        [],
    )
    .expect("Unable to create ExternalTemperatureRecords table");
    conn.execute("INSERT INTO ExternalTemperatureRecords(temp, humidity, pressure) VALUES (0.00, 0.00, 0.00)", [])
        .expect("Unable to insert into ExternalTemperatureRecords table");
}
