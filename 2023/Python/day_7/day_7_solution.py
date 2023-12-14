from __future__ import annotations
import numpy as np
import math
import functools


def get_lines(file: str):
    with open(file, "r", newline="") as f:
        lines = f.readlines()
        lines = [d.strip() for d in lines]

        return lines


class Hand:
    card_ranking = {
        "2": 2,
        "3": 3,
        "4": 4,
        "5": 5,
        "6": 6,
        "7": 7,
        "8": 8,
        "9": 9,
        "T": 10,
        "J": 11,
        "Q": 12,
        "K": 13,
        "A": 14,
    }
    card_ranking_2 = card_ranking.copy()
    card_ranking_2["J"] = 1

    def __init__(self, line: str):
        cards, bid = line.split(" ")
        self.cards = cards
        self.bid = int(bid)

    def __str__(self):
        return f"{self.cards}"

    def get_rank(self) -> int:
        buckets = {}
        for card in self.cards:
            if card in buckets.keys():
                buckets[card] += 1
            else:
                buckets[card] = 1
        asdf = [d for d in buckets.values()]
        asdf.sort(reverse=True)

        if asdf[0] == 5:
            # print("five of a kind")
            return 7
        if asdf[0] == 4 and asdf[1] == 1:
            # print("four of a kind")
            return 6
        if asdf[0] == 3 and asdf[1] == 2:
            # print("full house")
            return 5
        if asdf[0] == 3 and asdf[1] == 1 and asdf[1] == 1:
            # print("three of a kind")
            return 4
        if asdf[0] == 2 and asdf[1] == 2 and asdf[2] == 1:
            # print("two of a kind")
            return 3
        if asdf[0] == 2 and asdf[1] == 1 and asdf[2] == 1 and asdf[3] == 1:
            # print("one pair")
            return 2
        else:
            # print("high card")
            return 1

    def get_rank_2(self) -> int:
        buckets = {}
        for card in self.cards:
            if card in buckets.keys():
                buckets[card] += 1
            else:
                buckets[card] = 1

        if "J" in buckets.keys():
            max_value = 0
            mak_key = None
            for k, v in buckets.items():
                if k != "J" and v > max_value:
                    max_value = v
                    max_key = k

            # Protect against "JJJJJ"
            if len(buckets) > 1:
                buckets[max_key] = buckets["J"] + buckets[max_key]
                del buckets["J"]

        asdf = [d for d in buckets.values()]
        asdf.sort(reverse=True)

        if asdf[0] == 5:
            # print("five of a kind")
            return 7
        if asdf[0] == 4 and asdf[1] == 1:
            # print("four of a kind")
            return 6
        if asdf[0] == 3 and asdf[1] == 2:
            # print("full house")
            return 5
        if asdf[0] == 3 and asdf[1] == 1 and asdf[1] == 1:
            # print("three of a kind")
            return 4
        if asdf[0] == 2 and asdf[1] == 2 and asdf[2] == 1:
            # print("two of a kind")
            return 3
        if asdf[0] == 2 and asdf[1] == 1 and asdf[2] == 1 and asdf[3] == 1:
            # print("one pair")
            return 2
        else:
            # print("high card")
            return 1


def compare(hand1: Hand, hand2: Hand) -> int:
    rank1 = hand1.get_rank()
    rank2 = hand2.get_rank()

    if rank1 < rank2:
        return -1
    elif rank1 > rank2:
        return 1
    else:
        for a, b in zip(hand1.cards, hand2.cards):
            if hand1.card_ranking[a] < hand2.card_ranking[b]:
                return -1
            elif hand1.card_ranking[a] > hand2.card_ranking[b]:
                return 1

        return 0  # totally identical, probably comparing self (why python???)


def compare_2(hand1: Hand, hand2: Hand) -> int:
    rank1 = hand1.get_rank_2()
    rank2 = hand2.get_rank_2()

    if rank1 < rank2:
        return -1
    elif rank1 > rank2:
        return 1
    else:
        for a, b in zip(hand1.cards, hand2.cards):
            if hand1.card_ranking_2[a] < hand2.card_ranking_2[b]:
                return -1
            elif hand1.card_ranking_2[a] > hand2.card_ranking_2[b]:
                return 1

        return 0  # totally identical, probably comparing self (why python???)


def main(file: str, part: int):
    lines = get_lines(file)

    if part == 1:
        hands = []
        for line in lines:
            hand = Hand(line)
            hands.append(hand)

        hands.sort(key=functools.cmp_to_key(compare))

        total_winnings = 0
        for index, hand in enumerate(hands):
            total_winnings += hand.bid * (index + 1)

        print("Part 1 total winnings =", total_winnings)
    if part == 2:
        hands = []
        for line in lines:
            hand = Hand(line)
            hands.append(hand)

        hands.sort(key=functools.cmp_to_key(compare_2))

        total_winnings = 0
        for index, hand in enumerate(hands):
            total_winnings += hand.bid * (index + 1)

        print("Part 2 total winnings =", total_winnings)


if __name__ == "__main__":
    main("day_7_test_input.txt", 1)
    main("day_7_input.txt", 1)
    main("day_7_test_input.txt", 2)
    main("day_7_input.txt", 2)
