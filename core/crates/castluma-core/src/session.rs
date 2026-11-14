/// A typed operational failure reported by a session adapter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionFailure {
    /// The connection attempt exceeded its allowed here
    ConnectionTimeout,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionState {
    Idle,        // No session activity currently in progress
    Discovering, // Search for Receivers
    Connecting,  // Connection to the selected receiver
    Ready,       // Receiver connected, ready to load media
    Loading,     // Loading the requested media
    Playing,     // Now Playing
    Paused,      // Playing paused
    Stopping,    // Stopping playing
    Disconnected,
    Cancelled,
    /// The session failed with the recorded cause.
    Failed(SessionFailure),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionEvent {
    DiscoveryStarted,
    DiscoveryStopped,
    ConnectionRequested,
    ConnectionEstablished,
    LoadRequested,
    PlaybackStarted,
    PlaybackPaused,
    StopRequested,
    PlaybackStopped,
    MediaEnded,
    ConnectionLost,
    CancelRequested,
    /// The adapter reported a failed connection attempt.
    ConnectionFailed(SessionFailure),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InvalidTransition {
    pub state: SessionState, // Status of the event
    pub event: SessionEvent, // Event Not Allowed
}

impl SessionState {
    pub fn transition(self, event: SessionEvent) -> Result<Self, InvalidTransition> {
        match (self, event) {
            (Self::Idle, SessionEvent::DiscoveryStarted) => Ok(Self::Discovering),
            (Self::Discovering, SessionEvent::DiscoveryStopped) => Ok(Self::Idle),
            (Self::Discovering, SessionEvent::ConnectionRequested) => Ok(Self::Connecting),
            (Self::Connecting, SessionEvent::ConnectionEstablished) => Ok(Self::Ready),

            (Self::Ready, SessionEvent::LoadRequested) => Ok(Self::Loading),
            (Self::Loading, SessionEvent::PlaybackStarted) => Ok(Self::Playing),

            (Self::Playing, SessionEvent::PlaybackPaused) => Ok(Self::Paused),
            (Self::Paused, SessionEvent::PlaybackStarted) => Ok(Self::Playing),

            (Self::Playing, SessionEvent::StopRequested) => Ok(Self::Stopping),
            (Self::Paused, SessionEvent::StopRequested) => Ok(Self::Stopping),
            (Self::Stopping, SessionEvent::PlaybackStopped) => Ok(Self::Ready),

            (Self::Playing, SessionEvent::MediaEnded) => Ok(Self::Ready),
            (Self::Paused, SessionEvent::MediaEnded) => Ok(Self::Ready),

            (
                Self::Connecting
                | Self::Ready
                | Self::Loading
                | Self::Playing
                | Self::Paused
                | Self::Stopping,
                SessionEvent::ConnectionLost,
            ) => Ok(Self::Disconnected),
            (Self::Disconnected, SessionEvent::ConnectionLost) => Ok(Self::Disconnected),
            (Self::Disconnected, SessionEvent::DiscoveryStarted) => Ok(Self::Discovering),

            (
                Self::Discovering | Self::Connecting | Self::Loading,
                SessionEvent::CancelRequested,
            ) => Ok(Self::Cancelled),
            (Self::Cancelled, SessionEvent::CancelRequested) => Ok(Self::Cancelled),
            (Self::Cancelled, SessionEvent::DiscoveryStarted) => Ok(Self::Discovering),

            (Self::Connecting, SessionEvent::ConnectionFailed(failure)) => {
                Ok(Self::Failed(failure))
            }

            (Self::Failed(_), SessionEvent::DiscoveryStarted) => Ok(Self::Discovering),

            _ => Err(InvalidTransition { state: self, event }),
        }
    }
}

#[derive(Debug)]
pub struct CastSession {
    state: SessionState,
}

impl CastSession {
    pub fn new() -> Self {
        Self {
            state: SessionState::Idle,
        }
    }

    pub fn state(&self) -> SessionState {
        self.state
    }

    pub fn apply(&mut self, event: SessionEvent) -> Result<(), InvalidTransition> {
        let next_state = self.state.transition(event)?;
        self.state = next_state;
        Ok(())
    }
}

impl Default for CastSession {
    fn default() -> Self {
        Self::new()
    }
}
