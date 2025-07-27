NAME=cycles
CC=gcc
SRC=$(shell ls *.c)

all:
	make build
build:
	$(CC) -Wall -Wextra -pedantic -lcjson $(SRC) -no-pie -o $(NAME)	
