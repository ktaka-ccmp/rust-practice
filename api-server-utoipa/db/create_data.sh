#!/bin/bash

DB=./sqlite.db

for i in {001..080} ; do
echo "insert into customer(name,email) values('a$i','a$i@example.com')"  \
| sqlite3 $DB
done

echo "Customer:"
echo "select * from customer" | sqlite3 $DB | tail

