help:
	@cat Makefile

ts:
	@npm run start:ts

js:
	@npm run start:js

go:
	@go run src/main.go

py:
	@python src/main.py

sh:
	@${SHELL} src/main.sh

rs:
	cargo +1.58.2 run
