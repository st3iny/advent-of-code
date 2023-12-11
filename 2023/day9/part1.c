#include <assert.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>

int main() {
    int acc = 0;
    char* line = NULL;
    size_t len = 0;
    ssize_t bytes;
    while ((bytes = getline(&line, &len, stdin)) != -1) {
        printf("%s", line);

        int* levels[128];
        size_t levels_length = 0;

        int* root = malloc(sizeof(int) * 128);
        size_t root_length = 0;

        int num = 0;
        bool negative = false;
        for (size_t i = 0; i < bytes; i++) {
            const char c = line[i];
            if (c >= '0' && c <= '9') {
                num = num * 10 + c - '0';
            } else if (c == '-') {
                negative = true;
            } else {
                if (negative) {
                    num *= -1;
                }

                root[root_length++] = num;
                assert(root_length <= 128);

                printf("%i ", num);
                num = 0;
                negative = false;
            }
        }
        printf("\n");

        levels[levels_length++] = root;
        assert(levels_length <= 128);

        int* last_level = root;
        int depth = 0;
        bool is_zero;
        do {
            int* level = malloc(sizeof(int) * 128);
            size_t level_length = 0;

            is_zero = true;
            for (size_t i = 1; i < root_length - depth; i++) {
                const int diff = last_level[i] - last_level[i - 1];
                printf("%i ", diff);

                if (root_length - depth <= 2 && diff != 0) {
                    printf("\nERROR: Sequence yielded a single, non-zero element: %i\n", diff);
                    return 1;
                }

                if (diff != 0) {
                    is_zero = false;
                }

                level[level_length++] = diff;
                assert(level_length <= 128);
            }
            printf("\n");

            levels[levels_length++] = level;
            assert(levels_length <= 128);

            last_level = level;
            depth++;
        } while (!is_zero);

        printf("0");
        int next = 0;
        depth--;
        for (ssize_t i = levels_length - 2; i >= 0; i--) {
            const int inc = levels[i][root_length - depth - 1];
            printf(" + %i", inc);
            next += inc;
            depth--;
        }
        assert(depth == -1);
        printf(" = %i\n\n", next);

        for (size_t i = 0; i < levels_length; i++) {
            free(levels[i]);
        }

        acc += next;
    }

    printf("%i\n", acc);
    return 0;
}
