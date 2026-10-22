use std::{fmt::Write, sync::Arc};

use chrono::Utc;

use crate::{
    app::state::AppState,
    core::{action_error::ActionError, version::VERSION},
    features::uploads::{batch::Batch, types::uploader::Uploader},
    libs::{describe_device::describe_device, log_timestamp::log_timestamp},
};

pub fn create_batch(state: &AppState, uploader: &Uploader) -> Result<Arc<Batch>, ActionError> {
    let started_at = Utc::now();
    let batch = Arc::new(
        Batch::create(&state.config().data_path, started_at).map_err(ActionError::internal)?,
    );
    let session = state.session();
    let device = describe_device(&uploader.user_agent);

    let mut header = format!("QuickShare {VERSION}\n");
    let _ = writeln!(header, "executable sha256: {}", session.executable_sha256());
    let _ = writeln!(
        header,
        "session started:   {}",
        log_timestamp(session.started_at())
    );
    let _ = writeln!(header, "session token:     {}", session.token());
    let _ = writeln!(header, "batch started:     {}", log_timestamp(started_at));
    let _ = writeln!(header, "device:            {device}");
    let _ = writeln!(header, "ip:                {}", uploader.ip);
    let _ = writeln!(header, "user agent:        {}", uploader.user_agent);
    header.push('\n');

    batch.append(&header).map_err(ActionError::internal)?;

    session
        .batches()
        .insert(batch.id().to_string(), batch.clone());

    tracing::info!(folder = %batch.directory().display(), %device, ip = %uploader.ip, "batch started");

    Ok(batch)
}
