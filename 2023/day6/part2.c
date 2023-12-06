#include <stdbool.h>
#include <stdio.h>

typedef struct {
    long long time;
    long long distance;
} Race;

int main() {
    Race race;
    bool parse_distances = false;

    char* line = NULL;
    size_t len = 0;
    ssize_t bytes;
    while ((bytes = getline(&line, &len, stdin)) != -1) {
        printf("%s", line);

        long long num = 0;
        for (size_t i = 10; i < bytes; i++) { // Skip "Distance: "
            const char c = line[i];
            if (c >= '0' && c <= '9') {
                num = num * 10 + c - '0';
            } else if (c == '\n') {
                if (parse_distances) {
                    race.distance = num;
                } else {
                    race.time = num;
                }
                num = 0;
            }
        }

        parse_distances = true;
    }

    printf("\n{time: %lli, distance: %lli}\n", race.time, race.distance);

    int acc = 0;
    for (long long x = 1; x < race.time - 1; x++) {
        const long long y = x * x - x * race.time + race.distance;
        if (y < 0) {
            acc++;
        }
    }

    printf("\n%i\n", acc);
    return 0;
}
