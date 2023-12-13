#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define CARDS_PER_HAND 5

int char_compare(const void* a, const void* b) {
    return *(char*)a - *(char*)b;
}

typedef struct {
    char cards[CARDS_PER_HAND];
    int bid;
} Hand;

int hand_score(Hand hand) {
    char sorted[CARDS_PER_HAND];
    strncpy(sorted, hand.cards, CARDS_PER_HAND);
    qsort(sorted, CARDS_PER_HAND, 1, char_compare);

    int acc[2] = {0};
    size_t acc_i = 0;
    char last = 0;
    for (size_t i = 0; i < CARDS_PER_HAND; i++) {
        const char card = sorted[i];
        if (card == last && last != 0) {
            assert(acc_i < 2);
            if (acc[acc_i] == 0) {
                acc[acc_i] = 2;
            } else {
                acc[acc_i]++;
            }
        } else if (acc[acc_i] > 0) {
            acc_i++;
        }
        last = card;
    }

    if (acc[0] == 5) {
        // Five of a kind
        return 7;
    } else if (acc[0] == 4 || acc[1] == 4) {
        // Four of a kind
        return 6;
    } else if ((acc[0] == 2 && acc[1] == 3) || (acc[0] == 3 && acc[1] == 2)) {
        // Full house
        return 5;
    } else if (acc[0] == 3 || acc[1] == 3) {
        // Three of a kind
        return 4;
    } else if (acc[0] == 2 && acc[1] == 2) {
        // Two pairs
        return 3;
    } else if (acc[0] == 2 || acc[1] == 2) {
        // Pair
        return 2;
    }

    // High card
    return 1;
}

int card_score(char card) {
    switch (card) {
        case 'A':
            return 100;
        case 'K':
            return 99;
        case 'Q':
            return 98;
        case 'J':
            return 97;
        case 'T':
            return 96;
        default:
            assert(card >= '2' && card <= '9');
            return card - '0';
    }
}

int compare_hands(const void* a, const void* b) {
    const Hand hand1 = *(Hand*)a;
    const Hand hand2 = *(Hand*)b;

    int score_a = hand_score(hand1);
    int score_b = hand_score(hand2);

    if (score_a != score_b) {
        return score_a - score_b;
    }

    for (size_t i = 0; i < CARDS_PER_HAND; i++) {
        const int score_a = card_score(hand1.cards[i]);
        const int score_b = card_score(hand2.cards[i]);
        if (score_a > score_b) {
            return 1;
        } else if (score_a == score_b) {
            continue;
        } else {
            return -1;
        }
    }

    printf("ERROR: Tried to compare two equal hands: %s <-> %s\n", hand1.cards, hand2.cards);
    exit(1);
}

int main() {
    Hand hands[1024];
    size_t hands_length = 0;

    char* line = NULL;
    size_t len = 0;
    ssize_t bytes;
    while ((bytes = getline(&line, &len, stdin)) != -1) {
        printf("%s", line);

        Hand hand = { .bid = 0 };
        strncpy(hand.cards, line, CARDS_PER_HAND);

        char* iter = &line[6];
        char c;
        while ((c = *iter++) != '\n') {
            hand.bid = hand.bid * 10 + c - '0';
        }

        hands[hands_length++] = hand;
        assert(hands_length <= 1024);
    }
    printf("\n");

    qsort(hands, hands_length, sizeof(Hand), compare_hands);

    int acc = 0;
    for (size_t i = 0; i < hands_length; i++) {
        printf("[%4zu]: %s %4i -> %i\n",
            i + 1,
            hands[i].cards,
            hands[i].bid,
            hand_score(hands[i])
        );
        acc += (i + 1) * hands[i].bid;
    }

    printf("\n%i\n", acc);
    return 0;
}
