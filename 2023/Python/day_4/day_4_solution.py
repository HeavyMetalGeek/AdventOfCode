import regex as re


class Card:
    def __init__(self, line: str = None):
        if line is not None:
            match = re.match(r"Card +(\d+)", line)
            self.id = int(match.group(1))

            left, numbers = line.split("|")
            winning_numbers = left.split(":")[1]

            self.numbers = [int(d) for d in numbers.split(" ") if d != ""]
            self.winning_numbers = [
                int(d) for d in winning_numbers.split(" ") if d != ""
            ]
        else:
            self.id = 0
            self.numbers = []
            self.winning_numbers = []

    def __str__(self):
        return f"{self.id}: {self.winning_numbers} | {self.numbers}"

    def clone(cls):
        c = Card()
        c.id = cls.id
        c.numbers = cls.numbers.copy()
        c.winning_numbers = cls.winning_numbers.copy()

        return c

    def score(cls):
        intersection_count = len(set(cls.numbers) & set(cls.winning_numbers))
        total = 1 if intersection_count > 0 else 0
        for i in range(1, intersection_count):
            total *= 2

        return total

    def score2(cls):
        return len(set(cls.numbers) & set(cls.winning_numbers))


def main(file: str, part: int):
    with open(file, "r", newline="") as f:
        lines = f.readlines()
        lines = [d.strip() for d in lines]

        if part == 1:
            total = 0
            for line in lines:
                c = Card(line)
                total += c.score()

            print(f"Part 1 = {total}")
        if part == 2:
            total = 0
            # Build an initial pool of cards
            card_pool = [Card(d) for d in lines]

            for card in card_pool:
                total = count_winners(card_pool, card, total)
            print("part 2 total:", total)


def count_winners(card_pool: list[Card], c: Card, count: int) -> int:
    count += 1
    score = c.score2()
    if score > 0:
        for i in range(score):
            # count += 1
            if c.id + i >= len(card_pool):
                return count
            count = count_winners(card_pool, card_pool[c.id + i], count)

        return count
    else:
        return count


if __name__ == "__main__":
    main("day_4_test_input.txt", 1)
    main("day_4_input.txt", 1)
    main("day_4_test_input.txt", 2)
    main("day_4_input.txt", 2)
