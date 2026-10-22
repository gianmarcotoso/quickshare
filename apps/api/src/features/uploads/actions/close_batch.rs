use crate::{
    app::state::AppState,
    core::{action_error::ActionError, error_code::ErrorCode},
    features::uploads::actions::finish_batch::finish_batch,
};

pub fn close_batch(state: &AppState, id: &str) -> Result<(), ActionError> {
    let batch = state
        .session()
        .batches()
        .remove(id)
        .ok_or(ActionError::NotFound(ErrorCode::BatchNotFound))?;

    finish_batch(&batch).map_err(ActionError::internal)
}
