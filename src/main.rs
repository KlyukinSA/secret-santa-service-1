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

enum Access
{
    User,
    Admin,
}

struct UserGroup
{
    user_id: u32,
    santa_id: u32 = -1,
    group_id: u32,
    access_level: Access,
}

fn main()
{
    let f = async {
        let users: HashMap<u32, String> = HashMap::new();
        let groups: HashMap<u32, bool> = HashMap::new();
        let user_groups: HashSet<UserGroup> = HashSet::new();

        let state = Arc::new(Mutex::new((users, groups, user_groups)));

        let mut app = tide::with_state(state);

        app.at("/users")
            .get(|mut request: Request<Arc<Mutex<Tuple>>>| async move {
                let state = request.state();
                let guard = state.lock().unwrap();
                Ok(serde_json::json!(guard.0.all_as_jsonarray)
            });
        app.at("/groups")
            .get(|mut request: Request<Arc<Mutex<Tuple>>>| async move {
                let state = request.state();
                let guard = state.lock().unwrap();
                Ok(serde_json::json!(guard.1.all_as_jsonarray)
            });

        app.listen("127.0.0.1:8080").await
    }
    futures::executor::block_on(f)
}
