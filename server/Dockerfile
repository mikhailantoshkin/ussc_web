FROM rust:1.57-slim as builder

# создаем dummy-проект
RUN USER=root cargo new --bin server
WORKDIR /server

# копируем наши манифесты и собираем dummy-проект чтобы скомпилировать зависимости
COPY server/Cargo.lock ./Cargo.lock
COPY server/Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs

# копируем реальные исходники
COPY server/src ./src

# удаляем кеш предыдущей сборки для нашего кода и билдим реальный код
RUN rm ./target/release/deps/server*
RUN cargo build --release

FROM debian:buster-slim
COPY --from=builder /server/target/release/server /app/server
COPY server/Rocket.toml /app/Rocket.toml
WORKDIR /app
CMD ["/app/server"]