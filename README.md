# http-traffic-gen-rs

Simple http traffic generator.

## What it does

This traffic generator creates HTTP requests based on configuration and sends it to
specified target.

### Example

```
    HEAD /logo.jpg HTTP/1.1
    host: localhost
    X-Forwarded-For: 10.0.0.182
```

### Configurable parameters

  * Methods list
  * Uris list
  * Target
  * Source network (IP is put into `X-Forwarded-For` header)

Traffic generator is configured by `config.toml` file.
Please see the example in sources.

## Building

### Pre-requisites

  * Rust nigthly toolchain

### Building

```
cargo build
```

## Running

```
cargo run
```

## Debugging

Log level can be set with `RUST_LOG` environment variable.

### Example

```
$ RUST_LOG=trace cargo run
```

## Next Features

  - [ ] Passing configuration file in parameter
  - [ ] Simulate connections drops
  - [ ] Send requests with body
  - [ ] URI and body fuzzing
