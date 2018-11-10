#!/bin/sh
# test.sh

set -e

if [ -e .env ]; then export $(grep -v '^#' .env | xargs) ; fi
export RUST_BACKTRACE=full
export DIKE_ENV=testing

if ! [ -x "$(command -v diesel)" ]; then
  echo "> Installing diesel_cli from 'cargo'"
  cargo install diesel_cli --no-default-features --features mysql ;
fi

sh scripts/datastore.sh

echo "> Executing any new migration"
diesel migration run --database-url="${DIKE_DATABASE_URL}${DIKE_DATABASE_NAME_PREFIX}_${DIKE_ENV}"
echo "> Starting tests"
cargo build --verbose
cargo test --verbose
