CC ?= cc
CFLAGS ?= -Wall -Wextra -O2

build/{{name}}: src/main.c | build
	$(CC) $(CFLAGS) -o $@ $<

build:
	mkdir -p build

run: build/{{name}}
	./build/{{name}}

clean:
	rm -rf build

.PHONY: run clean
