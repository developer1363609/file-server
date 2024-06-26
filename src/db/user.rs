use std::{
    env,
    fs::{self,OpenOptions},
    path::PathBuf,
};

use rusqlite::params;
use tokio_rusqlite::Connection;
use crate::db::user;


pub(self) const DB_CREATE_TABLE: &str = "CREATE TABLE IF NOT EXISTS user (
    id                INTEGER PRIMARY KEY,
    username          TEXT,
    password          TEXT,
    salt              TEXT,
    create_time       INTEGER,
    update_time       INTEGER,
    delete_time       INTEGER
)";

pub(crate) struct User {
    #[allow(unused)]
    pub(crate) id: i64,
    pub(crate) username: String,
    pub(crate) password: String,
    pub(crate) salt: String,
    pub(crate) create_time: i64,
    pub(crate) update_time: i64,
    pub(crate) delete_time: i64,
}

pub(crate) struct UserDB {
    conn: Connection,
}

impl UserDB {
    pub(crate) async fn new() -> anyhow::Result<Self> {
        let path = if cfg!(unix) || cfg!(macos) {
            let home = env::var("HOME").expect("HOME not set");
            format!("{}/file-server/user.sqlite", home)
        } else if cfg!(windows) {
            "C:\\Program Files\\file-server\\user.db".to_owned()
        } else {
            panic!("unsupported platform");
        };
        let path = PathBuf::from(path);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        };
        OpenOptions::new()
            .create(true)
            .write(true)
            .read(true)
            .open(&path)?;
        let conn = Connection::open(path).await.unwrap();
        conn.call(|conn| {
            let mut stmt = conn.prepare(DB_CREATE_TABLE)?;
            stmt.execute(params![]).unwrap();
            Ok::<(), rusqlite::Error>(())
        })
            .await
            .unwrap();
        Ok(Self { conn })
    }

    pub(crate) async fn insert(&self, user: User) -> anyhow::Result<()> {
        self.conn
            .call(move |conn| {
                let mut stmt = conn
                    .prepare("INSERT INTO user (username, password, salt, create_time, update_time, delete_time) VALUES (?1, ?2, ?3, ?4, ?5, ?6)")
                    .unwrap();
                stmt.execute(params![
                    user.username,
                    user.password,
                    user.salt,
                    user.create_time,
                    user.update_time,
                    user.delete_time
                ])
                    .unwrap();
                Ok::<(), rusqlite::Error>(())
            })
            .await
            .unwrap();
        Ok(())
    }

    pub(crate) async fn get(&self, username: String) -> anyhow::Result<Option<User>> {
        let res = self.conn.call(move |conn| {
            let mut statement = conn.prepare("SELECT id, username, password, salt, create_time, update_time, delete_time FROM user WHERE username = ?1")?;
            let mut res = statement
                .query_map(params![username], |row| {
                    let metadata = User {
                        id: row.get(0)?,
                        username: row.get(1)?,
                        password: row.get(2)?,
                        salt: row.get(3)?,
                        create_time: row.get(4)?,
                        update_time: row.get(5)?,
                        delete_time: row.get(6)?,
                    };
                    Ok::<User, rusqlite::Error>(metadata)
                })?
                .collect::<Result<Vec<User>, rusqlite::Error>>()?;
            if res.len() == 0 {
                Ok::<Option<User>, rusqlite::Error>(None)
            } else {
                Ok::<Option<User>, rusqlite::Error>(Some(res.remove(0)))
            }
        }).await?;
        Ok(res)
    }
}
