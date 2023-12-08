import regex as re


def main(file: str, part: int):
    with open(file, "r", newline="") as f:
        lines = f.readlines()
        lines = [d.strip() for d in lines]


if __name__ == "__main__":
    main("day_5_test_input.txt", 1)
    # main("day_5_input.txt", 1)
    # main("day_5_test_input.txt", 2)
    # main("day_5_input.txt", 2)
