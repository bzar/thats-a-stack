FROM node:22 as tailwind_build
WORKDIR /srv/app
COPY tailwind tailwind
COPY assets assets
COPY templates templates
RUN cd tailwind && npm install --include dev && npm run prod

FROM rust:1.78 as build
WORKDIR /srv/app
COPY Cargo.toml Cargo.lock .
COPY assets assets
COPY src src
COPY --from=tailwind_build /srv/app/assets/main.css assets/main.css
COPY templates templates
# Build a statically linked binary with musl so we can use scratch image
RUN rustup target add x86_64-unknown-linux-musl && cargo build --release --target=x86_64-unknown-linux-musl

FROM scratch
COPY --from=build /srv/app/target/x86_64-unknown-linux-musl/release/thats-a-stack .
CMD ["./thats-a-stack"]
