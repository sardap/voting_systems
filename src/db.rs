use diesel::{
    r2d2::{self, ConnectionManager},
    PgConnection,
};

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub type DbConnection =
    diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::PgConnection>>;
