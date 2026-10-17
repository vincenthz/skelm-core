//! Bridge from generation events to an axum SSE response.
//!
//! A task forwards the generation's events as SSE events, through an unbounded
//! channel that backs the SSE stream.

use std::convert::Infallible;
use std::future::Future;

use axum::response::sse::{Event, KeepAlive, Sse};
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use tokio::sync::mpsc::{self, UnboundedSender};
use tokio_stream::wrappers::UnboundedReceiverStream;

/// Sink handed to the blocking generation body for emitting SSE events.
pub struct EventSender(UnboundedSender<Result<Event, Infallible>>);

impl EventSender {
    /// Serialize `value` as a JSON `data:` event. Returns `false` once the
    /// client has disconnected, which the caller can use to stop generating.
    pub fn send_json<T: Serialize>(&self, value: &T) -> bool {
        let data = serde_json::to_string(value).unwrap_or_default();
        self.0.send(Ok(Event::default().data(data))).is_ok()
    }

    /// Emit the terminating `data: [DONE]` marker.
    pub fn done(&self) {
        let _ = self.0.send(Ok(Event::default().data("[DONE]")));
    }
}

/// Run `body` in a task, streaming whatever it sends as an SSE response.
pub fn sse_stream<F, Fut>(body: F) -> Response
where
    F: FnOnce(EventSender) -> Fut,
    Fut: Future<Output = ()> + Send + 'static,
{
    let (tx, rx) = mpsc::unbounded_channel::<Result<Event, Infallible>>();
    tokio::spawn(body(EventSender(tx)));
    Sse::new(UnboundedReceiverStream::new(rx))
        .keep_alive(KeepAlive::default())
        .into_response()
}
