//! In-process, typed publish/subscribe event bus. Roughly Spring's
//! `ApplicationEventPublisher` + `@EventListener`, built on [`tokio::sync::broadcast`].
//!
//! One [`EventBus`] carries any number of event types. Each event type gets its own
//! broadcast channel, created lazily on first `publish` or `subscribe`.
//!
//! ```ignore
//! use crate::event_bus::EventBus;
//!
//! #[derive(Clone, Debug, PartialEq)]
//! struct OrderPlaced { id: u64 }
//!
//! # tokio::runtime::Builder::new_current_thread().build().unwrap().block_on(async {
//! let bus = EventBus::new();
//! let mut rx = bus.subscribe::<OrderPlaced>();
//! bus.publish(OrderPlaced { id: 7 });
//! assert_eq!(rx.recv().await.unwrap(), OrderPlaced { id: 7 });
//! # });
//! ```ignore
//!
//! Semantics worth knowing, which differ from Spring:
//! - Delivery is asynchronous; `publish` never waits for listeners.
//! - Only receivers that exist at publish time get the event. Events published before
//!   anyone subscribes are dropped.
//! - Each event is cloned once per receiver.
//! - A receiver that falls more than `capacity` events behind gets
//!   [`RecvError::Lagged`] and loses the oldest events.

use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use tokio::sync::broadcast;
pub use tokio::sync::broadcast::Receiver;
pub use tokio::sync::broadcast::error::RecvError;

/// Default per-event-type buffer size.
pub const DEFAULT_CAPACITY: usize = 1024;

/// Anything that can travel on the bus.
pub trait Event: Clone + Send + Sync + 'static {}
impl<T: Clone + Send + Sync + 'static> Event for T {}

/// Cheap to clone; clones share the same channels.
#[derive(Clone)]
pub struct EventBus {
    capacity: usize,
    channels: Arc<RwLock<HashMap<TypeId, Box<dyn Any + Send + Sync>>>>,
}

impl EventBus {
    /// Bus with [`DEFAULT_CAPACITY`] per event type.
    pub fn new() -> Self {
        Self::with_capacity(DEFAULT_CAPACITY)
    }

    /// Bus buffering up to `capacity` events per event type.
    ///
    /// # Panics
    ///
    /// If `capacity` is `0`.
    pub fn with_capacity(capacity: usize) -> Self {
        assert!(capacity > 0, "event bus capacity must be greater than 0");
        Self {
            capacity,
            channels: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Sends `event` to every current subscriber of `E`.
    ///
    /// Returns the number of receivers it was delivered to. Having no subscribers is not
    /// an error: the event is dropped and `0` is returned.
    pub fn publish<E: Event>(&self, event: E) -> usize {
        self.sender::<E>().send(event).unwrap_or(0)
    }

    /// Starts receiving events of type `E` published from now on.
    pub fn subscribe<E: Event>(&self) -> Receiver<E> {
        self.sender::<E>().subscribe()
    }

    /// Number of live receivers for `E`.
    pub fn subscriber_count<E: Event>(&self) -> usize {
        self.sender::<E>().receiver_count()
    }

    fn sender<E: Event>(&self) -> broadcast::Sender<E> {
        let key = TypeId::of::<E>();
        if let Some(tx) = self.channels.read().unwrap().get(&key) {
            return downcast::<E>(tx).clone();
        }
        let mut channels = self.channels.write().unwrap();
        let tx = channels
            .entry(key)
            .or_insert_with(|| Box::new(broadcast::channel::<E>(self.capacity).0));
        downcast::<E>(tx).clone()
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}

fn downcast<E: Event>(tx: &Box<dyn Any + Send + Sync>) -> &broadcast::Sender<E> {
    tx.downcast_ref::<broadcast::Sender<E>>()
        .expect("channel map is keyed by TypeId, so the type always matches")
}
