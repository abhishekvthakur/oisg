use std::io;

use crate::db;

pub fn save_user_details(
    user_name: String,
    user_id: String
) -> io::Result<()> {
    let query = format!(
        "INSERT INTO USER_INFO (USERNAME, USER_NAME) VALUES ('{}', '{}')",
        user_name, user_id
    );

    let connection = db::get_connection()?;
    return match connection.execute(query) {
        Ok(_) => Ok(()),
        Err(e) => Err(io::Error::new(io::ErrorKind::Other, e.message.unwrap()))
    }
}