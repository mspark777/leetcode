help:
	@cat Makefile

ts:
	@npm run start:ts

js:
	@npm run start:js

go:
	@go run src/main.go src/lib.go

py:
	@python src/main.py

sh:
	@${SHELL} src/main.sh

c:
	@gcc -std=c11 -fsanitize=address -fno-omit-frame-pointer -Wall -Wextra -o bin/main src/main.c
	@./bin/main
	@rm ./bin/main
