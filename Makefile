help:
	@cat Makefile

ts:
	@npm run start:ts

js:
	@npm run start:js

go:
	@go run src/go/main.go src/go/utils.go

py:
	@python src/py/main.py

sh:
	@${SHELL} src/main.sh

rs:
	cargo run
