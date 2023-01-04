# Тайный Кыш Бабай
![Тайный Кыш Бабай](https://user-images.githubusercontent.com/53406289/210182274-739bcd9c-3611-4d96-982b-66511ed5df6d.jpg)

Веб-сервис для игры в Тайного Кыш Бабая.

## Запуск

1. Склонировать репозиторий.
2. Ввести `cargo run` в терминал.

Запустится сервер, обрабатывающий HTTP запросы. Остановить можно с помощью Сtrl+C.

## Тестирование

1. Запустить решение.
2. Сделать HTTP запрос.

## HTTP запрос

### Для Windows

- В PowerShell пишите строчку по шаблону:
```powershell 
Invoke-RestMethod -ContentType "application/json" -Uri "http://127.0.0.1:8080/HTTP-путь" -Method HTTP-метод -Body ВХОДНЫЕ_ДАННЫЕ
``` 
Пример (добавляет пользователя "Danis" в список пользователей и выводит его ID): 
```powershell 
Invoke-RestMethod -ContentType "application/json" -Uri "http://127.0.0.1:8080/users/create" -Method Post -Body '{"name":"Danis"}'
```

### Для Linux

- CLI-программа cURL. В ней пишите строчку по шаблону:
```bash
curl --header "Content-Type: application/json" --request HTTP-метод --data ВХОДНЫЕ_ДАННЫЕ http://127.0.0.1:8080/HTTP-путь
``` 
Пример (добавляет пользователя "Danis" в список пользователей и выводит его ID): 
```bash
curl --header "Content-Type: application/json" --request POST --data '{"name":"Danis"}' http://127.0.0.1:8080/user/create
``` 

### Универсальный способ

- В файле `main/src/from_lessons/client.rs` делается HTTP запрос при помощи библиотеки Rust. 
![Функция](https://user-images.githubusercontent.com/53406289/210182204-86ba21a9-c128-4e12-8590-7b9e39953298.png)

## Авторы

- **3530904/10001**
  - [Подшивалов Георгий](https://github.com/George3005)
- **3530904/10002**
  - [Головатюков Сергей](https://github.com/serjunya)
  - [Клюкин Степан](https://github.com/KlyukinSA)
  - [Мирончук Юлиана](https://github.com/nanatic)
  - [Набиуллин Данис](https://github.com/dfnabiullin)
  - [Свириденко Сергей](https://github.com/NerouN1919)
  - [Тампио Илья](https://github.com/Quakumei)
  - [Худоложкин Андрей](https://github.com/Andrei2503)
  - [Шемаев Кирилл](https://github.com/ilyasvet)
  - [Шеремет Сергей](https://github.com/SxeCrew)
