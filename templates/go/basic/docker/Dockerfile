FROM golang:1.24-alpine AS build
WORKDIR /app
COPY . .
RUN go build -o app ./cmd/...

FROM alpine:3.21
WORKDIR /app
COPY --from=build /app/app ./app
CMD ["./app"]
