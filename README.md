# triton-grpc-proxy-rs

Proxy server for triton gRPC server that inferences embedding model in Rust.

* it refines the request and response formats of the Triton server.
* without `tritonclient` dependency.
* fast & easy to use.

## Build

### build & run a binary

```shell
export RUSTFLAGS="-C target-feature=native"
make server
```

### build & run both triton-server & proxy-server with docker-compose

```shell
docker-compose up -d
```

## Architecture

1. recieve request(s) from the user.
    * list of `text (String)` in this case.
2. request the Triton gRPC server to get embeddings.
3. post-process (cast and reshape) the embeddings and returns to the users.

## API Specs

### Configs

* parse configuration from the env variables.

* `SERVER_PORT`: proxy server port. default `8080`.
* `TRITON_SERVER_URL`: triton inference gRPC server url. default `http://triton-server`.
* `TRITON_SERVER_GRPC_PORT`: triton inference gRPC server port. default `8001`.
* `MODEL_VERSION`: model version. default `1`.
* `MODEL_NAME`: model name. default `model`.
* `INPUT_NAME`: input name. default `text`.
* `OUTPUT_NAME`: output name. default `embedding`.
* `EMBEDDING_SIZE`: size of the embedding. default `2048`.

### health

* GET `/health`

```shell
curl -i http://127.0.0.1:8080/health
```

```text
HTTP/1.1 200 OK
content-length: 2
date: Sun, 08 Oct 2023 06:33:53 GMT

ok
```

### embedding

* POST `/v1/embedding`
* Request Body : `[{'query': 'input'}, ... ]`

```shell
curl -H "Content-type:application/json" -X POST http://127.0.0.1:8080/v1/embedding -d "[{\"query\": \"asdf\"}, {\"query\": \"asdf asdf\"}, {\"query\": \"asdf asdf asdf\"}, {\"query\": \"asdf asdf asdf asdf\"}]"
```

* Response Body : `[{'embedding': '2048 f32 vector'}, ...]`

```shell
[{"embedding":[-0.30630538,-0.36736542,-0.13295595,0.9422532,-0.34492892,0.08723581,-0.085213244,-0.72103804,...,-0.06771816,0.068485156,-0.09190754,-0.90863633]}, ...]
```

## Benchmark

* Environment
  * CPU : i7-7700K (not overclocked)
  * GPU : GTX 1060 6 GB
  * Rust : v1.73.0 stable
  * Triton Server : `23-09-py3`
    * backend : onnxruntime-gpu
    * allocator : tcmalloc
* payload : `[{'query': 'asdf' * 125}] * batch_size`
* stages
  * request : end to end latency (client-side)
  * model : only triton gRPC server latency (preprocess + tokenize + model)
  * processing : request - model latency
    * json de/serialization
    * serialization (byte string, float vector)
    * cast & reshape 2d vectors

| batch size |  request  |   model   | processing |
|    :---:   |   :---:   |   :---:   |    :---:   |
|      8     |   27.2 ms |   25.4 ms |    1.8 ms  |
|     16     |   36.0 ms |   33.7 ms |    2.3 ms  |
|     32     |   50.6 ms |   47.3 ms |    3.3 ms  |
|     64     |   90.9 ms |   85.5 ms |    5.4 ms  |
|    128     |  139.2 ms |  129.9 ms |    9.3 ms  |
|    256     |  307.4 ms |  287.1 ms |   20.3 ms  |

## Maintainer

[@kozistr](http://kozistr.tech)
