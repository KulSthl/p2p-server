linux_build:
	cargo build --release --target=x86_64-unknown-linux-musl

linux_build_docker:
	cd $(shell pwd)
	docker run -v cargo-cache:/root/.cargo/registry -v $PWD:/volume --rm -t clux/muslrust cargo build --release;exit

linux_build_podman:
	cd $(shell pwd)
	podman run -v cargo-cache:/root/.cargo/registry -v $PWD:/volume --rm -t clux/muslrust cargo build --release;exit
