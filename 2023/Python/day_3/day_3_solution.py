from __future__ import annotations
import numpy as np
import regex as re
import math
from scipy.spatial import KDTree


def auto_id():
    for i in range(1000000):
        yield int(i)


class ValueBase:
    def __init__(self, bounding_box: BoundingBox):
        self.bounding_box = bounding_box


class SymbolValue(ValueBase):
    def __init__(self, bounding_box: BoundingBox, value: str):
        super(SymbolValue, self).__init__(bounding_box)
        self.value = value

    def __str__(self):
        return f"{self.value}; {self.bounding_box.start}"


class PartNumber(ValueBase):
    def __init__(self, bounding_box: BoundingBox, part_number: int, id_factory):
        super(PartNumber, self).__init__(bounding_box)
        self.part_number = part_number
        self.Id = int(id_factory.__next__())

    def __str__(self):
        return (
            f"{self.part_number}; {self.bounding_box.start} - {self.bounding_box.end}"
        )


class Point:
    def __init__(self, x: int, y: int):
        self.x = x
        self.y = y

    def __str__(self):
        return f"{self.x}, {self.y}"

    def distance(self, other: Point) -> float:
        return math.sqrt((self.x - other.x) ** 2 + (self.y - other.y) ** 2)

    def ToTuple(self) -> tuple[float, float]:
        return (self.x, self.y)


class BoundingBox:
    def __init__(self, start: Point, end: Point):
        self.start = start
        self.end = end
        self.mid = Point((start.x + end.x) / 2, (start.y + end.y) / 2)

    def min_distance(self, p: Point):
        # I ... Don't really like this anymore
        # dist_mid = Line(self.start, self.end).distance(p)
        dist_mid = p.distance(self.mid)
        dist_start = p.distance(self.start)
        dist_end = p.distance(self.end)

        return min(dist_mid, dist_start, dist_end)


class Board:
    def __init__(self, input: list[str]):
        self.symbols = []
        self.values = []
        input = [d.strip() for d in input]
        width = len(input[0])  # manual inspection the grid has a constant width
        height = len(input)

        self.grid = np.chararray([height, width], unicode=True)

        for i, j, _ in self.grid_enumerator():
            self.grid[i, j] = input[i][j]

    def grid_enumerator(self) -> tuple[int, int]:
        next_row = True
        for i in range(0, self.grid.shape[0]):
            next_row = True
            for j in range(0, self.grid.shape[1]):
                yield (i, j, next_row)
                next_row = False


def main(file: str, part: int):
    id_factory = auto_id()
    zero_code = ord("0")
    nine_code = ord("9")

    part_number_buffer = []
    part_number_start = Point(0, 0)
    part_numer_end = Point(0, 0)
    with open(file, "r", newline="") as f:
        board = Board(f.readlines())

        for i, j, next_row in board.grid_enumerator():
            # Maybe more efficient than regex searching?
            value = board.grid[i, j]
            code = ord(value)

            if next_row == True:
                if len(part_number_buffer) > 0:
                    part_numer_end = Point(i - 1, board.grid.shape[1])
                    part_number = int("".join(d for d in part_number_buffer))
                    board.values.append(
                        PartNumber(
                            BoundingBox(part_number_start, part_numer_end),
                            part_number,
                            id_factory,
                        )
                    )

                    part_number_buffer.clear()
                    part_number_start = Point(0, 0)
                    part_number_end = Point(0, 0)

            if code >= zero_code and code <= nine_code:
                if len(part_number_buffer) == 0:
                    part_number_start = Point(i, j)
                part_number_buffer.append(value)
            else:
                if len(part_number_buffer) > 0:
                    part_numer_end = Point(i, j - 1)
                    part_number = int("".join(d for d in part_number_buffer))
                    board.values.append(
                        PartNumber(
                            BoundingBox(part_number_start, part_numer_end),
                            part_number,
                            id_factory,
                        )
                    )

                    part_number_buffer.clear()
                    part_number_start = Point(0, 0)
                    part_number_end = Point(0, 0)

                if value != ".":
                    board.symbols.append(
                        SymbolValue(BoundingBox(Point(i, j), Point(i, j)), value)
                    )

        total = 0
        if part == 1:
            symbol_tree = KDTree(
                [
                    [d.bounding_box.start.x, d.bounding_box.start.y]
                    for d in board.symbols
                ]
            )
            for v in board.values:
                start_dist = symbol_tree.query(v.bounding_box.start.ToTuple())[0]
                mid_dist = symbol_tree.query(v.bounding_box.mid.ToTuple())[0]
                end_dist = symbol_tree.query(v.bounding_box.end.ToTuple())[0]
                min_distance = min(start_dist, mid_dist, end_dist)

                if min_distance <= 1 or abs(min_distance - 1.4142135623730951) < 1e-6:
                    total += v.part_number

        if part == 2:
            symbol_tree = KDTree(
                [
                    [d.bounding_box.start.x, d.bounding_box.start.y]
                    for d in board.symbols
                    if d.value == "*"
                ]
            )

            # Semi lazy hack. I need to be able to index into the value array, so I just duplicate it for each point
            # this lets me query any point and get the value
            unpacked_values = []
            duplicated_values = []
            for v in board.values:
                unpacked_values.append([v.bounding_box.start.x, v.bounding_box.start.y])
                unpacked_values.append([v.bounding_box.mid.x, v.bounding_box.mid.y])
                unpacked_values.append([v.bounding_box.end.x, v.bounding_box.end.y])
                duplicated_values.append(v)
                duplicated_values.append(v)
                duplicated_values.append(v)

            value_tree = KDTree([[d[0], d[1]] for d in unpacked_values])

            # Jeez... It sucks that I can't "bundle" the tree with a value
            # this is akward as F
            for s in [d for d in board.symbols if d.value == "*"]:
                v = value_tree.query(s.bounding_box.start.ToTuple(), 6)
                distances = v[0]
                valid_distances = []
                for d in distances:
                    if ez_dist_check(d):
                        valid_distances.append(True)
                    else:
                        valid_distances.append(False)
                indexes = v[1][valid_distances]
                candidates = [duplicated_values[d] for d in indexes]
                distinct_candidates = []
                for c in candidates:
                    blah = [d.Id for d in distinct_candidates]
                    if not c.Id in blah:
                        distinct_candidates.append(c)

                if len(distinct_candidates) == 2:
                    candidate_1 = distinct_candidates[0]
                    candidate_2 = distinct_candidates[1]
                    gear_ratio = candidate_1.part_number * candidate_2.part_number
                    total += gear_ratio

        print("Total is:", total)


def ez_min_distance(symbol_tree, v: PartNumber):
    start_dist = symbol_tree.query(v.bounding_box.start.ToTuple())[0]
    mid_dist = symbol_tree.query(v.bounding_box.mid.ToTuple())[0]
    end_dist = symbol_tree.query(v.bounding_box.end.ToTuple())[0]
    min_distance = min(start_dist, mid_dist, end_dist)

    return min_distance


def ez_dist_check(dist_1):
    return dist_1 <= 1 or abs(dist_1 - 1.4142135623730951) < 1e-6


if __name__ == "__main__":
    main("day_3_test_input.txt", 1)
    main("day_3_input.txt", 1)
    main("day_3_test_input.txt", 2)
    main("day_3_input.txt", 2)
