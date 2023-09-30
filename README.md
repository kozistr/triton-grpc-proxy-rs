# triton-grpc-proxy-rs

Proxy server for triton gRPC server that inferences embedding model in Rust.

* it refines the request and response formats of the Triton server.
* without `tritonclient` dependency.
* fast & easy to use.

## Build

```shell
$ export RUSTFLAGS="-C target-feature=native"
$ make server
```

## Architecture

1. recieve request(s) from the user.
    * list of `text (String)` in this case.
2. request the Triton gRPC server to get embeddings.
3. post-process (cast and reshape) the embeddings and returns to the users.

## API Specs

* endpoint : `http://127.0.0.1:8080`
  * triton gRPC server: `127.0.0.1:8001`
  * proxy server: `127.0.0.1:8080`

* Currently, configurations are hard-coded in [constants](https://github.com/kozistr/triton-grpc-proxy-rs/blob/main/src/constants/mod.rs).

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

## Benchmark

* CPU : i7-7700K (not overclocked)
* GPU : GTX 1060 6 GB
* payload : `[{'query': 'asdf asdf asdf asdf'}] * batch_size`
* stages
  * request : end to end latency
  * model : only triton gRPC server latency (preprocess + tokenize + model)
  * processing : request - model latency
    * json de/serialization
    * serialization (byte string, float vector)
    * cast & reshape 2d vectors

| batch size |  request  |   model   | processing |
|    :---:   |   :---:   |   :---:   |    :---:   |
|      8     |   45.2 ms |   43.4 ms |    1.8 ms  |
|     16     |   77.1 ms |   74.7 ms |    2.4 ms  |
|     32     |  115.8 ms |  111.6 ms |    4.2 ms  |
|     64     |  190.1 ms |  183.1 ms |    7.0 ms  |
|    128     |  361.6 ms |  350.0 ms |   11.6 ms  |
|    256     |  711.1 ms |  688.5 ms |   22.6 ms  |

## Maintainer

[@kozistr](http://kozistr.tech)
