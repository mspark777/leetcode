ifeq ($(shell uname), Darwin)
	CCFLAGS = -std=c11 -fsanitize=undefined -fsanitize=address -fno-omit-frame-pointer -Wall -Wextra -g
else
	CCFLAGS= -std=c11 -fsanitize=undefined -fsanitize=leak -fsanitize=address -fno-omit-frame-pointer -Wall -Wextra -g
endif

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
	@rm -f ./bin/c
	gcc $(CCFLAGS) -o bin/c src/c/main.c src/c/lib.c
	@./bin/c

rs:
	@rm -f ./bin/rs
	@rustc -o bin/rs src/rs/main.rs
	@./bin/rs
