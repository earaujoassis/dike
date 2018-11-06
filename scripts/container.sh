#!/bin/sh
# container.sh

set -e

export $(grep -v '^#' .env | xargs)
until echo "\q" | mysql --host=$(sed -E -e "s_.*://([^/@]*@)?([^/:]+).*_\2_" <<< "$DIKE_DATABASE_URL") -P "3306" -u "root" --password="root"; do
  >&2 echo "Datastore is unavailable; sleeping"
  sleep 1
done

>&2 echo "Datastore is up; executing command"

diesel migration run --database-url=$DIKE_DATABASE_URL
cargo run
