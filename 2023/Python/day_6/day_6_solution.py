from __future__ import annotations
import regex as re
import scipy.optimize as opt
import random


def main(file: str, part: int):
    with open(file, "r", newline="") as f:
        lines = f.readlines()
        lines = [d.strip() for d in lines]


if __name__ == "__main__":
    main("day_6_test_input.txt", 1)
    # main("day_6_input.txt", 1)
    # main("day_6_test_input.txt", 2)
    # main("day_6_input.txt", 2)
