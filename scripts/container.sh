#!/bin/sh
# start-knocking.sh

set -e

export $(grep -v '^#' .env | xargs)
until echo "\q" | mysql --host=$(sed -E -e "s_.*://([^/@]*@)?([^/:]+).*_\2_" <<< "$KNOCK_DATABASE_URL") -P "3306" -u "root" --password="root"; do
  >&2 echo "Datastore is unavailable; sleeping"
  sleep 1
done

>&2 echo "Datastore is up; executing command"

diesel migration run --database-url=$KNOCK_DATABASE_URL
cargo run
