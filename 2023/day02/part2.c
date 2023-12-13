#include <stdio.h>

#define RED "\e[0;31m"
#define GREEN "\e[0;32m"
#define BLUE "\e[0;34m"
#define YELLOW "\e[0;33m"
#define CRESET "\e[0m"

typedef struct {
    int red;
    int green;
    int blue;
} Game;

void game_print(Game game) {
    printf("%sr%i%s, %sg%i%s, %sb%i%s; ",
        RED, game.red, CRESET,
        GREEN, game.green, CRESET,
        BLUE, game.blue, CRESET
    );
}

int main() {
    int acc = 0;
    char* line = NULL;
    size_t len = 0;
    ssize_t bytes;
    while ((bytes = getline(&line, &len, stdin)) != -1) {
        line[bytes - 1] = ';';
        printf("%s\n", line);

        char* iter = &line[5]; // Skip "Game "
        char c;

        Game minimal = {0};

        // Parse ID
        int id = 0;
        while ((c = *(iter++)) != ':') {
            id = id * 10 + c - '0';
        }
        printf("%i: ", id);

        // Parse games and find minimal set of cubes
        while (iter < &line[bytes]) {
            Game game = {0};
            int current_color = 0;
            while ((c = *(iter++)) != ';') {
                if (c >= '0' && c <= '9') {
                    current_color = current_color * 10 + c - '0';
                } else if (c >= 'a' && c <= 'z') {
                    if (c == 'r') {
                        game.red = current_color;
                        iter += 2; // Skip "ed"
                    } else if (c == 'g') {
                        game.green = current_color;
                        iter += 4; // Skip "reen"
                    } else if (c == 'b') {
                        game.blue = current_color;
                        iter += 3; // Skip "lue"
                    } else {
                        printf("\nERROR: Invalid color: %c\n", c);
                        return 1;
                    }

                    current_color = 0;
                }
            }

            game_print(game);
            if (game.red > minimal.red) {
                minimal.red = game.red;
            }
            if (game.green > minimal.green) {
                minimal.green = game.green;
            }
            if (game.blue > minimal.blue) {
                minimal.blue = game.blue;
            }
        }

        printf("min: ");
        game_print(minimal);
        printf("\n\n");

        acc += minimal.red * minimal.green * minimal.blue;
    }

    printf("%i\n", acc);
    return 0;
}
