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

c:
	@gcc -Wall -Wextra -Werror -fsanitize=address -o bin/c src/main.c
	@./bin/c
	@rm ./bin/c
