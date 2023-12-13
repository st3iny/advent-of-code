#include <assert.h>
#include <stdbool.h>
#include <stdio.h>

#define AT(pos) map[pos.y][pos.x]

typedef struct {
    size_t x;
    size_t y;
} Pos;

bool has_south(char c) {
    return c == '|' || c == '7' || c == 'F' || c == 'S';
}

bool has_west(char c) {
    return c == '-' || c == 'J' || c == '7' || c == 'S';
}

bool has_north(char c) {
    return c == '|' || c == 'L' || c == 'J' || c == 'S';
}

bool has_east(char c) {
    return c == '-' || c == 'L' || c == 'F' || c == 'S';
}

int main() {
    char map[1024][256];
    size_t width = 0;
    size_t height = 0;

    Pos start;

    char* line = NULL;
    size_t len = 0;
    ssize_t bytes;
    while ((bytes = getline(&line, &len, stdin)) != -1) {
        printf("%s", line);

        for (size_t col = 0; col < bytes; col++) {
            if (height == 0 && col + 1 > width) {
                width = col + 1;
                assert(width <= 256);
            }
            assert(col < width);

            map[height][col] = line[col];

            if (line[col] == 'S') {
                start.x = col;
                start.y = height;
            }
        }

        height++;
        assert(height <= 1024);
    }

    bool block_north = false;
    bool block_south = false;
    bool block_east = false;
    bool block_west = false;
    Pos pos = start;
    size_t steps = 0;
    do {
        Pos south = { .x = pos.x,     .y = pos.y + 1 };
        Pos west =  { .x = pos.x - 1, .y = pos.y     };
        Pos north = { .x = pos.x,     .y = pos.y - 1 };
        Pos east =  { .x = pos.x + 1, .y = pos.y     };

        if (!block_north && pos.y > 0 && has_north(AT(pos)) && has_south(AT(north))) {
            pos = north;
            block_north = false;
            block_south = true;
            block_east = false;
            block_west = false;
        } else if (!block_south && pos.y < height - 1 && has_south(AT(pos)) && has_north(AT(south))) {
            pos = south;
            block_north = true;
            block_south = false;
            block_east = false;
            block_west = false;
        } else if (!block_east && pos.x > 0 && has_east(AT(pos)) && has_west(AT(east))) {
            pos = east;
            block_north = false;
            block_south = false;
            block_east = false;
            block_west = true;
        } else if (!block_west && pos.x < width - 1 && has_west(AT(pos)) && has_east(AT(west))) {
            pos = west;
            block_north = false;
            block_south = false;
            block_east = true;
            block_west = false;
        }

        steps++;
    } while (start.x != pos.x || start.y != pos.y);

    printf("Detected loop after %zu steps\n", steps);

    printf("\n%zu\n", steps / 2);
    return 0;
}
