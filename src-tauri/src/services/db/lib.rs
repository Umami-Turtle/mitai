use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};
use std::{fs, time::Duration};
use tauri::{AppHandle, Manager};

pub async fn setup_db(handle: &AppHandle) -> Result<DatabaseConnection, DbErr> {
    let _ = fs::create_dir_all(handle.path().app_data_dir().unwrap());
    let db_path = handle.path().app_data_dir().unwrap().join("mitra.db");
    let db_string = db_path.to_string_lossy();

    let mut opt = ConnectOptions::new(db_string);
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true);
    // .sqlx_logging_level(log::LevelFilter::Info)

    let connection = Database::connect(opt).await?;
    Migrator::up(&connection, None).await?;

    Ok(connection)
}
