# Start a new stage for a smaller image size
FROM debian:buster-slim

COPY ./target/x86_64-unknown-linux-gnu/debug/rollup ./rollup

# Command to run the executable
CMD ["./rollup"]
