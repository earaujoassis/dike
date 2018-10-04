-- Your SQL goes here
CREATE TABLE clients (
  update_timestamp TIMESTAMP NOT NULL,
  id SERIAL PRIMARY KEY NOT NULL,
  subdomain VARCHAR(50) NOT NULL,
  remote_ip VARCHAR(50) NOT NULL,
  sequence_number BIGINT NOT NULL DEFAULT 0
)
