#!/bin/bash

echo -n "odin password: "
read -s pgpass
echo

# http://stackoverflow.com/questions/369758/how-to-trim-whitespace-from-bash-variable
trim()
{
  # local var=$1
  # var="${var#"${var%%[![:space:]]*}"}"   # remove leading whitespace characters
  # var="${var%"${var##*[![:space:]]}"}"   # remove trailing whitespace characters
  # echo -n "$var"
  read -r line
  echo "$line"
}

# Add background data for environments
PGPASSWORD=${pgpass} psql -h postgres -U odin -d mjolnir -t -c "INSERT INTO main.index_types VALUES ('admins');"
PGPASSWORD=${pgpass} psql -h postgres -U odin -d mjolnir -t -c "INSERT INTO main.index_types VALUES ('streets');"
PGPASSWORD=${pgpass} psql -h postgres -U odin -d mjolnir -t -c "INSERT INTO main.index_types VALUES ('addresses');"
PGPASSWORD=${pgpass} psql -h postgres -U odin -d mjolnir -t -c "INSERT INTO main.index_types VALUES ('public_pois');"
PGPASSWORD=${pgpass} psql -h postgres -U odin -d mjolnir -t -c "INSERT INTO main.index_types VALUES ('private_pois');"

PGPASSWORD=${pgpass} psql -h postgres -U odin -d mjolnir -t -c "INSERT INTO main.data_sources VALUES ('osm');"
PGPASSWORD=${pgpass} psql -h postgres -U odin -d mjolnir -t -c "INSERT INTO main.data_sources VALUES ('cosmogony');"
PGPASSWORD=${pgpass} psql -h postgres -U odin -d mjolnir -t -c "INSERT INTO main.data_sources VALUES ('bano');"
PGPASSWORD=${pgpass} psql -h postgres -U odin -d mjolnir -t -c "INSERT INTO main.data_sources VALUES ('openaddress');"

PGPASSWORD=${pgpass} psql -h postgres -U odin -d mjolnir -t -c "INSERT INTO main.index_type_data_source VALUES('admins', 'cosmogony');"
PGPASSWORD=${pgpass} psql -h postgres -U odin -d mjolnir -t -c "INSERT INTO main.index_type_data_source VALUES('admins', 'osm');"
PGPASSWORD=${pgpass} psql -h postgres -U odin -d mjolnir -t -c "INSERT INTO main.index_type_data_source VALUES('streets', 'osm');"
PGPASSWORD=${pgpass} psql -h postgres -U odin -d mjolnir -t -c "INSERT INTO main.index_type_data_source VALUES('addresses', 'bano');"
PGPASSWORD=${pgpass} psql -h postgres -U odin -d mjolnir -t -c "INSERT INTO main.index_type_data_source VALUES('addresses', 'openaddress');"
PGPASSWORD=${pgpass} psql -h postgres -U odin -d mjolnir -t -c "INSERT INTO main.index_type_data_source VALUES('public_pois', 'osm');"

# Create a Feature with 2 scenarios
feat1=$(PGPASSWORD=${pgpass} psql -h postgres -U odin -d mjolnir -t -c "SELECT * FROM main.create_or_replace_feature ('search for pokemons', 'searching for pokemons is a fun activity', '{\"pokemon\", \"search\"}');")
feat1id=$(echo ${feat1} | cut -d '|' -f 1 | trim)

sce1=$(PGPASSWORD=${pgpass} psql -h postgres -U odin -d mjolnir -t -c "SELECT * FROM main.create_or_replace_scenario ('in the park', '{\"park\"}', '${feat1id}');")
sce1id=$(echo ${sce1} | cut -d '|' -f 1 | trim)

sce2=$(PGPASSWORD=${pgpass} psql -h postgres -U odin -d mjolnir -t -c "SELECT * FROM main.create_or_replace_scenario ('in the parking lot', '{\"parking lot\"}', '${feat1id}');")
sce2id=$(echo ${sce2} | cut -d '|' -f 1 | trim)

echo "Creating an index"
idx1=$(PGPASSWORD=${pgpass} psql -h postgres -U odin -d mjolnir -t -c "SELECT * FROM main.create_or_replace_index('admins', 'cosmogony', '{\"france\"}');")
idx1id=$(echo ${idx1} | cut -d '|' -f 1 | trim)
echo "created index 1: $idx1id"

echo "Adding the index to a scenario"
sceid=$(PGPASSWORD=${pgpass} psql -h postgres -U odin -d mjolnir -t -c "SELECT id FROM main.scenarios LIMIT 1;" | trim)
env1=$(PGPASSWORD=${pgpass} psql -h postgres -U odin -d mjolnir -t -c "SELECT * FROM main.add_index_to_scenario('${idx1id}', '${sce1id}');")
env1id=$(echo ${env1} | cut -d '|' -f 1 | trim)
echo "Added index $idx1id to environment $env1id (Scenario $sce1id)"

echo "Creating another index"
idx2=$(PGPASSWORD=${pgpass} psql -h postgres -U odin -d mjolnir -t -c "SELECT * FROM main.create_or_replace_index('streets', 'bano', '{\"75\", \"92\", \"94\"}');")
idx2id=$(echo ${idx2} | cut -d '|' -f 1 | trim)
echo "id index 2: $idx2id"

echo "Adding the index 2 to a scenario"
env11=$(PGPASSWORD=${pgpass} psql -h postgres -U odin -d mjolnir -t -c "SELECT * FROM main.add_index_to_scenario('${idx2id}', '${sce1id}');")
env11id=$(echo ${env11} | cut -d '|' -f 1 | trim)
# Since we're adding to the same scenario, we expect to be the same environment
echo "${env1id} < ? > ${env11id}"

signature=$(PGPASSWORD=${pgpass} psql -h postgres -U odin -d mjolnir -t -c "SELECT signature FROM main.environments WHERE id = '${env1id}';")
echo "${signature}"

