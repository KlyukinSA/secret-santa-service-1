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

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use tide::Request;

enum Access
{
    User,
    Admin,
}

struct UserGroup
{
    user_id: u32,
    santa_id: u32,
    group_id: u32,
    access_level: Access,
}

struct DataBase
{
    users: HashMap<u32, String>,
    groups: HashMap<u32, bool>,
    user_groups: HashSet<UserGroup>,
}

fn main() -> Result<(), std::io::Error> 
{
    let f = async {
        let mut data = DataBase
        {
            users: HashMap::new(),
            groups: HashMap::new(),
            user_groups: HashSet::new(),
        };
        data.users.insert(0, "Ilya".to_string());
        data.users.insert(1, "Stepan".to_string());
        data.groups.insert(0, true);
        data.groups.insert(1, false);
        let state = Arc::new(Mutex::new(data));

        let mut app = tide::with_state(state);

        app.at("/users")
            .get(|mut request: Request<Arc<Mutex<DataBase>>>| async move {
                let state = request.state();
                let guard = state.lock().unwrap();
                Ok(serde_json::json!(guard.users))
            });
        app.at("/groups")
            .get(|mut request: Request<Arc<Mutex<DataBase>>>| async move {
                let state = request.state();
                let guard = state.lock().unwrap();
                Ok(serde_json::json!(guard.groups))
            });

        app.listen("127.0.0.1:8080").await
    };
    futures::executor::block_on(f)
}
