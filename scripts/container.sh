#!/bin/sh
# container.sh

set -e

if [ -e .env ]; then export $(grep -v '^#' .env | xargs) ; fi

if ! [ -x "$(command -v diesel)" ]; then
  echo "> Installing diesel_cli from 'cargo'"
  cargo install diesel_cli --no-default-features --features mysql ;
fi

sh scripts/datastore.sh

echo "> Executing any new migration"
diesel migration run --database-url="${DIKE_DATABASE_URL}${DIKE_DATABASE_NAME_PREFIX}_${DIKE_ENV}"
echo "> Starting server"
cargo run
