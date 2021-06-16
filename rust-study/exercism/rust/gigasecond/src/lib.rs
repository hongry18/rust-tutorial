use chrono::{DateTime, Duration, Utc};

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    // 그냥 + operation 을 이용해 하는 방법
    start + Duration::seconds(1_000_000_000)
}
