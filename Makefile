help:
	@cat Makefile

ts:
	npm run start:ts

js:
	npm run start:js

rs:
	cargo run

go:
	go run src/main.go

py:
	python src/main.py

sh:
	${SHELL} src/main.sh
