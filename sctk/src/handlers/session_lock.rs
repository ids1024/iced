use crate::{handlers::SctkState, sctk_event::SctkEvent};
use sctk::{
    delegate_session_lock,
    reexports::client::{Connection, QueueHandle},
    session_lock::{
        SessionLock, SessionLockHandler, SessionLockSurface,
        SessionLockSurfaceConfigure,
    },
};
use std::fmt::Debug;

impl<T: 'static + Debug> SessionLockHandler for SctkState<T> {
    fn locked(
        &mut self,
        _conn: &Connection,
        qh: &QueueHandle<Self>,
        session_lock: SessionLock,
    ) {
        self.sctk_events.push(SctkEvent::SessionLocked);
    }

    fn finished(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _session_lock: SessionLock,
    ) {
        self.sctk_events.push(SctkEvent::SessionLockFinished);
    }

    fn configure(
        &mut self,
        _conn: &Connection,
        qh: &QueueHandle<Self>,
        session_lock_surface: SessionLockSurface,
        configure: SessionLockSurfaceConfigure,
        _serial: u32,
    ) {
        self.sctk_events
            .push(SctkEvent::SessionLockSurfaceConfigure {
                id: session_lock_surface.wl_surface().clone(),
                configure,
            });
        self.frame_events
            .push(session_lock_surface.wl_surface().clone());
    }
}

delegate_session_lock!(@<T: 'static + Debug> SctkState<T>);
