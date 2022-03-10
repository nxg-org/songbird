#![allow(missing_docs, clippy::large_enum_variant)]

use crate::tracks::Track;

pub enum DisposalMessage {
    Track(Track),

    Poison,
}
