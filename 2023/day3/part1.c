#include <assert.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

bool is_sym(char c) {
    return c != '.' && (c < '0' || c > '9');
}

int main() {
    char* lines[1024] = {NULL};
    size_t lines_len = 0;

    char* line = NULL;
    size_t len = 0;
    ssize_t bytes;
    while ((bytes = getline(&line, &len, stdin)) != -1) {
        char* line_copy = (char*)malloc(bytes + 1);
        strcpy(line_copy, line);
        assert(line_copy[bytes - 1] == '\n');
        line_copy[bytes - 1] = '.';
        lines[lines_len++] = line_copy;
        assert(lines_len < 1024);
    }
    free(line);

    int acc = 0;
    for (size_t i = 0; i < lines_len; i++) {
        char* prevl = i > 0 ? lines[i - 1] : NULL;
        char* cur = lines[i];
        char* nextl = i + 1 < lines_len ? lines[i + 1] : NULL;

        int num = 0;
        bool is_part = false;
        bool was_num = false;

        size_t cur_len = strlen(cur);
        for (size_t j = 0; j < cur_len; j++) {
            char c = cur[j];

            bool is_num = c >= '0' && c <= '9';
            if (is_num) {
                num = num * 10 + c - '0';
                if ((!was_num && j > 0 && is_sym(cur[j - 1]))  // previous
                    || (j + 1 < cur_len && is_sym(cur[j + 1])) // next
                    || (prevl != NULL && j     > 0       && is_sym(prevl[j - 1])) // top left
                    || (nextl != NULL && j     > 0       && is_sym(nextl[j - 1])) // bottom left
                    || (nextl != NULL && j + 1 < cur_len && is_sym(nextl[j + 1])) // bottom right
                    || (prevl != NULL && j + 1 < cur_len && is_sym(prevl[j + 1])) // top right
                    || (prevl != NULL && is_sym(prevl[j])) // above
                    || (nextl != NULL && is_sym(nextl[j])) // below
                ) {
                    is_part = true;
                }
            }

            if (was_num && !is_num) {
                if (is_part) {
                    is_part = false;
                    acc += num;
                }
                num = 0;
            }

            was_num = is_num;
        }
    }

    for (size_t i = 0; i < lines_len; i++) {
        free(lines[i]);
    }
    lines_len = 0;

    printf("%i\n", acc);
    return 0;
}
