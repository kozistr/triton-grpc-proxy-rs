# triton-grpc-proxy

proxy server for triton gRPC inference server

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

* endpoint : `http://[url]:8080`
  * triton gRPC server: `127.0.0.1:8001`
  * proxy server: `127.0.0.1:8080`

% Currently, `url` and `port` for proxy and triton gRPC server are hard-coded.

% Embedding dimension is a fixed value `2048`.

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

| batch size |  request  |   model   | processing |
|    :---:   |   :---:   |   :---:   |    :---:   |
|      8     |  130.1 ms |  129.4 ms |    0.7 ms  |
|     16     |  217.3 ms |  216.2 ms |    1.1 ms  |
|     32     |  352.9 ms |  350.3 ms |    2.6 ms  |
|     64     |  596.6 ms |  591.6 ms |    5.0 ms  |
|    128     | 1063.8 ms | 1054.0 ms |    9.8 ms  |
|    256     | 2182.5 ms | 2164.5 ms |   18.0 ms  |

## Maintainer

[@kozistr](http://kozistr.tech)
