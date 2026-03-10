use crate::{SandboxContext, Timestamp, TimestampExtension};

impl SandboxContext {
    pub async fn timestamp(&self) -> Timestamp {
        self.worker()
            .view_block()
            .await
            .unwrap()
            .timestamp()
            .ns_to_sec()
    }

    pub async fn fast_forward(&self, expected_date: Timestamp) {
        let mut timestamp = self.timestamp().await;

        while expected_date > timestamp {
            self.worker()
                .fast_forward(expected_date - timestamp)
                .await
                .expect("Fast forward");

            timestamp = self.timestamp().await;
        }
    }
}
