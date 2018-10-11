# Nasqueron private Docker registry API

This API exposes the content of the private registry available  
on Nasqueron Docker servers linked to the Docker PaaS.

The PaaS mainly images from Docker Hub, so the images references here  
are generally special case like work in progress containers or migrations.

## Reuse this microservice for my registry

This API is compatible for any [Docker registry container](https://docs.docker.com/registry/) installation.

It can be used  as a standalone server or composed with other microservices in a more comprehensive API.

You need to mount the same volume or host directory for /var/lib/registry you mount on the registry container.

## Contribute

### Dependencies

This microservice uses [Rocket](https://rocket.rs/), with the [Limiting Factor](http://docs.nasqueron.org/limiting-factor/rust/limiting_factor/) libraries. It's written in Rust.

A Swagger/OpenAPI [specification](nasqueron-api-docker-registry.spec.yaml) is available.
It allows to generate documentation, clients or servers.

## License
This work is licensed under [BSD-2-Clause](LICENSE).
