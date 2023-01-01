use std::{
    collections::HashMap,
    fs::File,
    sync::{Arc, Mutex},
};

use tide::Request;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
enum Access {
    Guest,
    User,
    Admin,
}

#[derive(serde::Deserialize)]
struct User {
    name: String,
    access: Access,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct DataBase {
    users: HashMap<String, Access>,
}

fn main() -> Result<(), std::io::Error> {
    let f = async {
        let version: &'static str = env!("CARGO_PKG_VERSION");

        let database = match File::open("data.base") {
            Ok(file) => serde_json::from_reader(file).map_err(|err| {
                let err = std::io::Error::from(err);
                std::io::Error::new(
                    err.kind(),
                    format!("Failed to read from database file. {err}"),
                )
            })?,
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
                eprintln!("Database file not found. Creating one");

                let file = File::create("data.base").map_err(|err| {
                    std::io::Error::new(
                        err.kind(),
                        format!("Failed to create database file. {err}"),
                    )
                })?;
                let database = DataBase {
                    users: HashMap::new(),
                };
                serde_json::to_writer(file, &database).map_err(|err| {
                    let err = std::io::Error::from(err);
                    std::io::Error::new(
                        err.kind(),
                        format!("Failed to write to database file. {err}"),
                    )
                })?;

                database
            }
            Err(err) => {
                panic!("Failed to open database file. {err}");
            }
        };

        let state = Arc::new(Mutex::new(database));

        let mut app = tide::with_state(state);
        app.at("/version")
            .get(move |_| async move { Ok(serde_json::json!({ "version": version })) });

        app.at("/add-user")
            .put(|mut request: Request<Arc<Mutex<DataBase>>>| async move {
                let User { name, access } = request.body_json().await?;

                eprintln!("Adding user {name} with {access:?}");

                let state = request.state();
                let mut guard = state.lock().unwrap();

                guard.users.insert(name, access);

                Ok(tide::StatusCode::Ok)
            });

        app.at("/get-user")
            .get(|mut request: Request<Arc<Mutex<DataBase>>>| async move {
                let name: String = request.body_json().await?;

                let state = request.state();
                let guard = state.lock().unwrap();

                eprintln!("Searching for user {name}");

                match guard.users.get(&name) {
                    None => Err(tide::Error::from_str(
                        tide::StatusCode::NotFound,
                        format!("User {name} not found"),
                    )),
                    Some(access) => Ok(serde_json::json!({ "access": access })),
                }
            });

        app.listen("127.0.0.1:8080").await
    };

    futures::executor::block_on(f)
}
