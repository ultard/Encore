FROM rust:latest as builder

WORKDIR /usr/src/app

COPY . .
RUN cargo build --release

FROM alpine:latest as bot

COPY --from=builder /usr/src/app/target/release/Encore ./Encore

CMD ["./Encore"]