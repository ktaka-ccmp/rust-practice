# Database setup

## PostgreSQL

```text
docker compose up -d
docker compose down

docker exec -it db-pg-1 bash
```

```text
root@b27cc4f46d97:/#

root@f6e164f5961c:/# psql -U fc -c '\l'
                                                   List of databases
   Name    | Owner | Encoding | Locale Provider |  Collate   |   Ctype    | ICU Locale | ICU Rules | Access privileges 
-----------+-------+----------+-----------------+------------+------------+------------+-----------+-------------------
 fc        | fc    | UTF8     | libc            | en_US.utf8 | en_US.utf8 |            |           | 
 postgres  | fc    | UTF8     | libc            | en_US.utf8 | en_US.utf8 |            |           | 
 template0 | fc    | UTF8     | libc            | en_US.utf8 | en_US.utf8 |            |           | =c/fc            +
           |       |          |                 |            |            |            |           | fc=CTc/fc
 template1 | fc    | UTF8     | libc            | en_US.utf8 | en_US.utf8 |            |           | =c/fc            +
           |       |          |                 |            |            |            |           | fc=CTc/fc
(4 rows)

root@f6e164f5961c:/# psql -U fc -c '\dt'
        List of relations
 Schema |  Name  | Type  | Owner 
--------+--------+-------+-------
 public | cities | table | fc
(1 row)

root@f6e164f5961c:/# psql -U fc -c 'select * from cities;'
 id |  name  |   lat    |  long   
----+--------+----------+---------
  1 | yamato | 35.47276 | 139.451
(1 row)
```

## SQLite

```text
cargo install sqlx-cli --no-default-features --features sqlite
cargo add sqlx --features "sqlite runtime-tokio-native-tls chrono"

sqlx database create --database-url "sqlite:./sqlite.db"

sqlx migrate add -r create_users_table

cat << EOF > migrations/20240524232423_create_users_table.up.sql 
CREATE TABLE IF NOT EXISTS cities (
        id SERIAL PRIMARY KEY,
        name TEXT NOT NULL,
        lat NUMERIC NOT NULL,
        long NUMERIC NOT NULL
);

CREATE INDEX IF NOT EXISTS cities_name_idx ON cities (name);
EOF

cat << EOF >  migrations/20240524232423_create_users_table.down.sql 
DROP TABLE cities;
EOF

sqlx migrate run --database-url sqlite:./sqlite.db
```

```text
$ echo ".tables" | sqlite3 ./sqlite.db 
_sqlx_migrations  cities          
$ echo "select * from cities" | sqlite3 ./sqlite.db 
$ echo "pragma table_info(cities)" | sqlite3 ./sqlite.db 
0|id|SERIAL|0||1
1|name|TEXT|1||0
2|lat|NUMERIC|1||0
3|long|NUMERIC|1||0
$ echo "select * from cities" | sqlite3 ./sqlite.db 
```
