use serde_json::Value;
use sqlx

use crate::permission::Permission;

pub trait SqlClient {
    fn read_data(&self, url: &str) -> Result<Vec<Permission>, Box<dyn std::error::Error>>;
}

pub enum SqlPool {
    Postgres(sqlx::postgres::PgPool),
    MySql(sqlx::mysql::MySqlPool),
}

pub struct PostgresClient;

impl SqlClient for PostgresClient {
    #[tokio::main]
    async fn read_data(&self, url: &str) -> Result<Vec<Permission>, Box<dyn std::error::Error>> {
        let pool = PgPool::connect(url).await?;
        let permissions = list_permissions(&pool).await?;
        return Ok(permissions);
    }
}

pub struct MysqlClient;

impl SqlClient for MysqlClient {
    #[tokio::main]
    async fn read_data(&self, url: &str) -> Result<Vec<Permission>, Box<dyn std::error::Error>> {
        let pool = MySqlPool::connect(url).await?;
        let permissions = list_permissions(&pool).await?;
        return Ok(permissions);
    }
}

// Similarly, create implementations for SQLite and MS SQL clients

async fn list_permissions(pool: &PgPool) -> anyhow::Result<Vec<Permission>> {
    let row: Vec<Permission> = sqlx::query_as!(
        Permission,
        "SELECT id, role, collection, action, permissions, validation, presets, fields from directus_permissions"
    )
    .fetch_all(pool)
    .await?;

    return Ok(row);
}
