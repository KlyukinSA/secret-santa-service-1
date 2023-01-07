// # Веб-сервис секретного Санты.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tide::{Request, Response};
use serde_json::{Value, json, Map};

#[derive(PartialEq,Eq, Clone)]
enum Access
{
    User,
    Admin,
}

type Id = u32;

#[derive(Eq, Hash, PartialEq, Clone, serde::Serialize)]
struct UserGroupId
{
    user_id: Id,
    group_id: Id,
}
#[derive(Clone)]
struct UserGroupProps
{
    access_level: Access,
    santa_id: Id,
}
impl UserGroupProps {
    fn new(access_level: Access) -> UserGroupProps {
        UserGroupProps {
            access_level,
            santa_id: 0,
        }
    }
}

struct DataBase
{
    users: HashMap<Id, String>,
    users_max_id: Id,
    groups: HashMap<Id, bool>,
    groups_max_id: Id,
    user_groups: HashMap<UserGroupId, UserGroupProps>,
}

fn get_field<T>(object: &serde_json::Map<String, Value>, key: &str) -> T
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    object.get(key).unwrap().as_str().unwrap().parse().unwrap()
}

fn response_data(value: Value) -> Response
{
    Response::builder(200)
        .body(tide::Body::from_json(&value).unwrap())
        .build()
}

fn response_empty() -> Response
{
    Response::builder(200).build()
}

fn response_error(msg: &str) -> Response
{
    Response::builder(400)
        .body(tide::Body::from_json(&json!({"error": msg})).unwrap())
        .build()
}




fn user_create(input_obj: &Map<String, Value>, state: &Arc<Mutex<DataBase>>) -> Response
{
    let name: String = get_field(input_obj, "name");
    if name.len() > 0
    {
        let mut guard = state.lock().unwrap();
        let id = guard.users_max_id;
        guard.users.insert(id, name);
        guard.users_max_id += 1;

        response_data(json!({"id": id}))
    }
    else
    {
        response_error("bad name")
    }
}

fn does_user_belong_to_group(user_id: Id, group_id: Id, user_groups: &HashMap<UserGroupId,UserGroupProps>) -> bool
{
    return user_groups.contains_key(&UserGroupId { user_id, group_id });
}

fn count_admins(group_id: Id, user_groups: &HashMap<UserGroupId, UserGroupProps>) ->usize
{
    let iter = user_groups.into_iter();
    let collection = iter.filter(|&x| x.0.group_id == group_id && x.1.access_level == Access::Admin);
    return collection.count();
}
fn is_admin(user_id: Id, group_id: Id, map: &HashMap<UserGroupId, UserGroupProps>) -> bool
{
    map.get(
        &UserGroupId {
            user_id,
            group_id,
        }
    ).unwrap().access_level == Access::Admin
}

fn get_secret_santas(group: &Vec<Id>) -> HashMap<Id, Id>
{
    let mut secret_santas = HashMap::new();
    let mut is_first = true;
    let mut prev: Id = 0;
    let mut first: Id = 0;
    let mut last :Id = 0;
    for user_id in group{
        if is_first{
            is_first = false;
            prev = *user_id;
            first = *user_id;
            continue;
        }
        secret_santas.insert(*user_id, prev);
        prev = *user_id;
        last = *user_id;
    }
    secret_santas.insert(first, last);
    secret_santas
}

fn main() -> Result<(), std::io::Error> 
{
    let f = async {
        let data = DataBase
        {
            users: HashMap::new(),
            users_max_id: 0,
            groups: HashMap::new(),
            groups_max_id: 0,
            user_groups: HashMap::new(),
        };
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
                let input_obj = body.as_object().unwrap();
                Ok(user_create(input_obj, request.state()))
            });
        app.at("/group/create")
            .post(|mut request: Request<Arc<Mutex<DataBase>>>| async move {
                let body: Value = request.body_json().await?;
                let object = body.as_object().unwrap();
                let creator_id: Id = get_field(object, "creator_id");

                let mut guard = request.state().lock().unwrap();
                Ok(if !guard.users.contains_key(&creator_id)
                {
                    response_error("no such user")
                }
                else
                {
                    let id = guard.groups_max_id;
                    guard.groups.insert(id, false);
                    guard.groups_max_id += 1;
                    guard.user_groups.insert(
                        UserGroupId
                        {
                            user_id: creator_id,
                            group_id: id,
                        },
                        UserGroupProps::new(Access::Admin)
                    );
                    response_data(json!({"group_id": id}))
                })
            });
        app.at("/group/join")
            .post(|mut request: Request<Arc<Mutex<DataBase>>>| async move {
                let value: Value = request.body_json().await.unwrap();
                let object = value.as_object().unwrap();
                let user_id = get_field(object, "user_id");
                let group_id = get_field(object, "group_id");

                let mut guard = request.state().lock().unwrap();
                Ok(match guard.groups.get(&group_id)
                {
                    None => response_error("no such group"),
                    Some(is_closed) =>
                    {
                        if *is_closed
                        {
                            response_error("group is closed")
                        }
                        else
                        {
                            if !guard.users.contains_key(&user_id)
                            {
                                response_error("no such user")
                            }
                            else
                            {
                                let user_group_id = UserGroupId{user_id, group_id};
                                if guard.user_groups.contains_key(&user_group_id)
                                {
                                    response_error("user already in group")
                                }
                                else
                                {
                                    guard.user_groups.insert(user_group_id, UserGroupProps::new(Access::User));
                                    response_empty()
                                }
                            }
                        }
                    },
                })
            });
        app.at("/group/unadmin")
            .post(|mut request: Request<Arc<Mutex<DataBase>>>| async move {
                let body: Value = request.body_json().await?;
                let object = body.as_object().unwrap();
                let admin_id = get_field(object, "admin_id");
                let group_id = get_field(object, "group_id");

                let mut guard = request.state().lock().unwrap();
                let user_group_id = UserGroupId{user_id: admin_id, group_id};
                Ok(match guard.user_groups.get(&user_group_id)
                {
                    None => response_error("user does not belong to this group"),
                    Some(user_group_props) =>
                    {
                        if user_group_props.access_level != Access::Admin
                        {
                            response_error("This user is not an admin.")
                        }
                        else
                        {
                            if count_admins(group_id, &guard.user_groups) < 2
                            {
                                response_error("It is impossible to remove the last admin in a group. You can appoint a new admin and repeat or delete the whole group.")
                            }
                            else
                            {
                                guard.user_groups.get_mut(&user_group_id).unwrap().access_level = Access::User;
                                response_empty()
                            }
                        }
                    }
                })
            });
        app.at("/group/delete")
            .post(|mut request: Request<Arc<Mutex<DataBase>>>| async move {
                let body: Value = request.body_json().await?;
                let object = body.as_object().unwrap();
                let admin_id = get_field(object, "admin_id");
                let group_id = get_field(object, "group_id");

                let mut guard = request.state().lock().unwrap();
                Ok(match guard.user_groups.get(&UserGroupId{user_id: admin_id, group_id})
                {
                    None => response_error("user does not belong to this group"),
                    Some(user_group_props) =>
                    {
                        if user_group_props.access_level != Access::Admin
                        {
                            response_error("This user is not an admin.")
                        }
                        else
                        {
                            // Before delete group, we need to delete all users from this group
                            guard.user_groups.retain(|user_group_id, _|
                                {
                                    user_group_id.group_id != group_id
                                });
                            guard.groups.remove(&group_id);
                            response_empty()
                        }
                    }
                }
            )});
        app.at("/group/make_admin")
            .post(|mut request: Request<Arc<Mutex<DataBase>>>| async move {
                let body: Value = request.body_json().await?;
                let object = body.as_object().unwrap();
                let group_id: Id = get_field(object, "group_id");
                let member_id: Id = get_field(object, "member_id");
                let admin_id: Id = get_field(object, "admin_id");

                let mut guard = request.state().lock().unwrap();
                Ok(if !guard.groups.contains_key(&group_id)
                {
                    response_error("no such group")
                }
                else if !does_user_belong_to_group(member_id, group_id, &guard.user_groups)
                {
                    response_error("user isn't a member of the group")
                }
                else if is_admin(member_id, group_id, &guard.user_groups)
                {
                    response_error("user is already an admin")
                }
                else if !is_admin(admin_id, group_id, &guard.user_groups)
                {
                    response_error("admin_id isn't an actual admin's ID")
                }
                else {
                    guard.user_groups.insert(
                        UserGroupId {
                            user_id: member_id,
                            group_id,
                        },
                        UserGroupProps::new(Access::Admin),
                    );
                    response_empty()
                }
            )});
        app.at("/group/quit")
            .post(|mut request: Request<Arc<Mutex<DataBase>>>| async move {
                let body: Value = request.body_json().await?;
                let object = body.as_object().unwrap();
                let group_id: Id = get_field(object, "group_id");
                let user_id: Id = get_field(object, "user_id");

                let mut guard = request.state().lock().unwrap();
                let user_group_id = UserGroupId{user_id, group_id};
                Ok(match guard.user_groups.get(&user_group_id)
                {
                    None => response_error("user does not belong to this group"),
                    Some(user_group_props) =>
                    {
                        if user_group_props.access_level == Access::Admin && count_admins(group_id, &guard.user_groups) < 2
                        {
                            response_error("user is only one Admin in this group")
                        }
                        else
                        {
                            if *guard.groups.get(&group_id).unwrap()
                            {
                                response_error("group is closed")
                            }
                            else
                            {
                                guard.user_groups.remove(&user_group_id);
                                response_empty()
                            }
                        }
                    }
                })
            });
        app.at("/group/target_by_id/:user_id/:group_id")
            .get(|request: Request<Arc<Mutex<DataBase>>>| async move{
                let first_id = request.param("user_id")?;
                let second_id = request.param("group_id")?;
                for c in first_id.chars() {
                    if !c.is_numeric() {
                        return Ok(response_error("Wrong format user id"));
                    } 
                }
                for c in second_id.chars() {
                    if !c.is_numeric() {
                        return Ok(response_error("Wrong format group id"));
                    } 
                }
                let user_id: Id = first_id.parse().unwrap(); // TODO
                let group_id: Id = second_id.parse().unwrap();

                let guard = request.state().lock().unwrap();
                Ok(match guard.user_groups.get(&UserGroupId{user_id, group_id})
                {
                    None => response_error("user does not belong to this group"),
                    Some(user_group_props) =>
                    {
                        response_data(json!({"cysh_for_id": user_group_props.santa_id}))
                    }
                })
            });
        app.at("/group/secret_santa")
            .post(|mut request: Request<Arc<Mutex<DataBase>>>| async move {
                let body: Value = request.body_json().await?;
                let object = body.as_object().unwrap();
                let group_id: Id = get_field(object, "group_id");
                let admin_id: Id = get_field(object, "admin_id");

                let mut guard = request.state().lock().unwrap();
                Ok(match guard.user_groups.get(&UserGroupId{user_id: admin_id, group_id})
                {
                    None => response_error("user does not belong to this group"),
                    Some(user_group_props) =>
                        {
                            if user_group_props.access_level != Access::Admin
                            {
                                response_error("its not admin")
                            }
                            else
                            {
                                *guard.groups.get_mut(&(group_id)).unwrap() = true;

                                //Пользователю присваивается тайный кыш бабай, предыдущий в списке группы.
                                //Если пользователь первый, то ему присвается кыш бабай последний пользователь группы


                                let group: Vec<Id> = guard.user_groups.keys().filter_map(|key|
                                    match key.group_id == group_id
                                    {
                                        true => Some(key.user_id),
                                        false => None,
                                    }
                                ).collect();
                                let santas: HashMap<Id, Id> = get_secret_santas(&group);
                                for user_id in group
                                {
                                    guard.user_groups.get_mut(&UserGroupId{user_id, group_id}).unwrap().santa_id = *santas.get(&user_id).unwrap();
                                }
                                response_empty()
                            }
                        }
                })
            });
            app.at("/user/update")
            .put(|mut request: Request<Arc<Mutex<DataBase>>>| async move{
                let body: Value = request.body_json().await?;
                let object = body.as_object().unwrap();
                let id : Id = get_field(object, "id");
                let name: String = get_field(object, "name");
                let mut guard = request.state().lock().unwrap();
                if !guard.users.contains_key(&id)
                {
                    return Ok(response_error("No such id"));
                }
                else
                {
                    guard.users.entry(id).and_modify(|k| *k = name);
                    return Ok(response_empty());
                }
            });


            app.at("/user/delete")
            .delete(|mut request: Request<Arc<Mutex<DataBase>>>| async move {
                let body: Value = request.body_json().await?;
                let object = body.as_object().unwrap();
                let user_id = get_field(object, "user_id");
                let mut guard = request.state().lock().unwrap();
                Ok(match guard.users.get(&user_id)
                {
                    None => response_error("This user does not exist."),
                    Some(_name) =>
                    {
                        if guard.user_groups.len() > 0
                        {
                            let iter1 = guard.user_groups.iter();
                            let iter2 = guard.user_groups.iter();
                            let collection = iter1.filter(|&x| x.0.user_id == user_id);
                            let collect_copy = iter2.filter(|&x| x.0.user_id == user_id);
                            let closed_collect = collection.filter(|&x| guard.groups.get(&x.0.group_id).unwrap() == &true);
                            let free_collect = collect_copy.filter(|&x| guard.groups.get(&x.0.group_id).unwrap() == &false);
                            let mut admin_flag = false;
                            let mut vec:Vec<Id> = Vec::new();
                            let mut delete_vec=Vec::new();
                            for x in free_collect
                            {
                                if x.1.access_level == Access::Admin && count_admins(x.0.group_id, &guard.user_groups) == 1
                                {
                                    admin_flag=true;
                                    vec.push(x.0.group_id);
                                }
                                else 
                                {
                                    delete_vec.push(UserGroupId{user_id: user_id, group_id: x.0.group_id});
                                }
                            }   
                            if closed_collect.count() > 0
                            {
                                for x in delete_vec
                                {
                                    guard.user_groups.remove(&x);
                                }
                                if admin_flag == true
                                {
                                    let mut string: String="User has closed groups. So he was deleted from opened groups, if he wasn't last admin. User cannot be delete from groups: ".to_string();
                                    for x in vec
                                    {
                                            string+=format!("{0}, ", x).as_str();
                                    }
                                    string+="because of last admin.";
                                    response_error(string.as_str())
                                }
                                else
                                {
                                    response_error("User has closed groups. So he was deleted from opened groups.")
                                }
                            }
                            else 
                            {
                                for x in delete_vec
                                {
                                    guard.user_groups.remove(&x);
                                }
                                if admin_flag == false
                                {
                                    guard.users.remove(&user_id);
                                    response_empty()
                                }
                                else 
                                {
                                    let mut string: String="User cannot be delete from groups: ".to_string();
                                    for x in vec
                                    {
                                        string+=format!("{0}, ", x).as_str();
                                    }
                                    string+="because he is the last admin in these groups.";
                                    response_error(string.as_str())
                                }
                            }
                        }
                        else
                        {
                            guard.users.remove(&user_id);
                            response_empty()
                        }
                    }
                })
            });
        app.listen("127.0.0.1:8080").await
    };
    futures::executor::block_on(f)
}