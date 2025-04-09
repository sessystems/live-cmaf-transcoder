use async_trait::async_trait;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::sync::mpsc::{self, Receiver, Sender};
use tokio::task::JoinHandle;

use super::subscriber::Connection;

pub struct SubscriberManager {
    handle: Option<JoinHandle<()>>,
    sender: Sender<EventType>,
}

#[derive(Debug)]
pub struct Event {
    pub channel: String,
    pub payload: String,
}

enum EventType {
    Event(Event),
    Disconnected,
    Reconnect,
    Quit,
}

#[async_trait]
pub trait EventHandler: Send {
    async fn on_event(&mut self, evt: &Event);
}

impl SubscriberManager {
    pub fn new(
        redis_conn_url: &str,
        event_handler: Box<dyn EventHandler>,
        channels: Vec<String>,
    ) -> std::io::Result<Self> {
        let (sender, receiver) = mpsc::channel(100);
        let sender_run = sender.clone();
        let redis_conn_url = redis_conn_url.to_owned();
        let handle = tokio::spawn(async move {
            SubscriberManager::run(
                &redis_conn_url,
                sender_run,
                receiver,
                event_handler,
                channels,
            )
            .await;
        });

        Ok(SubscriberManager {
            handle: Some(handle),
            sender,
        })
    }

    fn spawn_subscribe(
        redis_conn_url: &str,
        channels: Vec<String>,
        must_stop: Arc<AtomicBool>,
        sender: Sender<EventType>,
    ) -> std::io::Result<JoinHandle<()>> {
        let mut connection = Connection::new(redis_conn_url)?;
        let handle = tokio::task::spawn_blocking(move || {
            let mut subscriber = match connection.subscribe(&channels) {
                Ok(subscriber) => subscriber,
                Err(_) => {
                    tracing::error!("Failed to subscribe to redis");
                    sender.try_send(EventType::Disconnected).ok();
                    return;
                }
            };

            tracing::info!("Subscribed to redis !");
            loop {
                let e = subscriber.next_blocking();
                if must_stop.load(Ordering::Relaxed) {
                    break;
                }

                let (channel, payload) = match e {
                    Ok(Some(event)) => event,
                    Ok(None) => continue,
                    Err(_) => {
                        sender.try_send(EventType::Disconnected).ok();
                        break;
                    }
                };

                sender
                    .try_send(EventType::Event(Event { channel, payload }))
                    .ok();
            }
        });
        Ok(handle)
    }

    async fn run(
        redis_conn_url: &str,
        sender: Sender<EventType>,
        mut receiver: Receiver<EventType>,
        mut event_handler: Box<dyn EventHandler>,
        channels: Vec<String>,
    ) {
        let must_stop_subscribe: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
        let mut subscribe_handle: Option<JoinHandle<()>> = None;

        loop {
            if subscribe_handle.is_none() {
                subscribe_handle = Self::spawn_subscribe(
                    redis_conn_url,
                    channels.clone(),
                    must_stop_subscribe.clone(),
                    sender.clone(),
                )
                .ok();

                if subscribe_handle.is_none() {
                    tracing::error!("Failed to subscribe to redis, will retry in 2 seconds...");
                    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                    sender.send(EventType::Reconnect).await.ok();
                }
            }

            match receiver.recv().await {
                Some(EventType::Event(evt)) => event_handler.on_event(&evt).await,
                Some(EventType::Quit) => {
                    must_stop_subscribe.store(true, Ordering::Relaxed);
                    if let Some(handle) = subscribe_handle.take() {
                        handle.await.ok();
                    }
                    return;
                }
                Some(EventType::Disconnected) => {
                    if let Some(handle) = subscribe_handle.take() {
                        handle.await.ok();
                    }
                }
                Some(EventType::Reconnect) => {
                    tracing::info!("Reconnect Event");
                }
                None => {}
            };
        }
    }

    pub async fn stop(&mut self) {
        if self.handle.is_none() {
            return;
        }

        self.sender.send(EventType::Quit).await.ok();
        let handle = self.handle.take().unwrap();
        handle.await.ok();
    }
}
