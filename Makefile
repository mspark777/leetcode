help:
	@cat Makefile

ts:
	@npm run start:ts

js:
	@npm run start:js

go:
	@go run src/go/main.go

sh:
	@cd src/sh && bash main.sh

rs:
	@cargo run
