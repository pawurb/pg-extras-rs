use crate::structs::shared::Tabular;
use postgres::Row;

#[derive(Debug)]
pub struct Connections {
    username: String,
    pid: i32,
    client_addr: String,
}

impl Tabular for Connections {
    const FILE_NAME: &'static str = "connections";

    fn new(row: &Row) -> Self {
        Connections {
            username: row.get::<_, Option<String>>(0).unwrap_or_default(),
            pid: row.get::<_, Option<i32>>(1).unwrap_or_default(),
            client_addr: row.get::<_, Option<String>>(2).unwrap_or_default(),
        }
    }

    fn to_row(&self) -> prettytable::Row {
        row![self.username, self.pid, self.client_addr]
    }

    fn headers() -> prettytable::Row {
        row!["username", "pid", "client_addr"]
    }
}
