use redis::RedisResult;
use std::time::Duration;

use crate::new_io_error;

pub struct Connection {
    con: redis::Connection,
}

pub struct Subscriber<'a> {
    pubsub: redis::PubSub<'a>,
}

impl Connection {
    pub fn new(redis_uri: &str) -> std::io::Result<Self> {
        let client = redis::Client::open(redis_uri)
            .map_err(|e| new_io_error!(std::io::ErrorKind::Other, e.to_string()))?;
        let con = client
            .get_connection_with_timeout(std::time::Duration::from_secs(3))
            .map_err(|e| new_io_error!(std::io::ErrorKind::Other, e.to_string()))?;

        Ok(Connection { con })
    }

    pub fn subscribe(&mut self, channels: &Vec<String>) -> std::io::Result<Subscriber> {
        let mut pubsub = self.con.as_pubsub();
        for channel in channels {
            tracing::info!("Subscribe to {}", channel);
            pubsub
                .psubscribe(channel)
                .map_err(|e| new_io_error!(std::io::ErrorKind::Other, e.to_string()))?;
        }
        pubsub.set_read_timeout(Some(Duration::from_secs(2))).ok();

        Ok(Subscriber { pubsub })
    }
}

impl Subscriber<'_> {
    pub fn next_blocking(&mut self) -> std::io::Result<Option<(String, String)>> {
        let msg = match self.pubsub.get_message() {
            Ok(msg) => msg,
            Err(e) => {
                if e.is_timeout() {
                    tracing::debug!("Timeout");
                    return Ok(None);
                }

                tracing::error!(
                    "{:?} is_io_error={:?} is_connection_dropped={:?} is_connection_refusal={:?}",
                    e,
                    e.is_io_error(),
                    e.is_connection_dropped(),
                    e.is_connection_refusal()
                );

                if e.is_io_error() {
                    return Err(new_io_error!(std::io::ErrorKind::Other, e.to_string()));
                }

                return Ok(None);
            }
        };

        let payload: RedisResult<String> = msg.get_payload();
        let channel: RedisResult<String> = msg.get_channel();
        if payload.is_err() || channel.is_err() {
            return Ok(None);
        }

        let payload = payload.unwrap();
        let channel = channel.unwrap();
        Ok(Some((channel, payload)))
    }
}
