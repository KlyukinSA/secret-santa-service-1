# Тайный Кыш Бабай
![photo_2023-01-01_13-27-35](https://user-images.githubusercontent.com/53406289/210182274-739bcd9c-3611-4d96-982b-66511ed5df6d.jpg)

Веб-сервис для тайного кыш бабая

## Как запустить решение 

1. Склонировать репозиторий
2. `cargo run`

Запустится сервер, обрабатывающий http запросы. Чтобы его остановить, ctrl+C.

## Как тестировать

1. Запустить решение
2. Сделать HTTP запрос

## Как сделать HTTP запрос

### На Windows

Пишите строчку `Invoke-RestMethod -ContentType "application/json" -Uri "http://127.0.0.1:8080/user/create" -Method Post -Body '{"name":"Danis"}'` в PowerShell

### На Linux

- Проще всего использовать cli-программу curl. Например, `curl --header "Content-Type: application/json" --request POST --data '{"name":"Danis"}' http://127.0.0.1:8080/user/create` добавит Даниса в список пользователей и выведет его айдишник.
- Или без курла в файле `main/src/from_lessons/client.rs` делается http запрос с помощью растовской либы. 
![пример](https://user-images.githubusercontent.com/53406289/210182204-86ba21a9-c128-4e12-8590-7b9e39953298.png)

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
