use dotenv::dotenv;
use mysql::prelude::*;
use mysql::*;

#[derive(Debug, PartialEq, Eq)]
struct User {
    user_id: i32,
    first_name: Option<String>,
    alive: u8,
    salary: Option<String>,
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    dotenv().ok(); // this line loads the environment variables from the ".env" file
                   // now let's read the APP key

    let db_uri = get_env("DB_URI").to_owned();

    // here, &db_uri.to_owned()[..] is to turn String to &str
    let pool = match Pool::new(&db_uri[..]) {
        Ok(p) => p,
        Err(e) => {
            panic!("failed to create a db pool because {:?}", e);
        }
    };

    let mut conn = pool.get_conn()?;

    // create a user
    conn.exec_drop(
        r"INSERT INTO user (first_name, alive, salary) VALUES (?,?,?)",
        ("James", 1, 76.54),
    )?;

    let selected_users = conn.query_map(
        r"SELECT user_id, first_name, alive, salary FROM user",
        |(user_id, first_name, alive, salary)| User {
            user_id,
            first_name,
            alive,
            salary,
        },
    )?;

    println!("{:?}", selected_users);

    Ok(())
}

fn get_env(key: &str) -> String {
    let value = match std::env::var(key) {
        Ok(k) => k,
        Err(_) => {
            panic!("Key {} was not configured!", key);
        }
    };
    value
}
