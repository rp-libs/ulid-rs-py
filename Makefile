

.PHONY: build-dev
build-dev:
	maturin develop --uv


.PHONY: test
test:
	pytest tests
