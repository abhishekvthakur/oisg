use std::io;
use serde::{ Serialize, Deserialize };
use serde_json;
use crate::db;

#[derive(Debug, Serialize, Deserialize)]
pub struct ColumnDetail {
    name: String,
    column_type: String,
    constraints: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TableDetail {
    name: String,
    columns: Vec<ColumnDetail>,
}

pub type TableDetails = Vec<TableDetail>;

pub fn create_all() -> io::Result<()> {
    let (_, db_path) = db::get_db_path()?;
    let connection = match sqlite::open(db_path) {
        Ok(conn) => conn,
        Err(e) => {
            return Err(io::Error::new(
                io::ErrorKind::Other, e.message.unwrap()
            ));
        }
    };

    let table_details = get_table_details()?;

    create_tables_from_details(connection, table_details)
}

fn create_tables_from_details(
    conn: sqlite::Connection,
    table_details: TableDetails
) -> io::Result<()> {
    for table_detail in table_details.iter() {
        let table_query = get_create_table_query(table_detail);
        match conn.execute(table_query) {
            Ok(_) => {}
            Err(e) => {
                return Err(io::Error::new(
                    io::ErrorKind::Other, e.message.unwrap()
                ));
            }
        }
    }

    Ok(())
}

fn get_create_table_query(table_details: &TableDetail) -> String {
    let mut query = String::new();

    let table_name = &table_details.name;
    query.push_str(format!("CREATE TABLE IF NOT EXISTS \"{}\" (", table_name).as_str());
    let mut count: usize = 0;
    for column_detail in &table_details.columns {
        query.push_str(format!(
            "[{}] {}", column_detail.name, column_detail.column_type
        ).as_str());

        if let Some(constraints) = &column_detail.constraints {
            query.push_str(format!(" {}", constraints.join(" ")).as_str())
        }

        count = count + 1;
        if count != table_details.columns.len() {
            query.push_str(",");
        }
    }
    query.push_str(")");

    query
}

fn get_table_details() -> io::Result<TableDetails> {
    let tables_json = include_str!("../resources/tables.json");
    let details: TableDetails = serde_json::from_str(tables_json)?;

    Ok(details)
}
