# triton-grpc-proxy-rs

Proxy server for triton gRPC server that inferences embedding model in Rust.

* it refines the request and response formats of the Triton server.
* without `tritonclient` dependency.
* fast & easy to use.

## Build

### 1. Convert the embedding model to onnx

* [`BAAI/bge-m3`](https://huggingface.co/BAAI/bge-m3) is used for an example.
* It'll convert Pytorch into onnx model with the cls pooling + l2 normalization layers, and save it to `./model_repository/embedding/1/model.onnx`.
  * if you don't want to add the pooling + l2 normalization layers, then need to change the `config.pbtxt` properly.
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
export RUSTFLAGS="-C target-cpu=native"
make server
```

```shell
make build-docker
```

### Build & run triton inference server only

```shell
docker run --gpus all --rm --ipc=host --shm-size=8g --ulimit memlock=-1 --ulimit stack=67108864 -p8000:8000 -p8001:8001 -p8002:8002 -v$(pwd)triton-grpc-proxy-rs/model_repository:/models nvcr.io/nvidia/tritonserver:24.07-py3 bash -c "LD_PRELOAD=/usr/lib/$(uname -m)-linux-gnu/libtcmalloc.so.4:${LD_PRELOAD} && pip install transformers tokenizers && tritonserver --model-repository=/models"
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
* `EMBEDDING_SIZE`: size of the embedding. default `1024`.

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

### metrics

get prometheus metrics

* GET `/metrics`

```shell
curl -i http://127.0.0.1:8080/metrics
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
  * Rust : v1.79.0 stable
  * Triton Server : `24-07-py3`
    * backend : onnxruntime-gpu
    * allocator : tcmalloc
  * model : `BAAI/bge-m3` w/ fp32
* payload : `[{'query': 'asdf' * 126}] * batch_size` (`asdf * 126 == 255 tokens`)
* stages
  * model : only triton gRPC server latency (preprocess + tokenize + model)
  * processing : end-to-end latency (service-side)
    * json de/serialization
    * payload serialization (byte string, float vector)
    * cast & reshape 2d vectors

| batch size |   model (p90)   | processing (p90) |
|    :---:   |      :---:      |       :---:      |
|      8     |   1428.20 ms    |     0.044 ms     |
|     16     |   2915.01 ms    |     0.051 ms     |
|     32     |   5626.15 ms    |     0.055 ms     |

## To-Do

* [x] optimize the processing performance and memory usage
* [x] support `/metrics` endpoint to get prometheus metrics
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
