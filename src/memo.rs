//! A memo associated with a Zcash shielded output.

use crate::{blob_envelope, data};

data!(Memo, "A memo associated with a Zcash shielded output.");

blob_envelope!(Memo);
