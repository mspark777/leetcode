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

rs:
	@cargo run

cpp:
	@rm -f ./bin/cpp
	@clang++ --std=c++20 -Wall -Wextra -Werror -fsanitize=address -fno-omit-frame-pointer -g -o bin/cpp src/cpp/main.cpp src/cpp/tree_node.cpp
	@./bin/cpp
	@rm ./bin/cpp
