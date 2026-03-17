# CUDA Setup

Build llama.cpp with CUDA support after cloning:

```bash
cmake -S vendor/llama.cpp -B vendor/llama.cpp/build -DGGML_CUDA=ON
cmake --build vendor/llama.cpp/build -j
```
