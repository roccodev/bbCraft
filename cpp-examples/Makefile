CC=g++
CFLAGS=-c -Werror

server: server.cpp
	g++ -o output/server.o -fpic server.cpp
	g++ -shared server.o