pub use self::edit::*;

pub mod edit;
mod helpers;

/*
Having pub use self::edit::*; and pub mod edit;
allows me to do the following:

use text_distance::edit::Levenshtein;
use text_distance::Levenshtein;
*/
