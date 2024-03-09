FROM rust:1.72.0-bullseye as builder

WORKDIR /app
COPY . /app

RUN cargo build --release && chmod +x ./target/release/giuda && mv ./target/release/giuda /usr/local/bin && rm -rf /app

FROM freonius/latex-full

COPY --from=builder /usr/local/bin/giuda /usr/local/bin/giuda

WORKDIR /app

ENTRYPOINT ["/usr/local/bin/giuda"]