use sea_orm::DatabaseConnection;
use tauri::async_runtime::Mutex;

pub struct MitraServices {
    pub db: Mutex<DatabaseConnection>,
    // job:Mutex<JobManager>
}
