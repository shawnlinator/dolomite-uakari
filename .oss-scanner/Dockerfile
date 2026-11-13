# dolomite-uakari — Proxmox Backup Server v4.2.11 (official Debian trixie build path)
#
# Verified recipe (smoke-tested end to end in debian:trixie):
#  * `deb http://download.proxmox.com/debian/devel/ trixie main` resolves all 169
#    Build-Depends packages from debian/control (real install: 235s).
#  * The Proxmox librust-*-dev packages populate /usr/share/cargo/registry with
#    UNPACKED crate directories, which is exactly what the repo's committed
#    .cargo/config.toml `debian-packages` directory source expects — no
#    `cargo prepare-debian` step is needed.
#  * The Proxmox devel repo carries the `pathpatterns-1` fork (the crates.io
#    `pathpatterns` has no 1.x), which Cargo.toml requires.
#  * Cargo.toml declares rust-version = "1.91"; trixie's rustc (1.85) is too old,
#    so rustup stable is installed.
#  * No web UI / npm step exists (no web/ dir, no package.json, no npm in Makefile);
#    `cargo build --release` is the entire build. pbs-buildcfg/build.rs needs only
#    REPOID (set below) or git.
#  * gcc/g++/make/pkg-config are added on top of the verified 169: some build
#    scripts (zstd-sys, openssl-sys) invoke the C toolchain / pkg-config.

FROM debian:trixie

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates wget \
    && wget -qO /etc/apt/trusted.gpg.d/proxmox-trixie.gpg https://enterprise.proxmox.com/debian/proxmox-release-trixie.gpg \
    && echo "deb http://download.proxmox.com/debian/devel/ trixie main" > /etc/apt/sources.list.d/proxmox-devel.list \
    && apt-get update \
    && apt-get install -y --no-install-recommends \
        gcc \
        g++ \
        make \
        pkg-config \
        jq \
        libacl1-dev \
        libfuse3-dev \
        librust-anyhow-1+default-dev \
        librust-async-trait-0.1+default-dev \
        librust-bitflags-2+default-dev \
        librust-bytes-1+default-dev \
        librust-cidr-0.3+default-dev \
        librust-const-format-0.2+default-dev \
        librust-crc32fast-1+default-dev \
        librust-crossbeam-channel-0.5+default-dev \
        librust-endian-trait-0.6+arrays-dev \
        librust-endian-trait-0.6+default-dev \
        librust-env-logger-0.11+default-dev \
        librust-foreign-types-0.3+default-dev \
        librust-futures-0.3+default-dev \
        librust-h2-0.4+default-dev \
        librust-h2-0.4+stream-dev \
        librust-hex-0.4+default-dev \
        librust-hex-0.4+serde-dev \
        librust-hickory-resolver-0.24+system-config-dev \
        librust-hickory-resolver-0.24+tokio-runtime-dev \
        librust-http-body-util-0.1+default-dev \
        librust-hyper-1+default-dev \
        librust-hyper-1+full-dev \
        librust-hyper-util-0.1+client-dev \
        librust-hyper-util-0.1+client-legacy-dev \
        librust-hyper-util-0.1+default-dev \
        librust-hyper-util-0.1+http1-dev \
        librust-hyper-util-0.1+http2-dev \
        librust-hyper-util-0.1+server-auto-dev \
        librust-hyper-util-0.1+server-dev \
        librust-hyper-util-0.1+server-graceful-dev \
        librust-hyper-util-0.1+service-dev \
        librust-hyper-util-0.1+tokio-dev \
        librust-libc-0.2+default-dev \
        librust-log-0.4+default-dev \
        librust-nix-0.29+default-dev \
        librust-nix-0.29+feature-dev \
        librust-nix-0.29+mount-dev \
        librust-nix-0.29+reboot-dev \
        librust-nom-7+default-dev \
        librust-num-traits-0.2+default-dev \
        librust-once-cell-1+default-dev \
        librust-openssl-0.10+default-dev \
        librust-pathpatterns-1+default-dev \
        librust-pbs-api-types-1+default-dev \
        librust-percent-encoding-2+default-dev \
        librust-pin-project-lite-0.2+default-dev \
        librust-proxmox-acme-1+default-dev \
        librust-proxmox-acme-api-1+default-dev \
        librust-proxmox-acme-api-1+impl-dev \
        librust-proxmox-apt-1+cache-dev \
        librust-proxmox-apt-1+default-dev \
        librust-proxmox-apt-api-types-3+default-dev \
        librust-proxmox-async-0.5+default-dev \
        librust-proxmox-auth-api-1+api-dev \
        librust-proxmox-auth-api-1+default-dev \
        librust-proxmox-auth-api-1+pam-authenticator-dev \
        librust-proxmox-base64-1+default-dev \
        librust-proxmox-borrow-1+default-dev \
        librust-proxmox-compression-1+default-dev \
        librust-proxmox-config-digest-1+default-dev \
        librust-proxmox-daemon-1+default-dev \
        librust-proxmox-docgen-1+default-dev \
        librust-proxmox-fuse-3+default-dev \
        librust-proxmox-http-1+body-dev \
        librust-proxmox-http-1+client-dev \
        librust-proxmox-http-1+client-trait-dev \
        librust-proxmox-http-1+default-dev \
        librust-proxmox-http-1+http-helpers-dev \
        librust-proxmox-http-1+proxmox-async-dev \
        librust-proxmox-http-1+rate-limited-stream-dev \
        librust-proxmox-http-1+websocket-dev \
        librust-proxmox-human-byte-1+default-dev \
        librust-proxmox-io-1+default-dev \
        librust-proxmox-io-1+tokio-dev \
        librust-proxmox-lang-1+default-dev \
        librust-proxmox-ldap-1+default-dev \
        librust-proxmox-log-1+default-dev \
        librust-proxmox-metrics-1+default-dev \
        librust-proxmox-network-api-1+default-dev \
        librust-proxmox-network-api-1+impl-dev \
        librust-proxmox-network-types-1+default-dev \
        librust-proxmox-notify-2+default-dev \
        librust-proxmox-notify-2+legacy-matchers-dev \
        librust-proxmox-notify-2+pbs-context-dev \
        librust-proxmox-openid-1+default-dev \
        librust-proxmox-parallel-handler-1+default-dev \
        librust-proxmox-product-config-1+default-dev \
        librust-proxmox-rate-limiter-1+default-dev \
        librust-proxmox-rest-server-1+default-dev \
        librust-proxmox-rest-server-1+rate-limited-stream-dev \
        librust-proxmox-rest-server-1+templates-dev \
        librust-proxmox-router-3+cli-dev \
        librust-proxmox-router-3+server-dev \
        librust-proxmox-rrd-1+default-dev \
        librust-proxmox-rrd-api-types-1+default-dev \
        librust-proxmox-s3-client-1+default-dev \
        librust-proxmox-s3-client-1+impl-dev \
        librust-proxmox-schema-5+api-macro-dev \
        librust-proxmox-schema-5+default-dev \
        librust-proxmox-section-config-3+default-dev \
        librust-proxmox-serde-1+default-dev \
        librust-proxmox-serde-1+serde-json-dev \
        librust-proxmox-shared-cache-1+default-dev \
        librust-proxmox-shared-memory-1+default-dev \
        librust-proxmox-simple-config-1+default-dev \
        librust-proxmox-sortable-macro-1+default-dev \
        librust-proxmox-subscription-1+api-types-dev \
        librust-proxmox-subscription-1+default-dev \
        librust-proxmox-sys-1+acl-dev \
        librust-proxmox-sys-1+crypt-dev \
        librust-proxmox-sys-1+default-dev \
        librust-proxmox-sys-1+logrotate-dev \
        librust-proxmox-sys-1+timer-dev \
        librust-proxmox-syslog-api-1+default-dev \
        librust-proxmox-syslog-api-1+impl-dev \
        librust-proxmox-systemd-1+default-dev \
        librust-proxmox-tfa-6+api-dev \
        librust-proxmox-tfa-6+api-types-dev \
        librust-proxmox-tfa-6+default-dev \
        librust-proxmox-time-2+default-dev \
        librust-proxmox-upgrade-checks-1+default-dev \
        librust-proxmox-uuid-1+default-dev \
        librust-proxmox-uuid-1+serde-dev \
        librust-proxmox-worker-task-1+default-dev \
        librust-pxar-1+default-dev \
        librust-regex-1+default-dev \
        librust-rustyline-14+default-dev \
        librust-serde-1+default-dev \
        librust-serde-1+derive-dev \
        librust-serde-json-1+default-dev \
        librust-tar-0.4+default-dev \
        librust-tempfile-3+default-dev \
        librust-termcolor-1+default-dev \
        librust-thiserror-2+default-dev \
        librust-tokio-1+default-dev \
        librust-tokio-1+fs-dev \
        librust-tokio-1+io-std-dev \
        librust-tokio-1+io-util-dev \
        librust-tokio-1+macros-dev \
        librust-tokio-1+net-dev \
        librust-tokio-1+parking-lot-dev \
        librust-tokio-1+process-dev \
        librust-tokio-1+rt-dev \
        librust-tokio-1+rt-multi-thread-dev \
        librust-tokio-1+signal-dev \
        librust-tokio-1+sync-dev \
        librust-tokio-1+time-dev \
        librust-tokio-openssl-0.6+default-dev \
        librust-tokio-stream-0.1+default-dev \
        librust-tokio-util-0.7+codec-dev \
        librust-tokio-util-0.7+default-dev \
        librust-tokio-util-0.7+io-dev \
        librust-tower-service-0.3+default-dev \
        librust-tracing-0.1+default-dev \
        librust-udev-0.9+default-dev \
        librust-url-2+default-dev \
        librust-walkdir-2+default-dev \
        librust-xdg-2+default-dev \
        librust-zstd-0.13+default-dev \
        librust-zstd-safe-7+default-dev \
        libsgutils2-dev \
        libstd-rust-dev \
        libsystemd-dev \
        patchelf \
        rsync \
        uuid-dev \
    && rm -rf /var/lib/apt/lists/*

RUN curl -fsSL https://sh.rustup.rs -o /tmp/rustup-init.sh \
    && sh /tmp/rustup-init.sh -y --profile minimal --default-toolchain stable \
    && . "$HOME/.cargo/env" \
    && rustc --version && cargo --version

COPY . /src
WORKDIR /src

ENV REPOID=dolomite-uakari \
    PATH=/root/.cargo/bin:$PATH

# --jobs=8: memory-safe on this host (23GB RAM, ~8GB baseline).
RUN cargo build --release --jobs=8
