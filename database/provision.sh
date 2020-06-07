#!/bin/bash

echo -n "postgres password: "
read -s pgpass
echo

rm -fr ./init.sql
for file in `find functions -name '0*.sql' | sort`; do
  cat "${file}" >> ./init.sql
done

rm -fr ./build.sql
for file in `find functions -name '*.sql' ! -name '0*' | sort`; do
  cat "${file}" >> ./build.sql
done
for file in `find api -name '*.sql' | sort`; do
  cat "${file}" >> ./build.sql
done

echo "initializing"
PGPASSWORD=${pgpass} psql -h postgres -U postgres < init.sql
echo "building"
PGPASSWORD=${pgpass} psql -h postgres -U postgres mjolnir < build.sql
echo "provisioning"
PGPASSWORD=${pgpass} psql -h postgres -U postgres mjolnir < provision.sql
