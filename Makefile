help:
	@cat Makefile

ts:
	npm run start:ts

js:
	npm run start:js

go:
	go run src/main.go src/lib.go

py:
	python src/main.py

sh:
	${SHELL} src/main.sh

ex:
	mix clean
	mix run
