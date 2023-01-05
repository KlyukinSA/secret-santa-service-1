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
                    Ok(value) => tide::Response::builder(200)
                        .body(tide::Body::from_json(&value)?)
                        .build(),
                    Err(msg) => tide::Response::builder(400)
                        .body(tide::Body::from_json(&json!({"error": msg}))?)
                        .build(),
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
        app.at("/group/delete")
            .post(|mut request: Request<Arc<Mutex<DataBase>>>| async move {
                let body: Value = request.body_json().await?;
                let object = body.as_object().unwrap();

                let group_id: Id = get_field(object, "group_id");
                let admin_id: Id = get_field(object, "admin_id");

                let mut guard = request.state().lock().unwrap();

                if guard.groups.contains_key(&group_id)
                {
                    // Check if admin_id specified is admin of this group
                    let user_group = guard.user_groups.get(
                        &UserGroupId
                        {
                            user_id: admin_id,
                            group_id: group_id,
                        }
                    );
                    if user_group.is_none()
                    {
                        return Ok(tide::Response::builder(403)
                            .body(tide::Body::from_json(&json!({"error": "Forbidden"}))?)
                            .build());
                    }
                    // If access level is not admin, return error
                    if !matches!(user_group.unwrap().access_level, Access::Admin)
                    {
                        return Ok(tide::Response::builder(403)
                            .body(tide::Body::from_json(&json!({"error": "Forbidden"}))?)
                            .build());
                    }

                    // Before delete group, we need to delete all users from this group
                    let mut users_to_delete = Vec::new();
                    for (user_group_id, _) in guard.user_groups.iter()
                    {
                        if user_group_id.group_id == group_id
                        {
                            users_to_delete.push(user_group_id.user_id);
                        }
                    }

                    guard.user_groups.retain(|user_group_id, _| {
                        user_group_id.group_id != group_id
                    });

                    guard.groups.remove(&group_id);
                    Ok(tide::Response::builder(200).build())
                }
                else
                {
                    Ok(tide::Response::builder(400)
                        .body(tide::Body::from_json(&json!({"error": "bad group_id"}))?)
                        .build())
                }
            });
        app.listen("127.0.0.1:8080").await
    };
    futures::executor::block_on(f)
}
