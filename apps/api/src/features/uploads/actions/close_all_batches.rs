use crate::{app::state::AppState, features::uploads::actions::finish_batch::finish_batch};

pub fn close_all_batches(state: &AppState) {
    let batches: Vec<_> = state
        .session()
        .batches()
        .drain()
        .map(|(_, batch)| batch)
        .collect();

    for batch in batches {
        if let Err(error) = finish_batch(&batch) {
            tracing::error!(folder = %batch.directory().display(), %error, "cannot close the batch log");
        }
    }
}
