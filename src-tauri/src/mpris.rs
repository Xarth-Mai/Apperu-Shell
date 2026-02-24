use crate::state::PlayerState;
use std::sync::Arc;
use tokio::sync::RwLock;

pub async fn init_mpris(_shared: Arc<RwLock<PlayerState>>) -> zbus::Result<()> {
    // MVP skeleton: reserve DBus connection and keep this module as integration point.
    // Replace with a full org.mpris.MediaPlayer2 + Player interface implementation.
    let _conn = zbus::Connection::session().await?;
    Ok(())
}
