use datastore::models::client::Client;
use datastore::schema::updates;
use datastore::schema::updates::dsl::*;

use diesel;
use diesel::mysql::Mysql;
use diesel::prelude::*;
use utils::pool::DieselConnection;
use slog::Logger;
use chrono::*;

#[derive(Debug, Insertable, Deserialize)]
#[table_name="updates"]
pub struct InsertableUpdate {
    pub client_id: u64,
    pub request_ip: String
}


#[derive(Clone, Debug, Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(Client, foreign_key="client_id")]
pub struct Update {
    pub update_timestamp: NaiveDateTime,
    pub id: u64,
    pub client_id: u64,
    pub request_ip: String
}

impl Update {
    pub fn create(update: &InsertableUpdate, connection: &DieselConnection, logger: &Logger) -> Result<Update, diesel::result::Error> {
        let statement = diesel::insert_into(updates::table).values(update);
        let _sql = diesel::debug_query::<Mysql, _>(&statement);
        info!(logger, "Executing Query"; "query" => "oh god");
        statement.execute(&**connection)?;
        let update = updates.order(updates::columns::id.desc()).first::<Update>(&**connection);
        match update {
            Ok(update) => Ok(update),
            Err(error) => {
                warn!(logger, "Error creating update"; o!("error" => format!("{:?}", error)));
                Err(error)
            }
        }
    }
}
