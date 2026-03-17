FROM ubuntu:24.04
RUN apt-get update && apt-get install -y build-essential cmake git bash && rm -rf /var/lib/apt/lists/*
WORKDIR /workspace
COPY . .
CMD ["bash"]
