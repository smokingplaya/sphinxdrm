use std::env;
use log::error;
use redis::{Commands, Connection, ConnectionAddr, ConnectionInfo, RedisConnectionInfo, RedisResult};

pub type ProductsInDB = Vec<String>;

pub(crate) struct Redis {
  pub(crate) redis: Connection,
}

impl Redis {
  pub fn new(redis: Connection) -> Self {
    Self { redis }
  }

  /**
   * get
   * @param key - MAC Address устройства (они являются ключом)
   */
  pub fn get(&mut self, key: &str) -> RedisResult<ProductsInDB> {
    let data = self.redis.get(key).unwrap_or("[]".to_string());

    Ok(serde_json::from_str(data.as_str()).unwrap())
  }

  pub fn has(&mut self, key: &str, product: &str) -> RedisResult<bool> {
    let data = self.get(key).unwrap();

    Ok(data.contains(&product.to_string()))
  }

  #[allow(unused_must_use)]
  pub fn add(&mut self, key: &str, product: &str) -> RedisResult<()> {
    let mut products = self.get(key)?;
    products.push(product.to_string());

    self.update(key, products);

    Ok(())
  }

  pub fn update(&mut self, key: &str, products: ProductsInDB) -> RedisResult<()> {
    self.redis.set(key, serde_json::json!(products).to_string())?;

    Ok(())
  }
}

pub(crate) trait ResultError {
  fn check_err(&self) -> bool;
}

impl ResultError for RedisResult<Redis> {
  fn check_err(&self) -> bool {
    self.is_err().then(|| error!("redis not connected")).is_some()
  }
}

pub fn create_connection() -> RedisResult<Redis> {
  let client = redis::Client::open(ConnectionInfo {
    addr: ConnectionAddr::Tcp(env::var("REDIS_HOST").unwrap(), env::var("REDIS_PORT").unwrap().parse::<u16>().unwrap()),
    redis: RedisConnectionInfo {
      username: Some(env::var("REDIS_USER").unwrap()),
      password: Some(env::var("REDIS_USER_PASSWORD").unwrap()),
      ..Default::default()
    },
  })?;

  Ok(Redis::new(client.get_connection()?))
}