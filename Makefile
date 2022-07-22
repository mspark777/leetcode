
help:
	@cat Makefile

ts:
	npm run start:ts

js:
	npm run start:js

rs:
	cargo run

go:
	go run src/main.go src/solution.go

py:
	python src/main.py

ex:
	mix run
