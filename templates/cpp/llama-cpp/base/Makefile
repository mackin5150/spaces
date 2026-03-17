bootstrap:
	git clone https://github.com/ggml-org/llama.cpp.git vendor/llama.cpp

build:
	cmake -S vendor/llama.cpp -B vendor/llama.cpp/build
	cmake --build vendor/llama.cpp/build -j

run:
	bash scripts/run-local.sh
