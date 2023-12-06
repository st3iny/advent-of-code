#include <assert.h>
#include <stdbool.h>
#include <stdio.h>

#define RACES_CAPACITY 8

typedef struct {
    int time;
    int distance;
} Race;

int main() {
    Race races[RACES_CAPACITY];
    size_t races_length = 0;

    bool parse_distances = false;

    char* line = NULL;
    size_t len = 0;
    ssize_t bytes;
    while ((bytes = getline(&line, &len, stdin)) != -1) {
        printf("%s", line);

        size_t races_i = 0;
        int num = 0;
        for (size_t i = 10; i < bytes; i++) { // Skip "Distance: "
            const char c = line[i];
            if (c >= '0' && c <= '9') {
                num = num * 10 + c - '0';
            } else if (num != 0) {
                assert(races_i < RACES_CAPACITY);
                if (parse_distances) {
                    races[races_i].distance = num;
                } else {
                    races[races_i].time = num;
                }
                num = 0;
                races_i++;
            }
        }

        parse_distances = true;
        races_length = races_i;
    }

    printf("\n");
    assert(races_length < RACES_CAPACITY);

    int acc = 1;
    for (size_t i = 0; i < races_length; i++) {
        const Race race = races[i];
        printf("[%zu] {time: %2i, distance: %4i}\n", i, race.time, race.distance);

        int winners = 0;
        for (int x = 1; x < race.time - 1; x++) {
            const int y = x * x - x * race.time + race.distance;
            if (y < 0) {
                winners++;
            }
        }
        acc *= winners;
    }

    printf("\n%i\n", acc);
    return 0;
}
