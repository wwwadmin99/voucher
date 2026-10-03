# voucher

Система ваучеров: у каждого пользователя есть запас активаций на каждый
продукт. Администратор задаёт запас, пользователь активирует ваучер (запас
−1, запись в историю), активация блокируется, если ваучеров не осталось.

Стек: Rust / Axum / sqlx / PostgreSQL (backend), React / TypeScript / shadcn-ui
(frontend).

## Запуск одной командой

Нужен Docker (и, если хочется `npm start`, — Node.js; без него просто
используйте команду docker compose напрямую).

```sh
npm start
# или напрямую:
docker compose up --build
```

Открыть http://localhost:8080 — один сервис `api` поднимает миграции (и сид
пользователей/продуктов) и раздаёт и API, и собранный фронтенд.

Остановить: `npm stop` (или `docker compose down`).

Swagger UI с OpenAPI-схемой: http://localhost:8080/api/docs

## Локальная разработка (без Docker)

Backend:

```sh
cd backend/api
cp .env.example .env   # поправить DATABASE_URL под свой Postgres
cargo run
```

Frontend (с hot-reload, проксирует `/api` на `localhost:8080`):

```sh
cd frontend
pnpm install
pnpm dev
```

## Тесты

```sh
cd backend/api
cargo test
```

Тесты (`backend/api/tests/activation.rs`) поднимают через `#[sqlx::test]`
изолированную БД на миграциях и проверяют: декремент баланса + запись в
историю при активации, 409 при отсутствии ваучеров, upsert в админ-эндпоинте
задания запаса, 422 при отрицательном количестве.

## Генерация API-клиента для фронтенда

Схема OpenAPI генерируется backend'ом (`utoipa`) из кода хендлеров. TS-типы
под `frontend/src/lib/api-types.ts` сгенерированы из неё через
`openapi-typescript` и используются клиентом `openapi-fetch`
(`frontend/src/lib/api.ts`). Перегенерировать после изменения backend-роутов
(при запущенном `cargo run`):

```sh
cd frontend
pnpm gen:api
```

## Структура

```
backend/api/   — Axum API: users, products, voucher_balances, activations
backend/api/migrations/ — sqlx-миграции (схема + сид пользователей/продуктов)
frontend/      — React + TS + shadcn/ui
Dockerfile     — multi-stage: собирает фронт и бэкенд, runtime-образ раздаёт оба
docker-compose.yml — Postgres + api
```
