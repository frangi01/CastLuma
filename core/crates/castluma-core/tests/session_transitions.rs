use castluma_core::{InvalidTransition, SessionEvent, SessionFailure, SessionState};

#[test]
fn idle_can_start_discovery() {
    let result = SessionState::Idle.transition(SessionEvent::DiscoveryStarted);
    assert_eq!(result, Ok(SessionState::Discovering));
}

#[test]
fn idle_rejects_connection_established() {
    let result = SessionState::Idle.transition(SessionEvent::ConnectionEstablished);
    assert_eq!(
        result,
        Err(InvalidTransition {
            state: SessionState::Idle,
            event: SessionEvent::ConnectionEstablished
        })
    );
}

#[test]
fn discovering_can_stop_discovery() {
    let result = SessionState::Discovering.transition(SessionEvent::DiscoveryStopped);
    assert_eq!(result, Ok(SessionState::Idle))
}

#[test]
fn discovering_can_request_connection() {
    let result = SessionState::Discovering.transition(SessionEvent::ConnectionRequested);
    assert_eq!(result, Ok(SessionState::Connecting))
}

#[test]
fn connecting_can_establish_connection() {
    let result = SessionState::Connecting.transition(SessionEvent::ConnectionEstablished);
    assert_eq!(result, Ok(SessionState::Ready))
}

#[test]
fn ready_can_request_load() {
    let result = SessionState::Ready.transition(SessionEvent::LoadRequested);
    assert_eq!(result, Ok(SessionState::Loading))
}

#[test]
fn loading_can_start_playback() {
    let result = SessionState::Loading.transition(SessionEvent::PlaybackStarted);
    assert_eq!(result, Ok(SessionState::Playing))
}

#[test]
fn idle_rejects_load_requested() {
    let result = SessionState::Idle.transition(SessionEvent::LoadRequested);
    assert_eq!(
        result,
        Err(InvalidTransition {
            state: SessionState::Idle,
            event: SessionEvent::LoadRequested
        })
    )
}

#[test]
fn idle_rejects_connection_requested() {
    let result = SessionState::Idle.transition(SessionEvent::ConnectionRequested);
    assert_eq!(
        result,
        Err(InvalidTransition {
            state: SessionState::Idle,
            event: SessionEvent::ConnectionRequested
        })
    )
}

#[test]
fn playing_can_pause() {
    let result = SessionState::Playing.transition(SessionEvent::PlaybackPaused);
    assert_eq!(result, Ok(SessionState::Paused))
}

#[test]
fn paused_can_resume() {
    let result = SessionState::Paused.transition(SessionEvent::PlaybackStarted);
    assert_eq!(result, Ok(SessionState::Playing))
}

#[test]
fn playing_can_request_stop() {
    let result = SessionState::Playing.transition(SessionEvent::StopRequested);
    assert_eq!(result, Ok(SessionState::Stopping))
}

#[test]
fn paused_can_request_stop() {
    let result = SessionState::Paused.transition(SessionEvent::StopRequested);
    assert_eq!(result, Ok(SessionState::Stopping))
}

#[test]
fn stopping_can_finish_stop() {
    let result = SessionState::Stopping.transition(SessionEvent::PlaybackStopped);
    assert_eq!(result, Ok(SessionState::Ready))
}

#[test]
fn ready_rejects_playback_paused() {
    let result = SessionState::Ready.transition(SessionEvent::PlaybackPaused);
    assert_eq!(
        result,
        Err(InvalidTransition {
            state: SessionState::Ready,
            event: SessionEvent::PlaybackPaused
        })
    )
}

#[test]
fn playing_can_end_media() {
    let result = SessionState::Playing.transition(SessionEvent::MediaEnded);
    assert_eq!(result, Ok(SessionState::Ready))
}

#[test]
fn paused_can_end_media() {
    let result = SessionState::Paused.transition(SessionEvent::MediaEnded);
    assert_eq!(result, Ok(SessionState::Ready))
}

#[test]
fn idle_rejects_media_ended() {
    let result = SessionState::Idle.transition(SessionEvent::MediaEnded);
    assert_eq!(
        result,
        Err(InvalidTransition {
            state: SessionState::Idle,
            event: SessionEvent::MediaEnded
        })
    )
}

#[test]
fn connection_loss_disconnects_active_session_states() {
    let states = [
        SessionState::Connecting,
        SessionState::Ready,
        SessionState::Loading,
        SessionState::Playing,
        SessionState::Paused,
        SessionState::Stopping,
    ];
    for state in states {
        assert_eq!(
            state.transition(SessionEvent::ConnectionLost),
            Ok(SessionState::Disconnected),
            "connection loss should disconnect {state:?}"
        )
    }
}

#[test]
fn disconnected_accepts_repeated_connection_loss() {
    let result = SessionState::Disconnected.transition(SessionEvent::ConnectionLost);
    assert_eq!(result, Ok(SessionState::Disconnected));
}

#[test]
fn disconnected_can_start_discovery() {
    let result = SessionState::Disconnected.transition(SessionEvent::DiscoveryStarted);
    assert_eq!(result, Ok(SessionState::Discovering));
}

#[test]
fn connection_loss_is_rejected_without_active_connection() {
    let states = [SessionState::Idle, SessionState::Discovering];
    for state in states {
        assert_eq!(
            state.transition(SessionEvent::ConnectionLost),
            Err(InvalidTransition {
                state,
                event: SessionEvent::ConnectionLost
            }),
            "connection loss should be rejected in {state:?}"
        )
    }
}

#[test]
fn pending_operations_can_be_cancelled() {
    let states = [
        SessionState::Discovering,
        SessionState::Connecting,
        SessionState::Loading,
    ];
    for state in states {
        assert_eq!(
            state.transition(SessionEvent::CancelRequested),
            Ok(SessionState::Cancelled),
            "cancellation should succeed in {state:?}"
        )
    }
}

#[test]
fn cancelled_accepts_repeated_cancellation() {
    let result = SessionState::Cancelled.transition(SessionEvent::CancelRequested);
    assert_eq!(result, Ok(SessionState::Cancelled));
}

#[test]
fn cancelled_can_start_discovery() {
    let result = SessionState::Cancelled.transition(SessionEvent::DiscoveryStarted);
    assert_eq!(result, Ok(SessionState::Discovering));
}

#[test]
fn playback_rejects_cancellation() {
    let states = [SessionState::Playing, SessionState::Paused];
    for state in states {
        assert_eq!(
            state.transition(SessionEvent::CancelRequested),
            Err(InvalidTransition {
                state,
                event: SessionEvent::CancelRequested
            })
        )
    }
}

#[test]
fn connecting_records_connection_failure() {
    let failure = SessionFailure::ConnectionTimeout;
    let result = SessionState::Connecting.transition(SessionEvent::ConnectionFailed(failure));
    assert_eq!(result, Ok(SessionState::Failed(failure)));
}

#[test]
fn failed_session_can_start_discovery() {
    let state = SessionState::Failed(SessionFailure::ConnectionTimeout);
    let result = state.transition(SessionEvent::DiscoveryStarted);
    assert_eq!(result, Ok(SessionState::Discovering));
}

#[test]
fn idle_rejects_connection_failure() {
    let event = SessionEvent::ConnectionFailed(SessionFailure::ConnectionTimeout);
    let result = SessionState::Idle.transition(event);
    assert_eq!(
        result,
        Err(InvalidTransition {
            state: SessionState::Idle,
            event
        })
    );
}
