lint:
	@echo -e "Running linter"
	@isort concurrency tests
	@black concurrency tests
	@echo -e "Running type checker"

typecheck:
	@mypy -p concurrency

test: ## Run the tests.conf
	@pytest -s

all: lint typecheck test
