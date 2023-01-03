// # Веб-сервис секретного Санты.

// * Пользователи могут создавать группы.
// * Пользователи могут присоединяться к группам.
// * Пользователи могут иметь права администратора в группе.
// * Пользователь создавший группу автоматически становится администратором.
// * Администратор может назначить другого пользователя в группе администратором.
// * Администратор может снять с себя полномочия администратора, если в группе есть хотя бы еще 1 администратор.
// * Администратор может покинуть группу только есть в группе есть хотя бы еще 1 администратор.
// * Администратор может удалить группу.
// * Администратор может дать команду и сервис назначит секретного Санту для каждого члена группы, выбирая из остальных членов группы.
// * Каждый член группы будет назначен секретным Сантой строго одному другому члену группы.
// * После этого группа становится закрытой, в нее нельзя войти или выйти.
// * Пользователи могут запросить, для кого в группе они стали секретным Сантой.
// * Будет плюсом, если сервис будет использовать БД для хранения данных о пользователях, группах и секретных Сантах. Но можно обойтись хранением данных в памяти.
// * Сервис должен работать как HTTP REST с JSON сообщениями.
// * Будет плюсом написать консольную утилиту для общения с сервисом.

// REST API
// * POST /users - создать пользователя
// * GET /users - получить список пользователей
// * GET /users/{id} - получить пользователя по id
// * PUT /users/{id} - обновить пользователя по id
// * DELETE /users/{id} - удалить пользователя по id

// * POST /groups - создать группу
// * GET /groups - получить список групп
// * GET /groups/{id} - получить группу по id
// * PUT /groups/{id} - обновить группу по id
// * DELETE /groups/{id} - удалить группу по id

// * POST /groups/{id}/join - вступить в группу
// * POST /groups/{id}/leave - покинуть группу
// * POST /groups/{id}/admin - назначить администратором
// * POST /groups/{id}/unadmin - снять с себя полномочия администратора
// * POST /groups/{id}/secret-santa - назначить секретного Санту для каждого члена группы
// * GET /groups/{id}/secret-santa - получить секретного Санту для текущего пользователя

// * GET /groups/{id}/members - получить список членов группы
// * GET /groups/{id}/admins - получить список администраторов группы

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tide::Request;
use serde_json::{Value, json};

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
    2 // TODO
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
        data.users.insert(0, "Ilya".to_string());
        data.users.insert(1, "Stepan".to_string());
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
                user_id: 1,
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
        app.at("/users")
            .post(|mut request: Request<Arc<Mutex<DataBase>>>| async move {
                let body: Option<Value> = request.body_json().await.ok();
                match body.and_then(
                    |value| value.as_object().and_then(
                        |object| object.get("name").and_then(
                            |value| value.as_str().and_then(
                                |name| 
                                {
                                    let state = request.state();
                                    let mut guard = state.lock().unwrap();
                                    let id = get_not_used_in_map_id(&guard.users);
                                    guard.users.insert(id, name.to_string());
                                    let mut res = json!({"id": "0"});
                                    res["id"] = json!(id);
                                    Some(res)
                                }))))
                {
                    Some(res) => Ok(res),
                    None => Ok(json!({"error": "cant read name"})),
                }
            });

        app.listen("127.0.0.1:8080").await
    };
    futures::executor::block_on(f)
}
