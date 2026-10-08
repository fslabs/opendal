# Apache OpenDAL™ Amazon S3 Service

`opendal-service-s3` provides access to Amazon S3 and S3-compatible object storage for
applications built with Apache OpenDAL™.

## Use through `opendal`

Applications should normally enable this service through the `opendal` facade with the
`services-s3` feature:

```shell
cargo add opendal --features services-s3
```

The service is available as `opendal::services::S3`. Configure the
service builder, then pass it to `opendal::Operator::new`.

## Use with `opendal-core`

Add the split crates directly:

```shell
cargo add opendal-core opendal-service-s3
```

Pass a configured service builder to `Operator::new`:

```rust
use opendal_core::{Operator, OperatorRegistry, Result};
use opendal_service_s3::{register_s3_service, S3};

fn build_operator(builder: S3) -> Result<Operator> {
    Operator::new(builder)
}

fn register_for_uri() {
    register_s3_service(OperatorRegistry::get());
}
```

`register_for_uri` is only needed for scheme-driven construction through
`Operator::from_uri` or `Operator::via_iter`.

Services that send HTTP requests also require an HTTP transport in
`OperationContext`. See the
[`opendal-core` composition example](https://crates.io/crates/opendal-core).

## CRC32C checksums

The `checksum-crc32c` feature enables CRC32C upload checksums and is enabled by default.
Disable default features when CRC32C checksums are not needed:

```toml
opendal-service-s3 = { version = "0.59", default-features = false }
```

Reads, writes without checksums, and MD5 checksums remain available. Configuring
`checksum_algorithm("crc32c")` without this feature returns a configuration error when
building the service.

## Documentation

- [Service configuration and examples](https://opendal.apache.org/services/s3)
- [Rust API documentation](https://docs.rs/opendal-service-s3)
- [Apache OpenDAL documentation](https://opendal.apache.org/docs/)

## License

Licensed under the Apache License, Version 2.0.
