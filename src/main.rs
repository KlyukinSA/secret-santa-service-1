// # Веб-сервис секретного Санты.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tide::Request;
use serde_json::{Value, json};

#[derive(PartialEq)]
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

fn get_not_used_in_map_id<T>(map: &HashMap<Id, T>) -> Id
{
    *map.keys().max().unwrap() + 1
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
                let state = request.state();
                let guard = state.lock().unwrap();
                Ok(json!(guard.users))
            });
        app.at("/groups")
            .get(|request: Request<Arc<Mutex<DataBase>>>| async move {
                let state = request.state();
                let guard = state.lock().unwrap();
                Ok(json!(guard.groups))
            });
        app.at("/user/create")
            .post(|mut request: Request<Arc<Mutex<DataBase>>>| async move {
                let body: Value = request.body_json().await?;
                let object = body.as_object().unwrap();
                let name = object.get("name").unwrap().as_str().unwrap();
                if name.len() > 0
                {
                    let mut guard = request.state().lock().unwrap();
                    let id = get_not_used_in_map_id(&guard.users);
                    guard.users.insert(id, name.to_string());

                    Ok(tide::Response::builder(200)
                        .body(tide::Body::from_json(&json!({"id": id}))?)
                        .build())
                }
                else
                {
                    Ok(tide::Response::builder(400)
                        .body(tide::Body::from_json(&json!({"error": "bad name"}))?)
                        .build())
                }
            });
        app.at("/group/create")
            .post(|mut request: Request<Arc<Mutex<DataBase>>>| async move {
                let body: Value = request.body_json().await?;
                let object = body.as_object().unwrap();
                let creator_id :u32 = object.get("creator_id").unwrap().as_str().unwrap().parse().unwrap();
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
        app.at("/group/make_admin")
            .post(|mut request: Request<Arc<Mutex<DataBase>>>| async move {
                let body: Value = request.body_json().await?;
                let object = body.as_object().unwrap();
                let group_id = object.get("group_id").unwrap().as_str().unwrap().parse::<u32>().unwrap();
                let member_id = object.get("member_id").unwrap().as_str().unwrap().parse::<u32>().unwrap();
                let admin_id = object.get("admin_id").unwrap().as_str().unwrap().parse::<u32>().unwrap();

                let mut guard = request.state().lock().unwrap();
                if guard.user_groups.contains_key(
                    &UserGroupId {
                        user_id: member_id,
                        group_id,
                    }
                )
                && guard.user_groups.get(
                    &UserGroupId {
                        user_id: admin_id,
                        group_id,
                    }
                ).unwrap().access_level == Access::Admin {
                    guard.user_groups.insert(
                        UserGroupId {
                            user_id: member_id,
                            group_id,
                        },
                        UserGroupProps {
                            access_level: Access::Admin,
                            santa_id: 0,
                        }
                    );
                    Ok(tide::Response::builder(200)
                        .body(tide::Body::from_json(&json!({guard.users.get(&member_id).unwrap(): "is admin now"}))?)
                        .build())
                }
                else {
                    Ok(tide::Response::builder(400)
                        .body(tide::Body::from_json(&json!({"error": "bad id or group"}))?)
                        .build())
                }
            });
        app.listen("127.0.0.1:8080").await
    };
    
    futures::executor::block_on(f)
}
