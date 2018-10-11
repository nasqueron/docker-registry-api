#   -------------------------------------------------------------
#   Nasqueron - private Docker registry API image
#   - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#   Project:        Nasqueron
#   Created:        2018-10-11
#   License:        BSD-2-Clause
#   -------------------------------------------------------------

#    -------------------------------------------------------------
#     Builder phase
#    - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

FROM ekidd/rust-musl-builder:nightly AS builder
ADD . ./
RUN sudo chown -R rust:rust /home/rust && \
    cargo build --release

#    -------------------------------------------------------------
#     Release phase
#    - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

FROM alpine:latest
MAINTAINER Sébastien Santoro aka Dereckson <dereckson+nasqueron-docker@espace-win.org>

COPY --from=builder \
    /home/rust/src/target/x86_64-unknown-linux-musl/release/docker-registry-api \
    /usr/local/bin/

ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000

CMD /usr/local/bin/docker-registry-api
