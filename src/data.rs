use diesel;
use diesel::prelude::*;
use diesel::insertable::Insertable;
use serde::de::Deserializer;
use super::schema::{clients, updates};

#[derive(Serialize, Deserialize, Queryable)]
#[table_name = "clients"]
pub struct Client {
    pub update_timestamp: String,
    pub id: i32,
    pub subdomain: String,
    pub remote_ip: String,
    pub sequence_number: i64,
}

#[derive(Insertable)]
#[table_name = "clients"]
struct InsertableClient {
    pub subdomain: String,
    pub remote_ip: String,
    pub sequence_number: i64,
}

impl InsertableClient {
    fn from_client(client: Client) -> InsertableClient {
        InsertableClient {
            subdomain: client.subdomain,
            remote_ip: client.remote_ip,
            sequence_number: client.sequence_number,
        }
    }
}

#[derive(Serialize, Deserialize, Queryable)]
#[table_name = "updates"]
pub struct Update {
    pub update_timestamp: String,
    pub id: i32,
    pub client_id: i32,
    pub request_ip: String,
}

#[derive(Insertable)]
#[table_name = "updates"]
struct InsertableUpdate {
    pub request_ip: String,
}

impl InsertableUpdate {
    fn from_update(update: Update) -> InsertableUpdate {
        InsertableUpdate {
            request_ip: update.request_ip,
        }
    }
}
