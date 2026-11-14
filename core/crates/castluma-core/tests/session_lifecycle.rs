use castluma_core::{CastSession, InvalidTransition, SessionEvent, SessionFailure, SessionState};

#[test]
fn session_starts_idle() {
    let session = CastSession::new();
    assert_eq!(session.state(), SessionState::Idle);
}

#[test]
fn session_applies_valid_event() {
    let mut session = CastSession::new();

    let result = session.apply(SessionEvent::DiscoveryStarted);

    assert_eq!(result, Ok(()));
    assert_eq!(session.state(), SessionState::Discovering);
}

#[test]
fn session_preserves_state_after_invalid_event() {
    let mut session = CastSession::new();

    let result = session.apply(SessionEvent::LoadRequested);

    assert_eq!(
        result,
        Err(InvalidTransition {
            state: SessionState::Idle,
            event: SessionEvent::LoadRequested
        })
    );
    assert_eq!(session.state(), SessionState::Idle);
}

#[test]
fn session_complete_normal_playback_flow() {
    let mut session = CastSession::new();

    let steps = [
        (SessionEvent::DiscoveryStarted, SessionState::Discovering),
        (SessionEvent::ConnectionRequested, SessionState::Connecting),
        (SessionEvent::ConnectionEstablished, SessionState::Ready),
        (SessionEvent::LoadRequested, SessionState::Loading),
        (SessionEvent::PlaybackStarted, SessionState::Playing),
        (SessionEvent::PlaybackPaused, SessionState::Paused),
        (SessionEvent::PlaybackStarted, SessionState::Playing),
        (SessionEvent::StopRequested, SessionState::Stopping),
        (SessionEvent::PlaybackStopped, SessionState::Ready),
    ];

    for (event, expected_state) in steps {
        assert_eq!(
            session.apply(event),
            Ok(()),
            "event {event:?} should be accepted"
        );
        assert_eq!(
            session.state(),
            expected_state,
            "unexpected state after {event:?}"
        );
    }
}

#[test]
fn session_can_continue_after_rejected_event() {
    let mut session = CastSession::new();
    assert_eq!(session.apply(SessionEvent::DiscoveryStarted), Ok(()));
    assert_eq!(
        session.apply(SessionEvent::LoadRequested),
        Err(InvalidTransition {
            state: SessionState::Discovering,
            event: SessionEvent::LoadRequested
        })
    );
    assert_eq!(session.state(), SessionState::Discovering);
    assert_eq!(session.apply(SessionEvent::ConnectionRequested), Ok(()));
    assert_eq!(session.state(), SessionState::Connecting);
}

#[test]
fn session_can_restart_discovery_after_connection_failure() {
    let mut session = CastSession::new();
    let failure = SessionFailure::ConnectionTimeout;

    assert_eq!(session.apply(SessionEvent::DiscoveryStarted), Ok(()));
    assert_eq!(session.apply(SessionEvent::ConnectionRequested), Ok(()));
    assert_eq!(
        session.apply(SessionEvent::ConnectionFailed(failure)),
        Ok(())
    );
    assert_eq!(session.state(), SessionState::Failed(failure));
    assert_eq!(session.apply(SessionEvent::DiscoveryStarted), Ok(()));
    assert_eq!(session.state(), SessionState::Discovering);
}
