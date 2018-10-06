#!/bin/sh
# wait-for-mysql.sh

set -e

host="$1"

until echo "\q" | mysql --host="$host" -P "3306" -u "root" --password="root"; do
  >&2 echo "Datastore is unavailable; sleeping"
  sleep 1
done

>&2 echo "Datastore is up; executing command"
