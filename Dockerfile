# Use the official Rust image.
# https://hub.docker.com/_/rust
FROM rust:latest

# Copy local code to the container image.
WORKDIR /usr/scp/app
COPY . .

# Install production dependencies and build a release artifact
RUN cargo install --path .

# run the web service on container startup
CMD ["ip-reflector-service"]
#test