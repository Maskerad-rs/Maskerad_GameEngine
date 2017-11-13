//Will provide custom error and Result types :
//Error types :
// - If no low-lever cause : just fn description in Error trait and description as field,
// - Else : fn description + fn cause and description and cause as field.

// If multiple causes if failure : enum that encapsulate all possible errors.
pub mod error;