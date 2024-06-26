[![build](https://github.com/ruauka/mortgage-rs/actions/workflows/pipeline.yml/badge.svg)](https://github.com/ruauka/mortgage-rs/actions/workflows/pipeline.yml)
[![Coverage Status](https://codecov.io/github/ruauka/mortgage-rs/coverage.svg?branch=master)](https://codecov.io/gh/ruauka/mortgage-rs)

# Задание

Требуется написать сервис расчета параметров ипотеки (`ипотечный калькулятор`).
Рассчитываемые параметры:
- процентная ставка, исходя из запрошенной `программы кредитования`
- сумма кредита
- аннуитетный ежемесячный платеж
- переплата за весь срок кредита
- дата последнего платежа

Все расчеты требуется сохранять в локальном `кэше`.

## Программы кредитования
Есть 3 программы кредитования. Каждая из них предполагает свою годовую процентную ставку:

1. Программа для корпоративных клиентов. Годовая процентная ставка по кредиту - `8%`.
2. Военная ипотека. Годовая процентная ставка по кредиту - `9%`.
3. Базовая программа. Годовая процентная ставка по кредиту - `10%`.

Для каждой программы `первоначальный взнос должен быть не ниже 20%` от стоимости объекта.

Программа кредита указывается в запросе (входном JSON).

## Формула расчета ежемесячного (аннуитетного) платежа

<p align="left">
    <img src="assets/аннуитет.png" width="500">
</p>

- PM — размер ежемесячного платежа
- S — сумма ипотечной задолженности
- G — ежемесячная процентная ставка, равная 1/12 от годовой процентной ставки по кредиту
- T — количество периодических процентных периодов, оставшихся до окончательного погашения задолженности

Файл `for_check.xlsx` для проверки.

## Запрос (входной JSON)

Запрос на сервис:
```json
{
    "object_cost": 5000000,     // стоимость объекта
    "initial_payment": 1000000, // первоначальный взнос
    "months": 240,              // срок
    "program": {                // блок программы кредита
        "salary": true,         // программа для корпоративных клиентов
        "military": true,       // военная ипотека
        "base": true            // базовая программа
    }
}
```

Для указания программы требуется передавать только 1 поле. Например:
```json
{
    "object_cost": 5000000,
    "initial_payment": 1000000,
    "months": 240,
    "program": {
        "salary": true
    }
}
```

## Эндпоинты

Требуется реализовать 2 эндпоинта:
1. `/execute` - расчет ипотеки (POST).
2. `/cache` - получение всех рассчитанных ипотек из кэша (GET).

## /execute
В качестве входных данных эндпоинт принимает JSON:
```json
{
    "object_cost": 5000000,
    "initial_payment": 1000000,
    "months": 240,
    "program": {
        "salary": true
    }
}
```
В качестве ответа возвращается JSON и `status code: 200`:
```json
{
   "id": 0,                                 // id расчета, инкрементируется на каждом расчете
   "loan": {
      "params": {                           // запрашиваемые параметры кредита
         "object_cost": 5000000,
         "initial_payment": 1000000,                
         "months": 240
      },
      "program": {                          // программа кредита
         "salary": true
      },
      "aggregates": {                       // блок с агрегатами
         "rate": 8,                         // годовая процентная ставка
         "loan_sum": 4000000,               // сумма кредита
         "monthly_payment": 33458,          // аннуитетный ежемесячный платеж
         "overpayment": 4029920,            // переплата за весь срок кредита
         "last_payment_date": "2044-02-18"  // последняя дата платежа
      }
   }
}
```

В случае, если не выбрана ни одна из программ (во входном JSON все поля программы `false`), то требуется возвращать `status code: 400` и ошибку:
```json
{
    "error": "choose program"
}
```

В случае, если выбрана более, чем одна программа, то требуется возвращать `status code: 400` и ошибку:
```json
{
   "error": "choose only 1 program"
}
```

В случае, если первоначальный взнос ниже 20% от стоимости объекта, то требуется возвращать `status code: 400` и ошибку:
```json
{
    "error": "the initial payment should be more"
}
```


Результат расчета кредита требуется сохранять в `кэш`.

## /cache
Сервис возвращает массив из рассчитанных кредитов и `status code: 200`:
```json
[
   {
      "id": 0,
      "loan": {
          "params": {
              "object_cost": 5000000,
              "initial_payment": 1000000,
              "months": 240
          },
          "program": {
              "salary": true
          },
          "aggregates": {
              "rate": 8,
              "loan_sum": 4000000,
              "monthly_payment": 33458,
              "overpayment": 4029920,
              "last_payment_date": "2044-02-18"
          }
      }
   },
   {
      "id": 1,
      "loan": {
          "params": {
              "object_cost": 8000000,
              "initial_payment": 2000000,
              "months": 200
          },
          "program": {
              "military": true
          },
          "aggregates": {
              "rate": 9,
              "loan_sum": 6000000,
              "monthly_payment": 58019,
              "overpayment": 5603800,
              "last_payment_date": "2040-10-18"
          }
      }
   },
   {
      "id": 2,
      "loan": {
          "params": {
              "object_cost": 12000000,
              "initial_payment": 3000000,
              "months": 120
          },
          "program": {
              "base": true
          },
          "aggregates": {
              "rate": 10,
              "loan_sum": 9000000,
              "monthly_payment": 118936,
              "overpayment": 5272320,
              "last_payment_date": "2034-02-18"
          }
      }
   }
]
```
Если кэш пустой, то требуется возвращать `status code: 400` и ошибку:
```json
{
   "error": "empty cache"
}
```

## Кэш
Требуется сохранять рассчитанные кредиты и отдавть их по запросу на /cache.
Кэш должен быть реализован в `RAM`, без использования сторонних БД.

## Middleware
Требуется реализовать middleware, который будет выводить в консоль информацию о запросе:
- `path` - эндпоинт
- `status` - статус запроса
- `status_code` - http код запроса
- `duration` - время работы эндпоинта (μs - microseconds)
```bash
2024-05-19T10:56:27.387040Z  INFO path=/execute, status=Success, status_code=200 OK, duration=292 μs
2024-05-19T10:56:33.507095Z ERROR path=/execute, status=Error, status_code=400 Bad Request, duration=168 μs
```
Требование обязательно, даже если используемый вами web-framework предоставляет такой функционал "из коробки".

## Требования и ограничения
1. Можно использовать любой web-framework.
2. Сервис должен иметь настраиваемую конфигурацию.
   `Host` и `port`, на которых поднимается сервис, должены указываться в переменных окружения. Дефолтные значения:
```yaml
localhost:8080
```
3. Покрытие unit-тестами > 80%.
<p align="left">
    <img src="assets/coverage.png" width="800">
</p>

4. Код должен проходить проверку `clippy`. Конфигурация:
```toml
[workspace.lints]
clippy.all = { level = "deny", priority = -1 }
clippy.pedantic = { level = "deny", priority = -1 }
clippy.restriction = { level = "deny", priority = -1 }
clippy.cargo = { level = "deny", priority = -1 }
clippy.nursery = "deny"
```
5. Проект содержит `Dockerfile`. "Вес" образа должен быть не более `150 MB`.
6. Проект содержит `CI pipeline` конфиг c этапами:
    - Check (cargo check...)
    - Fmt (cargo fmt...)
    - Clippy (cargo clippy...)
    - Test (cargo test...)

   `CI Pipeline` запускается:
     - После коммита в master ветку
     - После мержа pull-request в master ветку
7. Проект содержит `Makefile`, в котором указаны команды:
    - запуска тестов
    - запуска тестов с отчетом покрытия кода
    - сборки образа
    - запуска контейнера
    - остановки и удаления контейнера