//! Various constants used in the program. These are largely based in game UI
//! and are outside of user control.

/// What percentage of the screen (left to right) the code matrix starts at
pub(crate) const CODE_MATRIX_START_X_RATIO: f32 = 0.94;
/// What percentage of the screen (left to right) the code matrix ends at
pub(crate) const CODE_MATRIX_END_X_RATIO: f32 = 0.46;
/// What percentage of the screen (top to bottom) the code matrix starts at
pub(crate) const CODE_MATRIX_START_Y_RATIO: f32 = 0.32;
/// What percentage of the screen (top to bottom) the code matrix ends at
// I actually have no idea how far down the code matrix can go, but there's not
// much that below anyways so we only need to trim a little bit of garbage
pub(crate) const CODE_MATRIX_END_Y_RATIO: f32 = 0.86;
