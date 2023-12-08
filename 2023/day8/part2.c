#include <assert.h>
#include <inttypes.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define NODE_ID_LEN 3

typedef struct {
    char name[NODE_ID_LEN + 1];
    char left[NODE_ID_LEN + 1];
    char right[NODE_ID_LEN + 1];
} Node;

Node* find_node(Node* nodes, size_t nodes_length, const char* name) {
    for (size_t i = 0; i < nodes_length; i++) {
        Node* node = &nodes[i];
        if (strcmp(node->name, name) == 0) {
            return node;
        }
    }

    return NULL;
}

int64_t gcd(int64_t a, int64_t b)
{
    if (b == 0) {
        return a;
    }

    return gcd(b, a % b);
}

int64_t lcm(int64_t arr[], size_t arr_length) {
    int64_t result = arr[0];

    for (size_t i = 1; i < arr_length; i++) {
        result = (arr[i] * result) / gcd(arr[i], result);
    }

    return result;
}

int main() {
    char* sequence = NULL;
    bool parse_sequence = true;

    Node nodes[1024];
    size_t nodes_length = 0;

    Node* firsts[1024];
    size_t firsts_length = 0;

    char* line = NULL;
    size_t len = 0;
    ssize_t bytes;
    while ((bytes = getline(&line, &len, stdin)) != -1) {
        printf("%s", line);

        // Skip empty line after sequence
        if (bytes == 1) {
            continue;
        }

        if (parse_sequence) {
            parse_sequence = false;
            sequence = (char*)malloc(bytes);
            strncpy(sequence, line, bytes - 1);
        } else {
            Node node;
            strncpy(node.name, line, NODE_ID_LEN);
            strncpy(node.left, &line[7], NODE_ID_LEN);
            strncpy(node.right, &line[12], NODE_ID_LEN);
            nodes[nodes_length++] = node;
            assert(nodes_length <= 1024);

            printf("[%s]: %s, %s\n\n", node.name, node.left, node.right);

            if (node.name[NODE_ID_LEN - 1] == 'A') {
                firsts[firsts_length++] = &nodes[nodes_length - 1];
                assert(firsts_length <= 1024);
            }
        }
    }

    assert(sequence != NULL);
    const size_t sequence_length = strlen(sequence);

    int64_t* steps = calloc(firsts_length, sizeof(int64_t));
    for (size_t i = 0; i < firsts_length; i++) {
        Node* current = firsts[i];

        while (true) {
            const char inst = sequence[steps[i]++ % sequence_length];
            char* next_name;
            switch (inst) {
                case 'L':
                    next_name = current->left;
                    break;
                case 'R':
                    next_name = current->right;
                    break;
                default:
                    printf("ERROR: Invalid instruction in sequence: %c\n", inst);
                    return 1;
            }

            if (next_name[NODE_ID_LEN - 1] == 'Z') {
                break;
            }

            current = find_node(nodes, nodes_length, next_name);
            assert(current != NULL);
        }

        printf("%zu took %"PRIi64" steps\n", i, steps[i]);
    }

    const int64_t steps_lcm = lcm(steps, firsts_length);

    free(sequence);
    free(steps);

    printf("\n%"PRIi64"\n", steps_lcm);
    return 0;
}
