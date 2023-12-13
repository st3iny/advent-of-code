#include <assert.h>
#include <stdio.h>

int main() {
    int acc = 0;
    char* line = NULL;
    size_t len = 0;
    ssize_t bytes;
    while ((bytes = getline(&line, &len, stdin)) != -1) {
        printf("%s", line);

        char* iter = line;
        int first = -1, second = -1;
        for (char* iter = line; *iter != 0; iter += sizeof(char)) {
            char c = *iter;
            if (c >= '0' && c <= '9') {
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
