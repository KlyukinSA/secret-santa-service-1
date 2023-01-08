

// Методы
static METHODS: &'static [&str] = &["GET", "POST", "PUT", "DELETE"];
static GET_COMMANDS: &'static [&str] = &["users", "groups", "group/target_by_id"];
static POST_COMMANDS: &'static [&str] = &["user/create", "group/create", "group/join", "group/unadmin", "group/make_admin", "group/quit", "group/secret_santa"];
static PUT_COMMANDS: &'static [&str] = &["user/update"];
static DELETE_COMMANDS: &'static [&str] = &["user/delete", "group/delete"];

// Файл с адресом сервера
static SERVER_ADDRESS_FILE: &'static str = "address.conf";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = hyper::Client::new();

    // Чтение адреса сервера из файла address.conf в формате http://server:port
    let address = std::fs::read_to_string(SERVER_ADDRESS_FILE)?;
    let address = address.lines().next().unwrap().to_string();
    println!("Server: {}", address);

    let mut iteration = 0;

    loop {
        println!("\nIteration: {}", iteration);
        // Выбрать метод
        let mut method = String::new();
        println!("Choose method:");
        for i in 0..METHODS.len() {
            println!("{} - {}", i, METHODS[i]);
        }
        let mut method_num;
        loop {
            std::io::stdin().read_line(&mut method)?;
            method_num = method.trim().parse::<usize>()?;
            if method_num < METHODS.len() {
                break;
            }
            println!("Wrong method number");
        }

        let mut command = String::new();
        let mut command_num;
        println!("Choose command:");
        match METHODS[method_num] {
            "GET" => {
                for i in 0..GET_COMMANDS.len() {
                    println!("{} - {}", i, GET_COMMANDS[i]);
                }
                loop {
                    std::io::stdin().read_line(&mut command)?;
                    command_num = command.trim().parse::<usize>()?;
                    if command_num < GET_COMMANDS.len() {
                        break;
                    }
                    println!("Wrong command number");
                }
            },
            "POST" => {
                for i in 0..POST_COMMANDS.len() {
                    println!("{} - {}", i, POST_COMMANDS[i]);
                }
                loop {
                    std::io::stdin().read_line(&mut command)?;
                    command_num = command.trim().parse::<usize>()?;
                    if command_num < POST_COMMANDS.len() {
                        break;
                    }
                    println!("Wrong command number");
                }
            },
            "PUT" => {
                for i in 0..PUT_COMMANDS.len() {
                    println!("{} - {}", i, PUT_COMMANDS[i]);
                }
                loop {
                    std::io::stdin().read_line(&mut command)?;
                    command_num = command.trim().parse::<usize>()?;
                    if command_num < PUT_COMMANDS.len() {
                        break;
                    }
                    println!("Wrong command number");
                }
            },
            "DELETE" => {
                for i in 0..DELETE_COMMANDS.len() {
                    println!("{} - {}", i, DELETE_COMMANDS[i]);
                }
                loop {
                    std::io::stdin().read_line(&mut command)?;
                    command_num = command.trim().parse::<usize>()?;
                    if command_num < DELETE_COMMANDS.len() {
                        break;
                    }
                    println!("Wrong command number");
                }
            },
            _ => {
                println!("Wrong method");
                return Ok(());
            }
        }

        // Ввод json тела запроса
        let mut json = String::new();
        println!("Enter json body string, none is ok:");
        std::io::stdin().read_line(&mut json)?;
        let json = json.trim().to_string();

        // Отправка запроса
        let mut req = hyper::Request::new(hyper::Body::from(json));
        *req.method_mut() = match METHODS[method_num] {
            "GET" => hyper::Method::GET,
            "POST" => hyper::Method::POST,
            "PUT" => hyper::Method::PUT,
            "DELETE" => hyper::Method::DELETE,
            _ => {
                println!("Wrong method");
                return Ok(());
            }
        };
        *req.uri_mut() = match METHODS[method_num] {
            "GET" => format!("{}/{}", address, GET_COMMANDS[command_num]).parse()?,
            "POST" => format!("{}/{}", address, POST_COMMANDS[command_num]).parse()?,
            "PUT" => format!("{}/{}", address, PUT_COMMANDS[command_num]).parse()?,
            "DELETE" => format!("{}/{}", address, DELETE_COMMANDS[command_num]).parse()?,
            _ => {
                println!("Wrong method");
                return Ok(());
            }
        };

        // Вывод запроса
        println!("====================");
        println!("Request: {}", req.method());
        println!("Uri: {}", req.uri());

        // Отправка запроса и обработать исключение если hyper::Error(IncompleteMessage)
        let res = match client.request(req).await {
            Ok(res) => res,
            Err(e) => {
                println!("Error: {}", e);
                continue;
            }
        };


        // Вывод ответа
        println!("Response: {}", res.status());
        let body = hyper::body::to_bytes(res).await?;
        println!("Body: {}", std::str::from_utf8(&body)?);

        // Повторить запрос
        let mut repeat = String::new();
        println!("Repeat? (y/n)");
        std::io::stdin().read_line(&mut repeat)?;
        if repeat.trim() == "n" {
            break;
        }

        iteration += 1;
    }
    Ok(())
}
