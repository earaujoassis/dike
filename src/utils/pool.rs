use diesel::mysql::MysqlConnection;
use r2d2;
use r2d2_diesel::ConnectionManager;
use std::env;
use slog::Logger;

pub type DieselConnection = r2d2::PooledConnection<ConnectionManager<MysqlConnection>>;
pub type DieselPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

pub fn pool_factory(logger: &Logger) -> DieselPool {
    let logger = logger.new(o!("module" => "pool_factory"));

    let database_url = env::var("DIKE_DATABASE_URL").expect("DIKE_DATABASE_URL must be set");
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    let pool = r2d2::Pool::new(manager).expect("Failed to create diesel pool.");

    info!(logger, "Diesel pool created");

    pool
}
