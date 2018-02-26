CC = gcc
CFLAGS = -Wall -g
LDFLAGS = -lrabbitmq
DEPS = utils.c utils.h

demo: $(DEPS)
	gcc $(CLFAGS) $(LDFLAGS) -o $@ main.c utils.c 
