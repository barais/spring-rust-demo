#[cfg(feature = "postgres")]
use spring::App;
use spring::app::AppBuilder;
use spring::async_trait;
use spring::config::ConfigRegistry;
#[cfg(feature = "postgres")]
use spring::error::Result;
use spring::plugin::Plugin;
#[cfg(feature = "postgres")]
use spring::plugin::{ComponentRegistry, MutableComponentRegistry};
use spring::tracing;
use sqlx::Database;
#[cfg(feature = "postgres")]
use std::sync::Arc;
use std::time::Duration;

use crate::{config::welds::WeldsConfig};

pub struct WeldsPlugin;

#[cfg(feature = "postgres")]
pub type WeldsClient = welds::connections::postgres::PostgresClient;

#[cfg(feature = "mysql")]
pub type WeldsClient = welds::connections::mysql::MySqlClient;
#[cfg(feature = "mssql")]
pub type WeldsClient = welds::connections::mssql::mssqlClient;


#[async_trait]
impl Plugin for WeldsPlugin {
    async fn build(&self, app: &mut AppBuilder) {
        let config = app
            .get_config::<WeldsConfig>()
            .expect("Welds plugin config load failed");
 
        #[cfg(feature = "postgres")]
        let client = Self::connect(&config)
            .await
            .expect("Welds plugin load failed");
        tracing::info!("Welds connection success");

        #[cfg(feature = "postgres")]
        app.add_component(client)
            .add_shutdown_hook(|app| Box::new(Self::close_db_connection(app)));
//        migrate(&client, &config).await.expect("Welds migration failed");
        
    }
}

impl WeldsPlugin {
    #[cfg(feature = "postgres")]
    pub async fn connect(config: &WeldsConfig) -> Result<WeldsClient> {
        use sqlx::postgres::PgPoolOptions;
        use welds::connections::postgres::PostgresClient;

        let opt = Self::configure_pool(PgPoolOptions::new(), config);
        let client1 = opt.connect(&config.uri).await;
        // welds::connections::postgres::From::from(client1);
        tracing::info!("Welds connection pool create success");
        let client: PostgresClient = PostgresClient::from(client1.unwrap());


        //welds::connections::postgres::connect(&config.uri).await.unwrap();
        // client.as_sqlx_pool().set_connect_options(opt);
        Ok(client)
    }

    


    #[cfg(feature = "postgres")]
    async fn close_db_connection(app: Arc<App>) -> Result<String> {
        app.get_component::<WeldsClient>()
            .expect("welds client not exists")
            .as_sqlx_pool()
            .close()
            .await;
        Ok("welds connection pool close successful".into())
    }

    fn configure_pool<T>(
        mut opt: sqlx::pool::PoolOptions<T>,
        config: &WeldsConfig,
    ) -> sqlx::pool::PoolOptions<T>
    where
        T: Database,
    {
        opt = opt
            .max_connections(config.max_connections)
            .min_connections(config.min_connections);

        if let Some(acquire_timeout) = config.acquire_timeout {
            opt = opt.acquire_timeout(Duration::from_millis(acquire_timeout));
        }
        if let Some(idle_timeout) = config.idle_timeout {
            opt = opt.idle_timeout(Duration::from_millis(idle_timeout));
        }
        if let Some(connect_timeout) = config.connect_timeout {
            opt = opt.max_lifetime(Duration::from_millis(connect_timeout));
        }

        opt
    }
}


