# Makefile for a Rust project

# Variables
CARGO = cargo
TARGET = target/release/sheldx

# Default target (runs when you just type 'make')
all: build run cleanlog

# Building the project
build:
	$(CARGO) build --release

# Running the project
run: build cleanlog
	$(TARGET)

# Delete any .log files
cleanlog:
	rm -f *.log

# Cleaning the project
clean:
	$(CARGO) clean

# Running tests
test:
	$(CARGO) test

# A phony target is one that is not a real file
.PHONY: all build run clean test cleanlog
