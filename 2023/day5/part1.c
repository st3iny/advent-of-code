#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <sys/param.h>

typedef enum {
    SEEDS,
    SKIP,
    PARSE_MAP_NAME,
    PARSE_MAP_DEST,
    PARSE_MAP_SRC,
    PARSE_MAP_RANGE,
    EVALUATE,
} State;

#define VEC_DEFAULT_CAPACITY 1024

#define vec_new(entry_t) \
    { \
        .entries = malloc(VEC_DEFAULT_CAPACITY * sizeof(entry_t)), \
        .capacity = VEC_DEFAULT_CAPACITY, \
        .count = 0, \
    };

#define vec_push(vec, entry) { \
    if (vec.count + 1 > vec.capacity) { \
        vec.capacity = MAX(vec.capacity * 2, VEC_DEFAULT_CAPACITY); \
        vec.entries = realloc(vec.entries, vec.capacity * sizeof(vec.entries[0])); \
    } \
    vec.entries[vec.count++] = entry; \
}

#define vec_free(vec) { \
    free(vec.entries); \
    vec.entries = NULL; \
    vec.capacity = 0; \
    vec.count = 0; \
}

typedef struct {
    unsigned int src;
    unsigned int dest;
    unsigned int range;
} MapEntry;

typedef struct {
    MapEntry* entries;
    size_t capacity;
    size_t count;
} Map;

typedef struct {
    unsigned int* entries;
    size_t capacity;
    size_t count;
} Seeds;

unsigned int map_translate(Map map, unsigned int src) {
    for (size_t i = 0; i < map.count; i++) {
        const MapEntry entry = map.entries[i];
        if (src >= entry.src && src <= entry.src + entry.range) {
            return src - entry.src + entry.dest;
        }
    }
    return src;
}

void evaluate(Seeds seeds, Seeds translated, Map map) {
    for (size_t i = 0; i < translated.count; i++) {
        unsigned int src = translated.entries[i];
        translated.entries[i] = map_translate(map, src);
        printf("[%10u]: %10u -> %10u\n", seeds.entries[i], src, translated.entries[i]);
    }
    printf("\n");
}

int main() {
    State state = SEEDS;
    Seeds seeds = vec_new(unsigned int); // For debugging
    Seeds translated = vec_new(unsigned int);
    Map map = vec_new(MapEntry);

    char* line = NULL;
    size_t len = 0;
    ssize_t bytes;
    while ((bytes = getline(&line, &len, stdin)) != -1) {
        printf("%s", line);

        switch (state) {
            case SKIP:
                state = PARSE_MAP_NAME;
                continue;
            case PARSE_MAP_NAME:
                map.count = 0;
                state = PARSE_MAP_DEST;
                continue;
        }

        unsigned int num = 0;
        MapEntry entry;

        char* iter = line;
        while (iter <= &line[bytes]) {
            const char c = *iter++;

            if (c >= '0' && c <= '9') {
                num = num * 10 + c - '0';
            }

            switch (state) {
                case SEEDS:
                    if ((c == ' ' || c == '\n') && num != 0) {
                        vec_push(seeds, num);
                        vec_push(translated, num);
                        num = 0;
                    }
                    break;
                case PARSE_MAP_DEST:
                    if (c == ' ') {
                        entry.dest = num;
                        num = 0;
                        state = PARSE_MAP_SRC;
                    } else if (c == '\n') {
                        state = EVALUATE;
                    }
                    break;
                case PARSE_MAP_SRC:
                    if (c == ' ') {
                        entry.src = num;
                        num = 0;
                        state = PARSE_MAP_RANGE;
                    }
                    break;
                case PARSE_MAP_RANGE:
                    if (c == '\n') {
                        entry.range = num;
                        num = 0;
                        vec_push(map, entry);
                        state = PARSE_MAP_DEST;
                    }
                    break;
            }
        }

        switch (state) {
            case SEEDS:
                assert(translated.count == seeds.count);
                state = SKIP;
                break;
            case EVALUATE:
                evaluate(seeds, translated, map);
                state = PARSE_MAP_NAME;
                break;
        }
    }

    printf("\n");
    evaluate(seeds, translated, map);

    unsigned int nearest = UINT_MAX;
    for (size_t i = 0; i < translated.count; i++) {
        unsigned int location = translated.entries[i];
        if (location < nearest) {
            nearest = location;
        }
    }
    assert(nearest != UINT_MAX);

    vec_free(seeds);
    vec_free(translated);
    vec_free(map);

    printf("\n%i\n", nearest);
    return 0;
}
