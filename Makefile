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
	rm -f ./bin/cpp
	clang++ --std=c++20 -xc++ -Wall -Wextra -Werror -o bin/cpp src/cpp/main.cpp
	./bin/cpp
	rm ./bin/cpp
