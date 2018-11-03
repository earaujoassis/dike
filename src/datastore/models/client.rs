use datastore::schema::clients;
use datastore::schema::clients::dsl::*;

use diesel;
use diesel::mysql::Mysql;
use diesel::prelude::*;
use utils::pool::DieselConnection;
use slog::Logger;
use chrono::*;
use std::vec::Vec;

#[derive(Debug, Insertable, Deserialize)]
#[table_name="clients"]
pub struct InsertableClient {
    pub subdomain: String,
    pub remote_ip: String,
    pub sequence_number: i64
}


#[derive(Clone, Debug, Queryable, Serialize, Identifiable, Associations)]
pub struct Client {
    pub update_timestamp: NaiveDateTime,
    pub id: u64,
    pub subdomain: String,
    pub remote_ip: String,
    pub sequence_number: i64
}

impl Client {
    pub fn create(client: &InsertableClient, connection: &DieselConnection, logger: &Logger) -> Result<Client, diesel::result::Error> {
        let statement = diesel::insert_into(clients::table).values(client);
        let _sql = diesel::debug_query::<Mysql, _>(&statement);
        info!(logger, "Executing Query"; "query" => "oh god");
        statement.execute(&**connection)?;
        let client = clients.order(clients::columns::id.desc()).first::<Client>(&**connection);
        match client {
            Ok(client) => Ok(client),
            Err(error) => {
                warn!(logger, "Error creating client"; o!("error" => format!("{:?}", error)));
                Err(error)
            }
        }
    }

    pub fn active_clients(connection: &DieselConnection, logger: &Logger) -> Vec<Client> {
        let statement = clients::table;
        let _sql = diesel::debug_query::<Mysql, _>(&statement);
        info!(logger, "Executing Query"; "query" => "oh god");
        let _clients: Result<Vec<Client>, _> = statement.load(&**connection);
        match _clients {
            Ok(_clients) => _clients,
            Err(error) => {
                warn!(logger, "Error listing clients"; o!("error" => format!("{:?}", error)));
                Vec::new()
            }
        }
    }
}
