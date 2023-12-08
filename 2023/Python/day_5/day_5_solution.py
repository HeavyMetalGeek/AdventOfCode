from __future__ import annotations
import regex as re
import numpy as np


class MapBase:
    def __init__(self, lines: list[str]):
        self.map = {}
        for line in lines:
            dest_start, source_start, length = [int(d) for d in line.split()]

            dest = [
                d
                for d in np.linspace(
                    dest_start,
                    dest_start + length,
                    length,
                    endpoint=False,
                    dtype=int,
                )
            ]

            source = [
                d
                for d in np.linspace(
                    source_start,
                    source_start + length,
                    length,
                    endpoint=False,
                    dtype=int,
                )
            ]

            self.map = self.map | dict(zip(source, dest))
        print(self.map[79])

    def __str__(self):
        return f"{self.dest_start, self.source_start, self.length}"


class SeedToSoilMap(MapBase):
    def __init__(self, line):
        super(SeedToSoilMap, self).__init__(line)

    def get_soil(self, seed: int):
        if seed >= self.dest_start and seed <= self.dest_end:
            print("in range")
            return (self.dest_end - self.dest_start) + seed
        else:
            print("not in range")
            return seed


def main(file: str, part: int):
    with open(file, "r", newline="") as f:
        lines = f.readlines()
        lines = [d.strip() for d in lines]
        seeds = [int(d) for d in lines[0].split(":")[1].split()]
        seed_to_soil_map = SeedToSoilMap([lines[3], lines[4]])
        soil = seed_to_soil_map.get_soil(79)
        print(soil)


if __name__ == "__main__":
    main("day_5_test_input.txt", 1)
    # main("day_5_input.txt", 1)
    # main("day_5_test_input.txt", 2)
    # main("day_5_input.txt", 2)
