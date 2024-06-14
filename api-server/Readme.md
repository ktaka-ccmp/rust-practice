# Simple api-server demo

## Run app from src dir

create .env file if not exists

```text
cat src/.env
DATABASE_URL="sqlite:../db/sqlite.db"
```

```text
cd src
cargo run
```

## Run app from app root

create .env file if not exists

```text
cat .env
DATABASE_URL="sqlite:./db/sqlite.db"
```

```text
cargo run
```

## Prepare SQLiteDB

```text
cd db/
sqlx migrate run --database-url sqlite:./sqlite.db
```

## Cleanup SQLiteDB

```text
cd db/
sqlx migrate revert --database-url sqlite:./sqlite.db
```

## Prepare migrations if not exists

```text
cd db/ 
sqlx migrate add -r customer
```

Then edit files under migrations

```text
ktaka@dyna:~/GitHub/rust-practice/api-server/db$ ls -la migrations/
total 16
drwxr-xr-x 2 ktaka ktaka 4096 May 26 05:00 .
drwxr-xr-x 3 ktaka ktaka 4096 Jun 14 18:43 ..
-rw-r--r-- 1 ktaka ktaka   21 May 26 05:03 20240525200001_customer.down.sql
-rw-r--r-- 1 ktaka ktaka  492 May 26 05:31 20240525200001_customer.up.sql
```

```text
ktaka@dyna:~/GitHub/rust-practice/api-server/db$ cat migrations/20240525200001_customer.up.sql 
CREATE TABLE IF NOT EXISTS customer (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL,
        email TEXT NOT NULL
);

-- Generate a sequence of numbers from 1 to 80
WITH RECURSIVE numbers AS (
  SELECT 1 AS num
  UNION ALL
  SELECT num + 1
  FROM numbers
  WHERE num < 80
)

-- Insert the generated sequence into the customer table
INSERT INTO customer (name, email)
SELECT 
  printf('a%03d', num) AS name, 
  printf('a%03d@example.com', num) AS email
FROM numbers;
ktaka@dyna:~/GitHub/rust-practice/api-server/db$ cat migrations/20240525200001_customer.down.sql 
DROP TABLE customer;
```
