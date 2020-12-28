# Fibonacci Challenge

Exposes a server with the following endpoints:

```
GET /next - Progresses the sequence and returns the result
GET /previous - Regresses the sequence and returns the result
GET /current - Returns the currect item in the sequence with no effect
```

## Running

The service can be run directly or in a container. In order to achieve simple restarts in the event of a crash, the container is the preferred deployment mechanism. The provided `docker-compose.yaml` will always restart and other container orchestration solutions have similar settings. The image that results from the provided Dockerfile is about 14.6M. The Rust binary is 8.6M after optimization. This makes it trivial to deploy on minimally resourced servers.

### Commands

- `docker-compose up` - With [Docker]() and [Docker Compose]() installed, `docker-compose up` in the project directory will start the application on the default `localhost:8080`. This can be configured in `docker-compose.yaml`.
- `cargo run` - With [rustup](https://rustup.rs/) installed, `cargo run` in the project directory will start the application on the default `localhost:8080`. If the default port is changed, be sure to update `docker-compose.yaml` to the new target port.

## Documentation

`cargo doc --open`

## Tests

`cargo test`

## How it Works

The Fibonacci sequence is generally simple once you account for the challenges in the initial 3 iterations. Adding the current number and its predecessor to achieve the next item is much simpler once the first 0, 1, 1 are out of the way. This project handles this initial unusual case manually to ensure that there are no issues traversing the sequence in either direction. These cases are best read in the code and its comments and are not detailed here.

The `Fibonacci` struct contains the current count and, in the general case, the most recent 3 items in the sequence from oldest to newest. This means that the `last()` item is the most recent. To progress, the last 2 items are summed and the result pushed, while the 0th item is dropped. As long as `next()` is called, this is the sequence. For `previous()` calls, the last item is dropped, the remaining items are shifted right, and the 0th item is derived by subtracting the item at index 1 from the, now current, item at index 2. The sequence will not drop below zero.

The decision to maintain only the last 3 numbers in the sequence was made to preserve memory. With the approach, we are able to achieve all of the requirements while still maintaining a very low memory footprint at runtime even for large numbers of iterations. This service should have no issue operating on the required resource constraints. Rust adds further to speed and size at runtime.

## Possible Improvements

There are a few uses of `clone()` that add slightly to the memory footprint of the application. Further optimization could be achieved by replacing these with references where possible.

There are a few uses of `unwrap()`. While these are all inside previous size checks against vector length, it is concievable that future changes to the code base could cause these unwraps to panic. Stricter vector length requirements and additional `match`es or the like would be an improvement.

While the application has cookie integration, it is not being used. We could achieve per-user sequences with a cookie. If the goal is a persistent, universal sequence like what exists now we should store the state in a cache or database to persist across restarts.
