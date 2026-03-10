use crate::{Timestamp, TimestampExtension};

const MS_DECIMALS: u64 = 1_000;
const NS_DECIMALS: u64 = 1_000_000_000;

impl TimestampExtension for Timestamp {
    fn sec_to_ms(self) -> Timestamp {
        self * MS_DECIMALS
    }

    fn ns_to_sec(self) -> Timestamp {
        self / NS_DECIMALS
    }
}
