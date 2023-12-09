FROM rust:bookworm

ENV PATH="/root/.cargo/bin:${PATH}"
RUN rustup default nightly
RUN cargo install cargo-leptos
RUN rustup target add wasm32-unknown-unknown
RUN cargo install diesel_cli --no-default-features --features postgres

WORKDIR /tukosmo

COPY . /tukosmo

RUN make install

ENV CARGO_NET_GIT_FETCH_WITH_CLI=true

ENV DATABASE_URL=postgres://tukosmo:tukosmo_db_password@tukosmo_db/tukosmo

CMD ["make", "run"]
