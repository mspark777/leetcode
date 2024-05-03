help:
	@cat Makefile

ts:
	@npm run start:ts

js:
	@npm run start:js

go:
	@go run src/go/main.go

py:
	@python src/py/main.py

sh:
	@cd src/sh && bash main.sh

c:
	@rm -f ./bin/c
	@clang --std=c11 -Wall -Wextra -Werror -fsanitize=address -fno-omit-frame-pointer -g -o bin/c src/c/main.c
	@./bin/c
	@rm ./bin/c
