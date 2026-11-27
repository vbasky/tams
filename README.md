# tams

![TAMS Banner](docs/banner.jpeg)

**Rust client and models for the [Time-Addressable Media Store (TAMS)](https://tams.org/) API.**

TAMS is an open specification from BBC Research & Development for storing and accessing segmented media by time rather than by file. Media lives on infinite timelines; any point can be addressed by a `(flow_id, timestamp)` pair.

[![crates.io](https://img.shields.io/crates/v/tams.svg)](https://crates.io/crates/tams)
[![docs.rs](https://docs.rs/tams/badge.svg)](https://docs.rs/tams)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)

## Features

- Async HTTP client for the TAMS API
- Strongly-typed models for Sources, Flows, and Flow Segments
- Nanosecond-resolution `Timestamp` and `TimeRange` types that serialize to the official TAMS string format
- Workspace layout matching other vbasky crates (Revelo, Viser, etc.)

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
use tams::{Client, Timestamp, TimeRange};
use uuid::Uuid;

#[tokio::main]
async fn main() -> tams::Result<()> {
    let client = Client::new("https://example.com/tams/v8.0")?;

    let flow_id = Uuid::parse_str("...")?;
    let flow = client.get_flow(flow_id).await?;
    println!("{:?}", flow);

    let segments = client
        .list_segments(flow_id, Some("[0:0_60:0)"))
        .await?;
    for seg in segments {
        println!("{} -> {}", seg.timerange, seg.object_id);
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

## Status

Early stage. Core models and basic client methods (get/list sources, flows, and segments) are implemented. More endpoints (storage allocation, segment registration, webhooks, etc.) are planned.

## License

Apache-2.0

## Links

- [TAMS Website](https://tams.org/)
- [Official Spec](https://github.com/bbc/tams)
- [Rendered API Docs](https://bbc.github.io/tams)
- [App Note 0008 — Timestamps](https://github.com/bbc/tams/blob/main/docs/appnotes/0008-timestamps-in-TAMS.md)
