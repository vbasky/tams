# tams

Rust implementation of the [Time-Addressable Media Store (TAMS)](https://tams.org/) API specification.

TAMS is an open API specification developed by BBC Research & Development for storing, querying, and randomly accessing segmented media in a sample-accurate, time-addressable manner. It separates the metadata plane from the media data plane, enabling cloud-native, scalable media workflows.

## Features (planned)

- Full TAMS API client
- Server implementation with pluggable storage backends (S3, filesystem, etc.)
- Types and models for Flows, Sources, Segments, Timeranges
- Support for the latest TAMS specification

## Status

Early development. This is a starter project.

## License

Apache-2.0

## Links

- [TAMS Website](https://tams.org/)
- [Official Spec Repository](https://github.com/bbc/tams)
- [Rendered API Spec](https://bbc.github.io/tams)
