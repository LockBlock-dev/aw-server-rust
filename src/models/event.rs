use serde_json::Value;
use chrono::DateTime;
use chrono::Utc;
use chrono::Duration;

use models::duration::DurationSerialization;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Event {
    pub id: Option<i64>,
    pub timestamp: DateTime<Utc>,
    #[serde(with = "DurationSerialization")]
    pub duration: Duration,
    pub data: Value, /* TODO: force this to be a value::Object somehow */
}

impl Event {
    pub fn calculate_endtime(&self) -> DateTime<Utc> {
        self.timestamp + chrono::Duration::nanoseconds(self.duration.num_nanoseconds().unwrap() as i64)
    }
}
