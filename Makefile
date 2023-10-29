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

cpp:
	@g++ -std=c++20 -Wall -Wextra -Werror -fsanitize=address -o bin/cpp src/main.cpp
	@./bin/cpp
	@rm ./bin/cpp
