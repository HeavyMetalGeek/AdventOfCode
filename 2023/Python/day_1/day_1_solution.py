import numpy as np
import regex as re


def get_first_last_digit(input: str, part: int):
    zero_code = 48
    nine_code = 57

    string_nums_to_code_map = {
        "one": "1",
        "two": "2",
        "three": "3",
        "four": "4",
        "five": "5",
        "six": "6",
        "seven": "7",
        "eight": "8",
        "nine": "9",
    }

    first_index = None
    last_index = None
    first = None
    last = None

    if part == 2:
        for s in string_nums_to_code_map.keys():
            index = input.find(s)

            if index == -1:
                continue

            if first_index is None or index < first_index:
                first_index = index
                first = string_nums_to_code_map[s]

        for s in string_nums_to_code_map.keys():
            index = input.rfind(s)
            if index == -1:
                continue

            if last_index is None or index > last_index:
                last_index = index
                last = string_nums_to_code_map[s]

    for index, code in enumerate([ord(d) for d in input]):
        if code >= zero_code and code <= nine_code:
            if first_index is None or index < first_index:
                first_index = index
                first = chr(code)
                # We need to set last here in case there is only one digit
                if last_index is None:
                    last_index = index
                    last = chr(code)
            elif index > last_index:
                last = chr(code)
                last_index = index

    try:
        int(first)
    except:
        print(input)

    try:
        int(last)
    except:
        print(input)

    return (int(first), int(last))


def main(file: str, part: int):
    total = 0
    with open(file, "r", newline="") as f:
        for line in f:
            first, last = get_first_last_digit(line, part)
            calibration_value = first * 10 + last
            total += calibration_value

    print(total)


if __name__ == "__main__":
    # main("day_1_part_2_test_input.txt")
    main("day_1_input.txt", part=1)
    main("day_1_input.txt", part=2)
    print("All done!")
