//! Rust implementation of the Time-Addressable Media Store (TAMS) API.
//!
//! See <https://tams.org/> and <https://github.com/bbc/tams> for the specification.

pub use tams_client::{Client, Error, Result};
pub use tams_core::{
    Flow, FlowSegment, FlowSegmentCreate, ObjectUrl, PutUrl, Service, Source, StorageAllocation,
    StorageAllocationRequest, StorageBackend, TimeRange, Timestamp,
};
