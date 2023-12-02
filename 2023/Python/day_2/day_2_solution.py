import regex as re


class Bag:
    # TODO: is there a better way to specify these?
    game_id = 0
    red = []
    blue = []
    green = []

    def __init__(self, game_id: int, red: list, green: list, blue: list):
        self.game_id = game_id
        self.red = red
        self.green = green
        self.blue = blue

    def __str__(self):
        return f"Game Id: {self.game_id}, red={self.red}, blue={self.blue}, green={self.blue}"

    def test_valid(self, max_red: int, max_green: int, max_blue: int):
        return (
            max(self.red) <= max_red
            and max(self.green) <= max_green
            and max(self.blue) <= max_blue
        )

    def get_power(self) -> float:
        return max(self.red) * max(self.green) * max(self.blue)


def parse_line(line: str) -> Bag:
    game_id = int(re.match(r"Game (\d+)", line).group(1))
    cube_iterator = re.finditer(r"(\d+) (red|green|blue)", line)

    red = []
    green = []
    blue = []
    for cube in cube_iterator:
        color = cube.group(2)
        num = int(cube.group(1))

        match color:
            case "red":
                red.append(num)
            case "green":
                green.append(num)
            case "blue":
                blue.append(num)

    bag = Bag(game_id, red, green, blue)

    return bag


def main(input: str, part: int):
    total = 0
    total_power = 0
    with open(input, "r", newline="") as f:
        for line in f:
            bag = parse_line(line)
            if bag.test_valid(12, 13, 14):
                total += bag.game_id
            if part == 2:
                total_power += bag.get_power()

    if part == 1:
        print(f"Sum of IDs = {total}")
    if part == 2:
        print(f"Total power = {total_power}")


if __name__ == "__main__":
    # main("day_2_part_1_test_input.txt", 1)
    main("day_2_input.txt", 1)
    main("day_2_input.txt", 2)
