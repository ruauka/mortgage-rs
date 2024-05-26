server:
	@cargo run
	#cargo run -- --host 0.0.0.0 --port 8000

fmt:
	@cargo fmt

lint:
	@cargo clippy

test:
	@cargo llvm-cov

test_html:
	@cargo llvm-cov --html && open target/llvm-cov/html/index.html

doc:
	@cargo doc --open

build:
	@docker build --tag=mg:latest .

run:
	@docker run -d --rm --name=mg -p 8080:8080 mg:latest

stop:
	@docker stop mg