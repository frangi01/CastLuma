//! Platform-idempendent domain models and session rules for CastLuma.

mod device;
mod media;
mod session;

pub use device::{Device, TransportKind};
pub use media::{Media, MediaSource};
pub use session::{CastSession, InvalidTransition, SessionEvent, SessionFailure, SessionState};
