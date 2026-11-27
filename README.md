# tams

![TAMS Banner](docs/banner.jpg)

Rust implementation of the [Time-Addressable Media Store (TAMS)](https://tams.org/) API specification.

TAMS is an open API specification developed by BBC Research & Development for storing, querying, and randomly accessing segmented media in a sample-accurate, time-addressable manner.

## Workspace structure

```
crates/
  tams/          # Facade crate (re-exports)
  tams-core/     # Models, types, timestamps, timeranges
  tams-client/   # HTTP client
```

## Crates

| Crate | Description |
|-------|-------------|
| `tams` | Main entry point — re-exports core + client |
| `tams-core` | Pure data models and types |
| `tams-client` | Async HTTP client for the TAMS API |

## Status

Early development. Starter workspace following the Revelo project structure.

## License

Apache-2.0

## Links

- [TAMS Website](https://tams.org/)
- [Official Spec](https://github.com/bbc/tams)
- [Rendered Spec](https://bbc.github.io/tams)
