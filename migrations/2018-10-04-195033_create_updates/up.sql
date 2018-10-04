-- Your SQL goes here
CREATE TABLE updates (
  update_timestamp TIMESTAMP NOT NULL,
  id SERIAL PRIMARY KEY NOT NULL,
  client_id BIGINT UNSIGNED NOT NULL,
  UNIQUE INDEX client_index (`client_id`),
  request_ip VARCHAR(50) NOT NULL,
  CONSTRAINT fk_client_id FOREIGN KEY (`client_id`) REFERENCES `clients`(`id`) ON DELETE CASCADE
);
