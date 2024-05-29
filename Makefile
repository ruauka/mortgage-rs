server:
	@echo "  >  Starting server..."
	@cargo run
	#cargo run -- --host 0.0.0.0 --port 8000

fmt:
	@echo "  >  Formatting code..."
	@cargo fmt

lint:
	@echo "  >  Linting code..."
	@cargo clippy

test:
	@echo "  >  Testing code..."
	@cargo test

coverage:
	@echo "  >  Testing code and print CLI coverage..."
	@cargo llvm-cov

coverage_html:
	@echo "  >  Testing code and print HTML coverage..."
	@cargo llvm-cov --html && open target/llvm-cov/html/index.html

build:
	@echo "  >  Building image..."
	@docker build --tag=mg:latest .

run:
	@echo "  >  Creating container..."
	@docker run -d --rm --name=mg -p 8080:8080 mg:latest

stop:
	@echo "  >  Stop container..."
	@docker stop mg
