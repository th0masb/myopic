
This application polls the lichess event stream endpoint
to detect valid challenges and react to them by
invoking the game lambda function. It is published as a
docker image for both amd64 and arm64 platforms, to pull
the latest version run

```shell
docker pull ghcr.io/th0masb/myopic/event-stream:latest
```

The image is built and published automatically each time
a change is merged to the main branch, versioning is just
done by commit sha.

The container requires various bits of configuration to
be injected into either through environment variables
or credential files on the host. The easiest way to start
the application is via the docker-compose config file
deploy.yaml which defines the config required and provides
default values for most variables. You can pull this file
using 

```shell
curl https://raw.githubusercontent.com/th0masb/myopic/master/event-stream/docker-compose.yaml \
    -o event-stream.yaml
```

and then start the application with

```shell
docker-compose -f event-stream.yaml up -d
```

Note that `~/myopic/credentials` must be present on the
docker host and will be injected into the container to be
used as the AWS credentials file. The `MYOPIC_LICHESS_AUTH_TOKEN`
defined in deploy.yaml is for authorizing the Lichess user
account, the value can be manually added in the file or it
will be pulled in automatically from the environment variable
of the same name defined on the docker host if it exists.

