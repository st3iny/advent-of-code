SOLUTIONS = $(wildcard day*/part*.c)

CFLAGS = -g

.PHONY: all
all: $(SOLUTIONS:.c=)

%: %.c
	gcc $(CFLAGS) -o $@ $^

.PHONY: clean
clean:
	rm -f $(SOLUTIONS:.c=)
