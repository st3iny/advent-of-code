#include <assert.h>
#include <stdbool.h>
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

int main() {
    char* sequence = NULL;
    bool parse_sequence = true;

    Node nodes[1024];
    size_t nodes_length = 0;

    Node* first = NULL;

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

            if (strcmp(node.name, "AAA") == 0) {
                assert(first == NULL);
                first = &nodes[nodes_length - 1];
            }
        }
    }
    assert(sequence != NULL);
    assert(first != NULL);

    const size_t sequence_length = strlen(sequence);

    size_t steps = 0;
    Node* current = first;
    while (true) {
        const char inst = sequence[steps++ % sequence_length];
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

        printf("[%5zu]: %s -> %s\n", steps, current->name, next_name);

        if (strcmp(next_name, "ZZZ") == 0) {
            break;
        }

        current = find_node(nodes, nodes_length, next_name);
        assert(current != NULL);
    }

    free(sequence);

    printf("\n%zu\n", steps);
    return 0;
}
