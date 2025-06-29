ARG RUST_VERSION=1.85.1

FROM rust:${RUST_VERSION} AS builder

WORKDIR /app

RUN apt-get update && \
    apt-get install -y libpq-dev && \
    rm -rf /var/lib/apt/lists/*

COPY . .

RUN cargo build && \
    cp target/debug/api /bin/backend

################################################################################

FROM debian:bookworm-slim AS final

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    git \
    ca-certificates \
    openssh-client \
    libpq-dev && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /bin/backend /bin/backend
COPY wait-for-it.sh /bin/wait-for-it.sh
RUN chmod +x /bin/wait-for-it.sh

ARG UID=989
ARG GID=989
RUN addgroup --system --gid "$GID" appgroup && \
    adduser --system \
            --disabled-password \
            --gecos "" \
            --home "/home/appuser" \
            --shell "/usr/sbin/nologin" \
            --uid "$UID" \
            --gid "$GID" \
            appuser

USER appuser

EXPOSE 3001

CMD ["/bin/backend"]