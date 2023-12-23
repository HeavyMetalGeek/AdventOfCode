from __future__ import annotations


def get_lines(file: str):
    with open(file, "r", newline="") as f:
        lines = f.readlines()
        lines = [d.strip() for d in lines]

        return lines


def compute_delta(values: list[int]) -> list[int]:
    new_values = []
    index = 1
    for v in values[1:]:
        new_values.append(values[index] - values[index - 1])
        index += 1

    return new_values


def compute_level(values: list[int], levels: list[int]) -> list[int]:
    new_list = compute_delta(values)

    if all(d == 0 for d in new_list):
        levels.append(new_list)
        return levels
    else:
        levels.append(new_list)
        return compute_level(new_list, levels)


def main(file: str, part: int):
    lines = get_lines(file)

    if part == 1:
        total = 0
        for line in lines:
            # print(line)
            values = [int(d) for d in line.split()]
            levels = []
            levels.append(values)
            levels = compute_level(values, levels)

            levels.reverse()
            for index, lvl in enumerate(levels[1:]):
                previous_level = levels[index]
                this_level = levels[index + 1]

                previous_value = previous_level[-1]
                this_value = this_level[-1]
                predicted_value = this_value + previous_value
                this_level.append(predicted_value)
                #    print("." * index, predicted_value)

            total += levels[-1][-1]

        print("Part 1 total =", total)

    if part == 2:
        total = 0
        for line in lines:
            values = [int(d) for d in line.split()]
            levels = []
            levels.append(values)
            levels = compute_level(values, levels)

            levels.reverse()

            for index, lvl in enumerate(levels[1:]):
                previous_level = levels[index]
                this_level = levels[index + 1]

                previous_value = previous_level[0]
                this_value = this_level[0]
                predicted_value = this_value - previous_value
                this_level.insert(0, predicted_value)
                # print("." * index, predicted_value)

            total += levels[-1][0]
        print("Part 2 total =", total)


if __name__ == "__main__":
    main("day_9_test_input.txt", 1)
    main("day_9_input.txt", 1)
    main("day_9_test_input.txt", 2)
    main("day_9_input.txt", 2)
