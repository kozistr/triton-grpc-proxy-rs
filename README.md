# triton-grpc-proxy-rs

Proxy server for triton gRPC server that inferences embedding model in Rust.

* it refines the request and response formats of the Triton server.
* without `tritonclient` dependency.
* fast & easy to use.

## Build

### 1. Convert the embedding model to onnx

* [`BAAI/bge-large-en-v1.5`](https://huggingface.co/BAAI/bge-large-en-v1.5) is used for an example.
* It'll convert Pytorch into onnx model, and save it to `./model_repository/embedding/1/v1.onnx`.
* Currently, `max_batch_size` is limited to `256` due to OOM. You can change this value to fit your environment.

```shell
python3 convert.py
```

### 2. Run docker-compose

* I'll run both Triton inference server and the proxy server.
* You need to edit the absolute path of the volume (where pointed to the `./model_repository`) in `docker-compose.yml`.

```shell
make run-docker-compose
```

### Build & run a proxy server only

* You can also build and run a triton proxy server with the below command.

```shell
export RUSTFLAGS="-C target-feature=native"
make server
```

```shell
make build-docker
```

### Build & run triton inference server only

```shell
docker run --gpus all --rm --ipc=host --shm-size=8g --ulimit memlock=-1 --ulimit stack=67108864 -p8000:8000 -p8001:8001 -p8002:8002 -v$(pwd)triton-proxy-server-rs/model_repository:/models nvcr.io/nvidia/tritonserver:23.09-py3 bash -c "LD_PRELOAD=/usr/lib/$(uname -m)-linux-gnu/libtcmalloc.so.4:${LD_PRELOAD} && pip install transformers tokenizers && tritonserver --model-repository=/models"
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

* Response Body : `[{'embedding': '1024 f32 vector'}, ...]`

```shell
[{"embedding": [-0.8067292,-0.004603,-0.24123234,0.59398544,-0.5583446,...]}, ...]
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
  * request : end-to-end latency (client-side)
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

## To-Do

* [x] add `Dockerfile` and `docker-compose` to easily deploy the servers
* [x] triton inference server
  * [x] add model converter script.
  * [x] configurations
* [x] move hard-coded configs to `env`
* [x] optimize the proxy server performance
* [x] README
* [ ] move `tokenizer` part from triton server into `proxy-server`

## Maintainer

[@kozistr](http://kozistr.tech)
