help:
	@cat Makefile

ts:
	@npm run start:ts

js:
	@npm run start:js

go:
	@go run src/go/main.go src/go/lib.go

py:
	@python src/py/main.py

sh:
	@${SHELL} src/main.sh

c:
	@gcc -std=c11 -fsanitize=address -fno-omit-frame-pointer -Wall -Wextra -o bin/main src/c/main.c
	@./bin/main
	@rm ./bin/main
