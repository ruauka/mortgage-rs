server:
	@cargo run
	#cargo run -- --host 0.0.0.0 --port 8000

fmt:
	@cargo fmt

lint:
	@cargo clippy

doc:
	@cargo doc --open

build:
	@docker build --tag=mg:latest .

run:
	@docker run -d --rm --name=mg -p 8080:8080 mg:latest

stop:
	@docker stop mg