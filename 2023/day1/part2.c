#include <assert.h>
#include <stdio.h>
#include <stdbool.h>
#include <string.h>
#include <sys/param.h>

int parse_literal(char** str, char* end) {
    if (*str >= end) {
        return -1;
    }

    int num = -1;
    char* next_str = end;

#define SEARCH_LIT(LIT, NUM, SKIP) \
    { \
        char* pos = strstr(*str, LIT); \
        if (pos != NULL && pos < next_str) { \
            next_str = MIN(&pos[SKIP], end); \
            num = NUM; \
        } \
    }

    SEARCH_LIT("one", 1, 2);
    SEARCH_LIT("two", 2, 2);
    SEARCH_LIT("three", 3, 4);
    SEARCH_LIT("four", 4, 4);
    SEARCH_LIT("five", 5, 3);
    SEARCH_LIT("six", 6, 3);
    SEARCH_LIT("seven", 7, 3);
    SEARCH_LIT("eight", 8, 4);
    SEARCH_LIT("nine", 9, 3);
#undef SEARCH_LIT

    *str = next_str;
    return num;
}

int main() {
    int acc = 0;
    char* line = NULL;
    size_t len = 0;
    ssize_t bytes;
    while ((bytes = getline(&line, &len, stdin)) != -1) {
        printf("%s", line);

        char* iter = line;
        int first = -1, second = -1;
        char lit_buf[1024];
        size_t lit_buf_pos = 0;
        for (char* iter = line; *iter != 0; iter += sizeof(char)) {
            char c = *iter;

            bool is_digit = (c >= '0' && c <= '9');

            if (c >= 'a' && c <= 'z') {
                lit_buf[lit_buf_pos] = c;
                lit_buf_pos++;
                assert(lit_buf_pos < 1023);
            }

            if ((is_digit || c == '\n') && lit_buf_pos > 0) {
                lit_buf[lit_buf_pos] = 0;
                lit_buf_pos++;

                int num;
                char* lit_buf_iter = lit_buf;
                while ((num = parse_literal(&lit_buf_iter, &lit_buf[lit_buf_pos])) > 0) {
                    if (first == -1) {
                        first = num;
                    } else {
                        second = num;
                    }
                }

                lit_buf_pos = 0;
            }

            if (is_digit) {
                const int num = c - '0';
                if (first == -1) {
                    first = num;
                } else {
                    second = num;
                }
            }
        }

        assert(first != -1);

        if (second == -1) {
            second = first;
        }

        printf("%i%i\n", first, second);
        acc += first * 10 + second;
    }

    printf("\n%i\n", acc);
    return 0;
}
