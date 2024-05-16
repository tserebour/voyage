FROM rust:1.75-buster as builder



WORKDIR /app
ARG DATABASE_URL
ENV DATABASE_URL=$DATABASE_URL
COPY . .


RUN cargo build --release

# Preduction stage

FROM debian:buster-slim
RUN apt-get update && apt-get install -y libssl-dev
WORKDIR /user/local/bin

COPY --from=builder /app/target/release/voyage ./

CMD [ "./voyage" ]

