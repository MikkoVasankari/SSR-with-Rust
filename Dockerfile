FROM  rust:latest AS app_ssr_rust

RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk wasm-bindgen-cli

WORKDIR /usr/src/myapp
COPY . .

RUN cd /usr/src/myapp/frontend/ && trunk build --release
RUN cd /usr/src/myapp && cargo build --release 


FROM debian:latest

RUN apt update && apt install -y ca-certificates

COPY --from=app_ssr_rust /usr/src/myapp/target/release/backend /usr/local/bin/backend
COPY --from=app_ssr_rust /usr/src/myapp/frontend /usr/local/bin/frontend

WORKDIR /usr/local/bin
CMD ["backend"]

