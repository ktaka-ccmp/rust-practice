# Simple api-server-swagger demo

## Run app from app root

create .env file if not exists

```text
cat .env
DATABASE_URL="sqlite:./db/sqlite.db"
```

```text
cargo run
```

## Doc URLs

- http://localhost:3000/swagger-ui
- http://localhost:3000/redoc
- http://localhost:3000/rapidoc
- http://localhost:3000/scalar

- For openapi.json
  - `curl http://localhost:3000/api-docs/openapi.json|jq .|less`
