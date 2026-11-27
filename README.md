# tams

![TAMS Banner](docs/banner.jpeg)

**Rust client and models for the [Time-Addressable Media Store (TAMS)](https://tams.org/) API.**

TAMS is an open specification from BBC Research & Development for storing and accessing segmented media by time rather than by file. Media lives on infinite timelines; any point can be addressed by a `(flow_id, timestamp)` pair.

[![crates.io](https://img.shields.io/crates/v/tams.svg)](https://crates.io/crates/tams)
[![docs.rs](https://docs.rs/tams/badge.svg)](https://docs.rs/tams)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)

## Features

- Async HTTP client covering the main TAMS workflows
- Strongly-typed models for Sources, Flows, Segments, Storage, and Service info
- Nanosecond-resolution `Timestamp` and `TimeRange` that serialize to the official TAMS string format
- Bearer token authentication support
- Workspace layout matching other vbasky crates

## Workspace

```
crates/
  tams/          # Facade — re-exports everything
  tams-core/     # Models, timestamps, timeranges (no I/O)
  tams-client/   # Async HTTP client
```

## Quick start

```toml
[dependencies]
tams = "0.1"
tokio = { version = "1", features = ["full"] }
```

```rust
use tams::{
    Client, Flow, FlowSegmentCreate, Source, StorageAllocationRequest,
    TimeRange, Timestamp,
};
use uuid::Uuid;

#[tokio::main]
async fn main() -> tams::Result<()> {
    // Connect (optionally with a bearer token)
    let client = Client::with_bearer_token(
        "https://example.com/tams/v8.0",
        "your-token",
    )?;

    // Service discovery
    let service = client.get_service().await?;
    let backends = client.list_storage_backends().await?;

    // Create a Source and Flow
    let source_id = Uuid::new_v4();
    let source = Source {
        id: source_id,
        label: Some("My Content".into()),
        description: None,
        created: None,
        tags: None,
    };
    client.put_source(&source).await?;

    let flow_id = Uuid::new_v4();
    let flow = Flow {
        id: flow_id,
        source_id: Some(source_id),
        label: Some("Video".into()),
        description: None,
        format: Some("video/mp4".into()),
        created: None,
        timerange: None,
        tags: None,
    };
    client.put_flow(&flow).await?;

    // Allocate storage and get upload URLs
    let alloc = client
        .allocate_storage(
            flow_id,
            &StorageAllocationRequest {
                limit: Some(1),
                object_ids: None,
            },
        )
        .await?;
    let put_url = &alloc.put_urls[0];
    // Upload your media bytes to put_url.put_url (direct to object storage)

    // Register the segment on the timeline
    let segment = FlowSegmentCreate {
        object_id: put_url.object_id.clone(),
        timerange: TimeRange::new(Timestamp::zero(), Timestamp::new(10, 0)),
        ts_offset: None,
    };
    client.create_segments(flow_id, &[segment]).await?;

    // Query segments by timerange
    let segments = client
        .list_segments(flow_id, Some("[0:0_60:0)"))
        .await?;
    for seg in segments {
        if let Some(urls) = &seg.get_urls {
            println!("Download: {}", urls[0].url);
        }
    }

    Ok(())
}
```

## Timestamp & TimeRange

Timestamps use the TAMS format `seconds:nanoseconds` (nanosecond resolution).

```rust
use tams::{Timestamp, TimeRange};
use std::str::FromStr;

let ts = Timestamp::new(90, 500_000_000);
assert_eq!(ts.to_string(), "90:500000000");

let range = TimeRange::from_str("[0:0_10:0)").unwrap();
```

## Supported operations

| Area | Methods |
|------|---------|
| Service | `get_service`, `list_storage_backends` |
| Sources | `get_source`, `list_sources`, `put_source` |
| Flows | `get_flow`, `list_flows`, `put_flow` |
| Segments | `list_segments`, `create_segments` |
| Storage | `allocate_storage` |

## Status

Usable for the core TAMS workflows (service discovery, source/flow management, storage allocation, segment registration and query). Additional endpoints (webhooks, deletion requests, tags, pagination helpers) can be added as needed.

## License

Apache-2.0

## Links

- [TAMS Website](https://tams.org/)
- [Official Spec](https://github.com/bbc/tams)
- [Rendered API Docs](https://bbc.github.io/tams)
- [App Note 0008 — Timestamps](https://github.com/bbc/tams/blob/main/docs/appnotes/0008-timestamps-in-TAMS.md)
