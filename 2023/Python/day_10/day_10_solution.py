from __future__ import annotations
from enum import IntEnum
from enum import auto


class Direction(IntEnum):
    NONE = 0
    NORTH = auto()
    SOUTH = auto()
    EAST = auto()
    WEST = auto()


class Node:
    # Nodes can only have two directions
    direction: tuple[Direction, Direction]
    row: int
    col: int
    value: str

    def __init__(self, width: int, raw_value: str, row: int, col: int):
        self.value = raw_value
        match raw_value:
            case "|":
                self.direction = (Direction.NORTH, Direction.SOUTH)
            case "-":
                self.direction = (Direction.EAST, Direction.WEST)
            case "L":
                self.direction = (Direction.NORTH, Direction.EAST)
            case "J":
                self.direction = (Direction.NORTH, Direction.WEST)
            case "7":
                self.direction = (Direction.SOUTH, Direction.WEST)
            case "F":
                self.direction = (Direction.SOUTH, Direction.EAST)
            case ".":
                self.direction = (Direction.NONE, Direction.NONE)
            case "S":
                self.direction = (Direction.NONE, Direction.NONE)

        self.row = row
        self.col = col

        self.index = row * width + col

    def __str__(self):
        return f"{self.value}, ({self.row}, {self.col})"


class Map:
    def __init__(self, lines: list[str]):
        self.field = []
        self.width = len(lines[0])
        self.height = len(lines)
        for row, line in enumerate(lines):
            for col, char in enumerate(line):
                n = Node(self.width, char, row, col)
                self.field.append(n)

    def __getitem__(self, pos):
        row, col = pos
        index = row * self.width + col
        return self.field[index]

    def __iter__(self):
        return iter(self.field)


def get_lines(file: str):
    with open(file, "r", newline="") as f:
        lines = f.readlines()
        lines = [d.strip() for d in lines]

        return lines


def main(file: str, part: int):
    lines = get_lines(file)

    if part == 1:
        # Build a map
        m = Map(lines)
        start_node = next(filter(lambda d: "S" in d.value, m))

        path = []
        # need to "seed" the start, we could guess, but the data is "nice"
        # and I can just look for it.
        offset = (1, 0)
        next_node = m[
            (start_node.row + offset[0]) % m.width,
            (start_node.col + offset[1]) % m.height,
        ]

        this_node = next_node

        def ez_offset(row_offset: int, col_offset: int) -> Node:
            offset_node = m[
                (start_node.row + row_offset) % m.width,
                (start_node.col + col_offset) % m.height,
            ]
            return offset_node

        while next_node.value != "S":
            candidates = []

            if (
                Direction.NORTH in this_node.direction
                and Direction.SOUTH in this_node.direction
            ):
                candidates.append(ez_offset(1, 0))
                candidates.append(ez_offset(-1, 0))
            if (
                Direction.EAST in this_node.direction
                and Direction.WEST in this_node.direction
            ):
                candidates.append(ez_offset(0, 1))
                candidates.append(ez_offset(0, -1))
            if (
                Direction.NORTH in this_node.direction
                and Direction.EAST in this_node.direction
            ):
                candidates.append(ez_offset(-1, 1))
                candidates.append(ez_offset(1, -1))
            if (
                Direction.NORTH in this_node.direction
                and Direction.WEST in this_node.direction
            ):
                candidates.append(ez_offset(-1, -1))
                candidates.append(ez_offset(1, 1))
            if (
                Direction.SOUTH in this_node.direction
                and Direction.WEST in this_node.direction
            ):
                candidates.append(ez_offset(1, -1))
                candidates.append(ez_offset(-1, 1))
            if (
                Direction.SOUTH in this_node.direction
                and Direction.EAST in this_node.direction
            ):
                candidates.append(ez_offset(1, 1))
                candidates.append(ez_offset(-1, -1))

            # print(len(candidates))
            # for d in candidates:
            #     print(d, d.direction)
            # exit()

            # This only works because nodes only have one entrance and one exit
            for n in candidates:
                if Direction.NONE in n.direction:
                    print("none")
                    continue
                else:
                    this_node = n
                    path.append(n)
                    print(n)
                    break

        print(path)

    if part == 2:
        print("Not implemented yet")


if __name__ == "__main__":
    main("day_10_test_input.txt", 1)
    # main("day_10_input.txt", 1)
    # main("day_10_test_input.txt", 2)
    # main("day_10_input.txt", 2)
