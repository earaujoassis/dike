#!/bin/sh
# datastore.sh

set -e

if [[ -z "${DIKE_DATABASE_URL}" ]]; then export $(grep -v '^#' .env | xargs) ; fi
echo "> Attempt connection to 'mysql --host=$(echo $DIKE_DATABASE_URL | scripts/urlparser.py host) -P $(echo $DIKE_DATABASE_URL | scripts/urlparser.py port) -u $(echo $DIKE_DATABASE_URL | scripts/urlparser.py user) --password=$(echo $DIKE_DATABASE_URL | scripts/urlparser.py passwd) ${DIKE_DATABASE_NAME_PREFIX}_${DIKE_ENV}'"
until echo "\q" | mysql --host=$(echo $DIKE_DATABASE_URL | scripts/urlparser.py host) -P $(echo $DIKE_DATABASE_URL | scripts/urlparser.py port) -u $(echo $DIKE_DATABASE_URL | scripts/urlparser.py user) --password=$(echo $DIKE_DATABASE_URL | scripts/urlparser.py passwd) "${DIKE_DATABASE_NAME_PREFIX}_${DIKE_ENV}" 2> /dev/null; do
  >&2 echo "> Datastore is unavailable; sleeping"
  sleep 5
done

>&2 echo "> Datastore is up; executing command"

# Create database
# echo "CREATE DATABASE IF NOT EXISTS \`${DIKE_DATABASE_NAME_PREFIX}_${DIKE_ENV}\` ; GRANT ALL ON \`${DIKE_DATABASE_NAME_PREFIX}_${DIKE_ENV}\`.* TO '$(echo $DIKE_DATABASE_URL | scripts/urlparser.py user)'@'%' ;" | mysql --host=$(echo $DIKE_DATABASE_URL | scripts/urlparser.py host) -P $(echo $DIKE_DATABASE_URL | scripts/urlparser.py port) -u root --password=$DIKE_ROOT_PASSWORD
