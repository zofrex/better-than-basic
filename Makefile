build-docker:
	docker build . -t my-rust

run-docker:
	docker run -it -p 3000:3000 my-rust
