use std::fmt;
use std::str::FromStr;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

/// A timestamp in TAMS format (`seconds:nanoseconds`).
///
/// Nanosecond resolution. Seconds may be negative.
/// See [App Note 0008](https://github.com/bbc/tams/blob/main/docs/appnotes/0008-timestamps-in-TAMS.md).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Timestamp {
    pub seconds: i64,
    pub nanoseconds: u32,
}

impl Timestamp {
    pub fn new(seconds: i64, nanoseconds: u32) -> Self {
        assert!(nanoseconds < 1_000_000_000);
        Self {
            seconds,
            nanoseconds,
        }
    }

    /// Zero timestamp.
    pub fn zero() -> Self {
        Self::new(0, 0)
    }
}

impl fmt::Display for Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.seconds, self.nanoseconds)
    }
}

impl FromStr for Timestamp {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (sign, rest) = if let Some(stripped) = s.strip_prefix('-') {
            (-1i64, stripped)
        } else {
            (1i64, s)
        };
        let (secs, nanos) = rest
            .split_once(':')
            .ok_or_else(|| format!("invalid timestamp: {s}"))?;
        let seconds: i64 = secs
            .parse()
            .map_err(|_| format!("invalid seconds in timestamp: {s}"))?;
        let nanoseconds: u32 = nanos
            .parse()
            .map_err(|_| format!("invalid nanoseconds in timestamp: {s}"))?;
        if nanoseconds >= 1_000_000_000 {
            return Err(format!("nanoseconds out of range: {s}"));
        }
        Ok(Self {
            seconds: sign * seconds,
            nanoseconds,
        })
    }
}

impl Serialize for Timestamp {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Timestamp {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}

/// A half-open timerange `[start_end)`.
///
/// Represented in the API as `[start_end)` by default.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TimeRange {
    pub start: Timestamp,
    pub end: Timestamp,
}

impl TimeRange {
    pub fn new(start: Timestamp, end: Timestamp) -> Self {
        Self { start, end }
    }
}

impl fmt::Display for TimeRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}_{})", self.start, self.end)
    }
}

impl FromStr for TimeRange {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Simplified parser for the common `[start_end)` form
        let s = s.trim();
        let (start_part, end_part) = s
            .strip_prefix('[')
            .and_then(|rest| rest.strip_suffix(')'))
            .and_then(|inner| inner.split_once('_'))
            .ok_or_else(|| format!("invalid timerange: {s}"))?;

        let start = Timestamp::from_str(start_part)?;
        let end = Timestamp::from_str(end_part)?;
        Ok(Self { start, end })
    }
}

impl Serialize for TimeRange {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for TimeRange {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}

/// A Source represents abstract content that can be realized by one or more Flows.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Source {
    pub id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Map<String, Value>>,
}

/// A Flow is an immutable timeline of media grains.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Flow {
    pub id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timerange: Option<TimeRange>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Map<String, Value>>,
}

/// URL information for getting or putting a media object.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectUrl {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presigned: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_id: Option<Uuid>,
}

/// A Flow Segment maps media objects onto a portion of a Flow's timeline.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowSegment {
    pub timerange: TimeRange,
    pub object_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub get_urls: Option<Vec<ObjectUrl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ts_offset: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_frame_count: Option<u32>,
}

/// Request body for creating/registering a Flow Segment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowSegmentCreate {
    pub object_id: String,
    pub timerange: TimeRange,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ts_offset: Option<Timestamp>,
}

/// Request for storage allocation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageAllocationRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_ids: Option<Vec<String>>,
}

/// Response from storage allocation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageAllocation {
    pub put_urls: Vec<PutUrl>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PutUrl {
    pub object_id: String,
    pub put_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

/// Service information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Service {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

/// Storage backend information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageBackend {
    pub id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}
