.PHONY: gen server client

gen:
	protoc --grpc_out=. --rust_out=kernel=upb,experimental-codegen=enabled:. \
	--plugin=protoc-gen-grpc=`which grpc_rust_plugin` protos/hello.proto

server:
	cargo run --bin server

client:
	cargo run --bin client
