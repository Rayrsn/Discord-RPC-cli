use std::time::Duration;

use user_idle::UserIdle;

#[derive(Debug)]
pub(crate) enum IdleState {
    Active,
    Idle(Duration),
}

/// Provider that monitors user idle state across Wayland (ext-idle-notify-v1),
/// DBus (GNOME / KDE), X11, and OS-native APIs.
pub(crate) struct IdleMonitor {
    #[cfg(target_os = "linux")]
    wayland: Option<wayland::WaylandIdleTracker>,
}

impl IdleMonitor {
    pub(crate) fn new(_threshold: Duration) -> Self {
        #[cfg(target_os = "linux")]
        let wayland = wayland::WaylandIdleTracker::start(_threshold);
        Self {
            #[cfg(target_os = "linux")]
            wayland,
        }
    }

    /// Checks the current idle state. Returns `IdleState::Idle(elapsed)` if the user
    /// has been idle for at least the threshold, or `IdleState::Active` otherwise.
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub(crate) fn check_idle(&mut self, threshold: Duration) -> IdleState {
        if threshold == Duration::ZERO {
            return IdleState::Idle(Duration::ZERO);
        }
        #[cfg(target_os = "linux")]
        if let Some(wayland) = &mut self.wayland {
            if let Some(state) = wayland.get_state(threshold) {
                return state;
            }
        }

        // 2. Try GNOME Mutter DBus if available on Linux
        #[cfg(target_os = "linux")]
        if let Ok(idle_duration) = get_gnome_mutter_idle() {
            if idle_duration >= threshold {
                return IdleState::Idle(idle_duration);
            } else {
                return IdleState::Active;
            }
        }

        // 3. Fall back to user_idle crate (DBus / X11 / Windows / macOS native)
        if let Ok(idle) = UserIdle::get_time() {
            let duration = Duration::from_secs(idle.as_seconds());
            if duration >= threshold {
                return IdleState::Idle(duration);
            } else {
                return IdleState::Active;
            }
        }

        // 4. Default to Active if no idle system is reachable
        IdleState::Active
    }
}

#[cfg(target_os = "linux")]
fn get_gnome_mutter_idle() -> Result<Duration, ()> {
    use dbus::blocking::Connection;
    let conn = Connection::new_session().map_err(|_| ())?;
    let proxy = conn.with_proxy(
        "org.gnome.Mutter.IdleMonitor",
        "/org/gnome/Mutter/IdleMonitor/Core",
        Duration::from_millis(500),
    );
    let (time_ms,): (u64,) = proxy
        .method_call("org.gnome.Mutter.IdleMonitor", "GetIdletime", ())
        .map_err(|_| ())?;
    Ok(Duration::from_millis(time_ms))
}

// ---------------------------------------------------------------------------
// Wayland ext-idle-notify-v1 implementation
// ---------------------------------------------------------------------------

#[cfg(target_os = "linux")]
mod wayland {
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::time::{Duration, Instant};

    use wayland_client::protocol::wl_registry;
    use wayland_client::protocol::wl_seat::WlSeat;
    use wayland_client::{Connection, Dispatch, EventQueue, QueueHandle};
    use wayland_protocols::ext::idle_notify::v1::client::ext_idle_notification_v1::{
        self, ExtIdleNotificationV1,
    };
    use wayland_protocols::ext::idle_notify::v1::client::ext_idle_notifier_v1::ExtIdleNotifierV1;

    use super::IdleState;

    struct WaylandState {
        seat: Option<WlSeat>,
        notifier: Option<ExtIdleNotifierV1>,
        notification: Option<ExtIdleNotificationV1>,
        is_idle: Arc<AtomicBool>,
        idle_since: Arc<parking_lot::Mutex<Option<Instant>>>,
    }

    pub(super) struct WaylandIdleTracker {
        conn: Connection,
        event_queue: EventQueue<WaylandState>,
        state: WaylandState,
    }

    impl WaylandIdleTracker {
        pub(super) fn start(threshold: Duration) -> Option<Self> {
            if std::env::var_os("WAYLAND_DISPLAY").is_none()
                && std::env::var_os("WAYLAND_SOCKET").is_none()
            {
                return None;
            }

            let conn = Connection::connect_to_env().ok()?;
            let display = conn.display();
            let mut event_queue = conn.new_event_queue();
            let qh = event_queue.handle();

            let is_idle = Arc::new(AtomicBool::new(false));
            let idle_since = Arc::new(parking_lot::Mutex::new(None));

            let mut state = WaylandState {
                seat: None,
                notifier: None,
                notification: None,
                is_idle: is_idle.clone(),
                idle_since: idle_since.clone(),
            };

            let _registry = display.get_registry(&qh, ());

            // Initial roundtrip to get registry globals
            event_queue.roundtrip(&mut state).ok()?;

            let seat = state.seat.as_ref()?;
            let notifier = state.notifier.as_ref()?;

            let timeout_ms = threshold.as_millis().max(1).min(u32::MAX as u128) as u32;
            let notification = notifier.get_idle_notification(timeout_ms, seat, &qh, ());
            state.notification = Some(notification);

            let _ = conn.flush();

            Some(Self {
                conn,
                event_queue,
                state,
            })
        }

        pub(super) fn get_state(&mut self, threshold: Duration) -> Option<IdleState> {
            // Dispatch pending wayland events non-blockingly
            let _ = self.event_queue.dispatch_pending(&mut self.state);
            let _ = self.conn.flush();

            let is_idle = self.state.is_idle.load(Ordering::Acquire);
            if is_idle {
                let mut guard = self.state.idle_since.lock();
                let start = *guard.get_or_insert_with(Instant::now);
                let elapsed = threshold + start.elapsed();
                Some(IdleState::Idle(elapsed))
            } else {
                *self.state.idle_since.lock() = None;
                Some(IdleState::Active)
            }
        }
    }

    impl Dispatch<wl_registry::WlRegistry, ()> for WaylandState {
        fn event(
            state: &mut Self,
            registry: &wl_registry::WlRegistry,
            event: wl_registry::Event,
            _: &(),
            _: &Connection,
            qh: &QueueHandle<Self>,
        ) {
            if let wl_registry::Event::Global {
                name,
                interface,
                version,
            } = event
            {
                if interface == "wl_seat" && state.seat.is_none() {
                    state.seat = Some(registry.bind(name, version.min(1), qh, ()));
                } else if interface == "ext_idle_notifier_v1" && state.notifier.is_none() {
                    state.notifier = Some(registry.bind(name, version.min(1), qh, ()));
                }
            }
        }
    }

    impl Dispatch<WlSeat, ()> for WaylandState {
        fn event(
            _: &mut Self,
            _: &WlSeat,
            _: wayland_client::protocol::wl_seat::Event,
            _: &(),
            _: &Connection,
            _: &QueueHandle<Self>,
        ) {
        }
    }

    impl Dispatch<ExtIdleNotifierV1, ()> for WaylandState {
        fn event(
            _: &mut Self,
            _: &ExtIdleNotifierV1,
            _: wayland_protocols::ext::idle_notify::v1::client::ext_idle_notifier_v1::Event,
            _: &(),
            _: &Connection,
            _: &QueueHandle<Self>,
        ) {
        }
    }

    impl Dispatch<ExtIdleNotificationV1, ()> for WaylandState {
        fn event(
            state: &mut Self,
            _: &ExtIdleNotificationV1,
            event: ext_idle_notification_v1::Event,
            _: &(),
            _: &Connection,
            _: &QueueHandle<Self>,
        ) {
            match event {
                ext_idle_notification_v1::Event::Idled => {
                    state.is_idle.store(true, Ordering::Release);
                    let mut guard = state.idle_since.lock();
                    if guard.is_none() {
                        *guard = Some(Instant::now());
                    }
                }
                ext_idle_notification_v1::Event::Resumed => {
                    state.is_idle.store(false, Ordering::Release);
                    *state.idle_since.lock() = None;
                }
                _ => {}
            }
        }
    }
}
