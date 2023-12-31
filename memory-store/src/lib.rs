use std::{collections::HashMap, convert::Infallible, sync::Arc};

use async_trait::async_trait;
use parking_lot::Mutex;
use time::OffsetDateTime;
use tower_sessions_core::{
    session::{Id, Session},
    SessionStore,
};

/// A session store that lives only in memory.
///
/// This is useful for testing but not recommended for real applications.
///
/// # Examples
///
/// ```rust
/// use tower_sessions::MemoryStore;
/// MemoryStore::default();
/// ```
#[derive(Clone, Debug, Default)]
pub struct MemoryStore(Arc<Mutex<HashMap<Id, (Session, OffsetDateTime)>>>);

#[async_trait]
impl SessionStore for MemoryStore {
    type Error = Infallible;

    async fn save(&self, session: &Session) -> Result<(), Self::Error> {
        self.0
            .lock()
            .insert(*session.id(), (session.clone(), session.expiry_date()));
        Ok(())
    }

    async fn load(&self, session_id: &Id) -> Result<Option<Session>, Self::Error> {
        Ok(self
            .0
            .lock()
            .get(session_id)
            .filter(|(_, expiry_date)| is_active(*expiry_date))
            .map(|(session, _)| session)
            .cloned())
    }

    async fn delete(&self, session_id: &Id) -> Result<(), Self::Error> {
        self.0.lock().remove(session_id);
        Ok(())
    }
}

fn is_active(expiry_date: OffsetDateTime) -> bool {
    expiry_date > OffsetDateTime::now_utc()
}
