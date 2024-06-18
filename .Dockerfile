FROM messense/rust-musl-cross:x86_64-musl as builder
ENV SQLX_OFFLINE=true
WORKDIR /voidkandy-dot-space

COPY . .

RUN cargo build --release --target x86_64-unknown-linux-musl

FROM scratch 
COPY --from=builder /voidkandy-dot-space/target/x86_64-unknown-linux-musl/release/voidkandy-dot-space /voidkandy-dot-space
ENTRYPOINT ["/voidkandy-dot-space"]
EXPOSE 3000
