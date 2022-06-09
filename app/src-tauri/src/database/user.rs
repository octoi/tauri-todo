use bcrypt::{hash, DEFAULT_COST};
use rusqlite::Connection;

pub struct User {
    pub id: i32,
    pub email: String,
    password: String,
}

pub fn sign_up(db: &Connection, email: String, password: String) -> Result<(), &'static str> {
    let hashed = hash_string(password)?;

    match db.execute(
        "INSERT INTO user (email, password) VALUES (?1, ?2)",
        &[&email, &hashed],
    ) {
        Ok(_) => return Ok(()),
        Err(_) => {
            eprintln!("[-] Failed to insert data to database");
            return Err("Failed to insert data to database");
        }
    };
}

pub fn login(db: &Connection, email: String, password: String) -> Result<User, &'static str> {
    let hashed = hash_string(password)?;

    let user = match get_user(db, email.clone()) {
        Ok(user) => user,
        Err(err) => {
            return Err(err);
        }
    };

    if user.password == hashed {
        Ok(user)
    } else {
        Err("Incorrect password")
    }
}

pub fn get_user(db: &Connection, email: String) -> Result<User, &'static str> {
    let mut sql_query = match db.prepare("SELECT * FROM user WHERE email=(?1)") {
        Ok(statement) => statement,
        Err(_) => {
            return Err("Failed to get user");
        }
    };

    let user_iter = match sql_query.query_map(&[&email], |row| {
        Ok(User {
            id: row.get(0)?,
            email: row.get(1)?,
            password: row.get(2)?,
        })
    }) {
        Ok(user_iter) => user_iter,
        Err(_) => return Err("Failed to find user"),
    };

    for user_result in user_iter {
        match user_result {
            Ok(user) => return Ok(user),
            Err(_) => return Err("Failed to find user"),
        }
    }

    Err("User not found")
}

// utils
fn hash_string(string: String) -> Result<String, &'static str> {
    match hash(string, DEFAULT_COST) {
        Ok(string_hash) => return Ok(string_hash),
        Err(_) => {
            eprintln!("[-] Failed to hash password");
            return Err("Failed to hash password");
        }
    };
}