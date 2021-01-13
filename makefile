
linux_build_docker:
	docker run -v $(shell pwd):/volume -v cargo-cache:/root/.cargo/registry --rm -t clux/muslrust cargo build --release --target x86_64-unknown-linux-musl ;exit

linux_build_podman:
	podman run -v $(shell pwd):/volume -v cargo-cache:/root/.cargo/registry --rm -t clux/muslrust cargo build --release --target x86_64-unknown-linux-musl ;exit
	
linux_build_docker_dev:
	docker run -v $(shell pwd):/volume -v cargo-cache:/root/.cargo/registry --rm -t clux/muslrust cargo build --target x86_64-unknown-linux-musl ;exit

linux_build_podman_dev:
	podman run -v $(shell pwd):/volume -v cargo-cache:/root/.cargo/registry --rm -t clux/muslrust cargo build --target x86_64-unknown-linux-musl ;exit

# docker-compose build can be used instead
build_image_docker:
	docker build -f Dockerfile .
build_image_podman:
	podman build -f Dockerfile .
build_image_docker_dev:
	docker build -f Dockerfile.dev .
build_image_podman_dev:
	podman build -f Dockerfile.dev .

run_full_linux_podman:
	make linux_build_podman & make build_image_podman
run_full_linux_docker:
	make linux_build_docker & make build_image_docker