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

    def __init__(self, width: int, char: str, row: int, col: int):
        match char:
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


class Map:
    def __init__(self, lines: list[str]):
        self.field = []
        self.width = len(lines[0])
        for row, line in enumerate(lines):
            for col, char in enumerate(line):
                n = Node(self.width, char, row, col)
                self.field.append(n)

    def __getitem__(self, pos):
        row, col = pos
        index = row * self.width + col
        return self.field[index]


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
        # Now what do we do?

    if part == 2:
        print("Not implemented yet")


if __name__ == "__main__":
    main("day_10_test_input.txt", 1)
    # main("day_10_input.txt", 1)
    # main("day_10_test_input.txt", 2)
    # main("day_10_input.txt", 2)
