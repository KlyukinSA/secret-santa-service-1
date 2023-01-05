// # Веб-сервис секретного Санты.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tide::Request;
use serde_json::{Value, json, Map};

enum Access
{
    User,
    Admin,
}

type Id = u32;

#[derive(Eq, Hash, PartialEq)]
struct UserGroupId
{
    user_id: Id,
    group_id: Id,
}
struct UserGroupProps
{
    access_level: Access,
    santa_id: Id,
}

struct DataBase
{
    users: HashMap<Id, String>,
    groups: HashMap<Id, bool>,
    user_groups: HashMap<UserGroupId, UserGroupProps>,
}

fn get_field<T>(object: &serde_json::Map<String, Value>, key: &str) -> T
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    object.get(key).unwrap().as_str().unwrap().parse().unwrap()
}

fn get_not_used_in_map_id<T>(map: &HashMap<Id, T>) -> Id
{
    *map.keys().max().unwrap() + 1
}

fn response_data(value: Value) -> tide::Response
{
    tide::Response::builder(200)
        .body(tide::Body::from_json(&value).unwrap())
        .build()
}

fn response_empty() -> tide::Response
{
    tide::Response::builder(200).build()
}

fn response_error(msg: String) -> tide::Response
{
    tide::Response::builder(400)
        .body(tide::Body::from_json(&json!({"error": msg})).unwrap())
        .build()
}




fn user_create(input_obj: &Map<String, Value>, state: &Arc<Mutex<DataBase>>) -> Result<Value, String>
{
    let name: String = get_field(input_obj, "name");
    if name.len() > 0
    {
        let mut guard = state.lock().unwrap();
        let id = get_not_used_in_map_id(&guard.users);
        guard.users.insert(id, name);

        Ok(json!({"id": id}))
    }
    else
    {
        Err("bad name".to_string())
    }
}

fn main() -> Result<(), std::io::Error> 
{
    let f = async {
        let mut data = DataBase
        {
            users: HashMap::new(),
            groups: HashMap::new(),
            user_groups: HashMap::new(),
        };
        
        // Mock data (данные для тестирования)
        data.users.insert(0, "Ilya".to_string());
        data.users.insert(2, "Stepan".to_string());
        data.groups.insert(0, false);
        data.groups.insert(1, false);
        data.user_groups.insert(
            UserGroupId
            {
                user_id: 0,
                group_id: 0,
            },
            UserGroupProps
            {
                access_level: Access::Admin,
                santa_id: 0,
            }
        );
        data.user_groups.insert(
            UserGroupId
            {
                user_id: 2,
                group_id: 1,
            },
            UserGroupProps
            {
                access_level: Access::Admin,
                santa_id: 0,
            }
        );
     
        let state = Arc::new(Mutex::new(data));
        let mut app = tide::with_state(state);

        // Routes
        app.at("/users")
            .get(|request: Request<Arc<Mutex<DataBase>>>| async move {
                let guard = request.state().lock().unwrap();
                Ok(json!(guard.users))
            });
        app.at("/groups")
            .get(|request: Request<Arc<Mutex<DataBase>>>| async move {
                let guard = request.state().lock().unwrap();
                Ok(json!(guard.groups))
            });
        app.at("/user/create")
            .post(|mut request: Request<Arc<Mutex<DataBase>>>| async move {
                let body: Value = request.body_json().await?;
                let object = body.as_object().unwrap();
                Ok(match user_create(object, request.state())
                {
                    Ok(value) => response_data(value),
                    Err(msg) => response_error(msg),
                })
            });
        app.at("/group/create")
            .post(|mut request: Request<Arc<Mutex<DataBase>>>| async move {
                let body: Value = request.body_json().await?;
                let object = body.as_object().unwrap();

                let creator_id: Id = get_field(object, "creator_id");
                let mut guard = request.state().lock().unwrap();
                if guard.users.contains_key(&creator_id)
                {
                    let id = get_not_used_in_map_id(&guard.groups);
                    guard.groups.insert(id, false);
                    guard.user_groups.insert(
                        UserGroupId
                        {
                            user_id: creator_id,
                            group_id: id,
                        },
                        UserGroupProps
                        {
                            access_level: Access::Admin,
                            santa_id: 0,
                        }
                    );
                    Ok(tide::Response::builder(200)
                        .body(tide::Body::from_json(&json!({"group_id": id}))?)
                        .build())
                }
                else
                {
                    Ok(tide::Response::builder(400)
                        .body(tide::Body::from_json(&json!({"error": "bad creator_id"}))?)
                        .build())
                }
            });
        app.at("/group/join")
            .post(|mut request: Request<Arc<Mutex<DataBase>>>| async move {
                let value: Value = request.body_json().await.unwrap();
                let object = value.as_object().unwrap();
                let mut user_group_id = UserGroupId{user_id: 0, group_id: 0};
                user_group_id.user_id = get_field(object, "user_id");
                user_group_id.group_id = get_field(object, "group_id");

                let mut guard = request.state().lock().unwrap();
                Ok(match guard.groups.get(&user_group_id.group_id)
                {
                    None => tide::Response::builder(400)
                        .body(tide::Body::from_json(&json!({"error": "no such group"}))?)
                        .build(),
                    Some(is_closed) => 
                    {
                        if *is_closed
                        {
                            tide::Response::builder(400)
                                .body(tide::Body::from_json(&json!({"error": "group is closed"}))?)
                                .build()
                        }
                        else
                        {
                            if !guard.users.contains_key(&user_group_id.user_id)
                            {
                                tide::Response::builder(400)
                                    .body(tide::Body::from_json(&json!({"error": "no such user"}))?)
                                    .build()
                            }
                            else
                            {
                                if guard.user_groups.contains_key(&user_group_id)
                                {
                                    tide::Response::builder(400)
                                        .body(tide::Body::from_json(&json!({"error": "user already in group"}))?)
                                        .build()
                                }
                                else
                                {
                                    guard.user_groups.insert(user_group_id, UserGroupProps{access_level: Access::User, santa_id: 0});
                                    tide::Response::builder(200).build()
                                }
                            }
                        }
                    }
                })
            });
        
        app.listen("127.0.0.1:8080").await
    };
    
    futures::executor::block_on(f)
}
