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

cpp:
	@g++ -std=c++20 -Wall -Wextra -Werror -fsanitize=address -o bin/cpp src/cpp/main.cpp src/cpp/tree_node.cpp
	@./bin/cpp
	@rm ./bin/cpp

c:
	@gcc -std=c11 -Wall -Wextra -Werror -fsanitize=address -o bin/c src/c/main.c
	@./bin/c
	@rm ./bin/c
