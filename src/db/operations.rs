use std::io;

use crate::db::{
    self,
    models
};

pub fn get_user_info<'a>() -> io::Result<Option<models::UserInfo>> {
    let connection = db::get_connection()?;
    let query = "SELECT USER_NAME, USER_ID, JOINED_AT FROM USER_INFO LIMIT 1";

    let mut res = models::UserInfo::new();
    let _ = connection.iterate(query, |pairs| {
        for &(col, val) in pairs.iter() {
            res.insert(col.to_string(), val.unwrap().to_string());
        }

        false
    });


    if res.len() > 0 {
        Ok(Some(res))
    } else {
        Ok(None)
    }
}

pub fn save_user_details(
    user_name: String,
    user_id: String
) -> io::Result<()> {
    let query = format!(
        "INSERT INTO USER_INFO (USER_NAME, USER_ID) VALUES ('{}', '{}')",
        user_name, user_id
    );

    let connection = db::get_connection()?;
    return match connection.execute(query) {
        Ok(_) => Ok(()),
        Err(e) => Err(io::Error::new(io::ErrorKind::Other, e.message.unwrap()))
    }
}