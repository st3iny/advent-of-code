#include <assert.h>
#include <stdbool.h>
#include <stdio.h>

#define GREEN "\e[0;32m"
#define YELLOW "\e[0;33m"
#define CRESET "\e[0m"

int main() {
    int acc = 0;
    char* line = NULL;
    size_t len = 0;
    ssize_t bytes;
    while ((bytes = getline(&line, &len, stdin)) != -1) {
        printf("%s", line);

        char* iter = &line[8]; // Skip "Card x: "
        char c;

        int numbers[1024];
        size_t numbers_len = 0;

        bool parse = true;

        int score = 1;
        int num = 0;
        while (iter < &line[bytes]) {
            const char c = *iter++;

            if (c >= '0' && c <= '9') {
                num = num * 10 + c - '0';
            } else if (c == '|') {
                parse = false;
            } else if (num != 0) {
                if (parse) {
                    numbers[numbers_len++] = num;
                    assert(numbers_len < 1024);
                } else {
                    for (size_t i = 0; i < numbers_len; i++) {
                        if (numbers[i] == num) {
                            score *= 2;
                            printf("%s", GREEN);
                            break;
                        }
                    }
                    printf("%2i%s ", num, CRESET);
                }
                num = 0;
            }
        }

        score /= 2;
        printf("score: %s%i%s\n\n", YELLOW, score, CRESET);
        acc += score;
    }

    printf("%i\n", acc);
    return 0;
}
