use castluma_core::{SessionEvent, SessionFailure};

/// Return the events for a simulated normal playback session.
pub fn normal_events() -> Vec<SessionEvent> {
    vec![
        SessionEvent::DiscoveryStarted,
        SessionEvent::ConnectionRequested,
        SessionEvent::ConnectionEstablished,
        SessionEvent::LoadRequested,
        SessionEvent::PlaybackStarted,
        SessionEvent::PlaybackPaused,
        SessionEvent::PlaybackStarted,
        SessionEvent::StopRequested,
        SessionEvent::PlaybackStopped,
    ]
}

/// Returns the evebts for a simulated connection timeout.
pub fn timeout_events() -> Vec<SessionEvent> {
    vec![
        SessionEvent::DiscoveryStarted,
        SessionEvent::ConnectionRequested,
        SessionEvent::ConnectionFailed(SessionFailure::ConnectionTimeout),
    ]
}
