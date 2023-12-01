#include <assert.h>
#include <stdio.h>
#include <stdlib.h>

int main() {
    int acc = 0;
    char* line = NULL;
    size_t len = 0;
    ssize_t bytes;
    while ((bytes = getline(&line, &len, stdin)) != -1) {
        printf("%s", line);

        char* iter = line;
        char first = 0, second = 0;
        for (char* iter = line; *iter != 0; iter += sizeof(char)) {
            char c = *iter;
            if (c >= '0' && c <= '9') {
                if (first == 0) {
                    first = c;
                } else {
                    second = c;
                }
            }
        }

        assert(first != 0);

        if (second == 0) {
            second = first;
        }

        char lit[] = {first, second, 0};
        printf("%s\n", lit);
        acc += atoi(lit);
    }

    printf("\n%i\n", acc);
    return 0;
}
