# triton-grpc-proxy

proxy server for triton gRPC inference server

## Build

```shell
$ export RUSTFLAGS="-C target-feature=native"
$ make server
```

## API Specs

* endpoint : `http://[url]:8080`

% currently, `url` and `port` for proxy and triton gRPC server are hard-coded.
% embedding dimenstion is also fixed with value `2048`.

### health

* GET `/health`

```shell
$ curl -i http://127.0.0.1/health
```

```shell
HTTP/1.1 200 OK
content-length: 2
content-type: application/json
date: Fri, 29 Sep 2023 04:26:41 GMT

Ok
```

### embedding

* POST `/v1/embedding`
* Request Body : `[{'query': 'input'}, ... ]`

```shell
$ curl -H "Content-type:application/json" -X POST http://127.0.0.1:8080/v1/embedding -d "[{\"query\": \"asdf\"}, {\"query\": \"asdf asdf\"}, {\"query\": \"asdf asdf asdf\"}, {\"query\": \"asdf asdf asdf asdf\"}]"
```

* Response Body : `[{'embedding': [[2048 f32 vector], ...]}]`

```shell
[{"embedding":[-0.30630538,-0.36736542,-0.13295595,0.9422532,-0.34492892,0.08723581,-0.085213244,-0.72103804,...,-0.06771816,0.068485156,-0.09190754,-0.90863633]}]
```

## Maintainer

[@kozistr](http://kozistr.tech)
