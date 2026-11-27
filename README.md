# tams

![TAMS Banner](docs/banner.jpeg)

Rust client and models for the [Time-addressable Media Store (TAMS)](https://github.com/bbc/tams) API.

TAMS is an open API specification developed by BBC Research & Development for storing, querying and accessing segmented media. Media is addressed by time on an infinite timeline rather than by file. Once written, media and its association with the timeline are immutable.

[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)

## What this crate provides

- Async HTTP client for the TAMS API
- Models for Sources, Flows, Flow Segments, Profiles, Webhooks, Objects and Service metadata
- `Timestamp` and `TimeRange` types that serialize to the official TAMS string format
- Bearer token authentication

This is a client library. It does not implement a TAMS server.

## Core concepts

From the official specification:

- **Flows** exist on an infinite timeline and are immutable. A Flow ID plus a timerange always refers to the same sequence of media.
- **Sources** represent abstract content that can be realized by one or more Flows (for example a mezzanine and a proxy of the same programme).
- **Flow Segments** map Media Objects onto a portion of a Flow timeline. Grains (frames, audio samples) are grouped into segments, typically in a container such as MPEG-TS.
- **Media Objects** are the actual essence stored in object storage. The API returns download URLs; clients fetch the media directly.
- Any point on a Flow can be uniquely addressed by a `<flow_id, timestamp>` tuple.

See the [official README](https://github.com/bbc/tams#readme) and [rendered specification](https://bbc.github.io/tams) for the full model.

## Reading and writing

**Read**

1. Identify the Flow ID and timerange of interest.
2. Call `list_segments` with that timerange.
3. Download the returned URLs and concatenate the segments.
4. Skip any grains outside the requested timerange (the first and last segments may contain extra grains).

**Write**

1. Create a Flow with `put_flow` if it does not already exist.
2. Call `allocate_storage` to receive PUT URLs.
3. Upload the media data directly to those URLs.
4. Call `create_segments` to register each segment on the Flow timeline.

## Example

```rust
use tams::{Client, Flow, FlowSegmentCreate, StorageAllocationRequest, TimeRange, Timestamp};
use uuid::Uuid;

#[tokio::main]
async fn main() -> tams::Result<()> {
    let client = Client::with_bearer_token("https://example.com/tams/v8.2", "token")?;

    let flow_id = Uuid::new_v4();
    let flow = Flow {
        id: flow_id,
        source_id: None,
        label: Some("Programme".into()),
        description: None,
        format: Some("video/mp2t".into()),
        created: None,
        timerange: None,
        tags: None,
        status: None,
        read_only: None,
        avg_bit_rate: None,
        max_bit_rate: None,
    };
    client.put_flow(&flow).await?;

    let alloc = client
        .allocate_storage(
            flow_id,
            &StorageAllocationRequest {
                limit: Some(1),
                object_ids: None,
            },
        )
        .await?;

    // Upload media bytes to alloc.put_urls[0].put_url

    client
        .create_segments(
            flow_id,
            &[FlowSegmentCreate {
                object_id: alloc.put_urls[0].object_id.clone(),
                timerange: TimeRange::new(Timestamp::zero(), Timestamp::new(10, 0)),
                ts_offset: None,
            }],
        )
        .await?;

    let segments = client.list_segments(flow_id, Some("[0:0_60:0)")).await?;
    for seg in segments {
        if let Some(urls) = &seg.get_urls {
            println!("{}", urls[0].url);
        }
    }

    Ok(())
}
```

## Workspace

```
crates/
  tams/          # Facade
  tams-core/     # Models and types
  tams-client/   # HTTP client
```

## Links

- [Official TAMS repository](https://github.com/bbc/tams)
- [TAMS website](https://timeaddressablemediastore.org/)
- [Rendered specification](https://bbc.github.io/tams)
- [OpenAPI definition](https://github.com/bbc/tams/blob/main/api/TimeAddressableMediaStore.yaml)

## License

Apache-2.0
