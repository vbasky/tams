use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A timestamp in TAMS format (seconds:nanoseconds).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Timestamp {
    pub seconds: i64,
    pub nanoseconds: u32,
}

impl Timestamp {
    pub fn new(seconds: i64, nanoseconds: u32) -> Self {
        Self {
            seconds,
            nanoseconds,
        }
    }
}

/// A half-open timerange `[start_end)`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TimeRange {
    pub start: Timestamp,
    pub end: Timestamp,
}

/// A Source represents abstract content that can be realized by one or more Flows.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Source {
    pub id: Uuid,
    pub label: Option<String>,
    pub description: Option<String>,
    pub created: Option<DateTime<Utc>>,
}

/// A Flow is an immutable timeline of media grains.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Flow {
    pub id: Uuid,
    pub source_id: Option<Uuid>,
    pub label: Option<String>,
    pub description: Option<String>,
    pub format: Option<String>,
    pub created: Option<DateTime<Utc>>,
}

/// A Flow Segment maps media objects onto a portion of a Flow's timeline.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowSegment {
    pub timerange: TimeRange,
    pub object_id: Uuid,
}
