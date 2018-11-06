#!/bin/sh

set -e

export $(grep -v '^#' .env.testing | xargs)
docker-compose -f docker-compose.testing.yml up -d --build knock-mariadb-testing
if ! [ -x "$(command -v diesel)" ]; then
  cargo install diesel_cli --no-default-features --features mysql ;
fi
diesel migration run --database-url=$KNOCK_DATABASE_URL
cargo test
