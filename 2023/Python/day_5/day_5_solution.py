from __future__ import annotations
import regex as re
import numpy as np


class MapBase:
    def __init__(self, source_start: int, dest_start: int, length: int):
        self.source_start = source_start
        self.dest_start = dest_start
        self.length = length
        self.source_end = source_start + length - 1
        self.dest_end = dest_start + length - 1
        self.offset = self.dest_start - self.source_start

        self.map_function = lambda x: x + self.offset if self.in_range(x) else x

    def __str__(self):
        return f"source_start: {self.source_start}; dest_start: {self.dest_start}; offset: {self.offset}"

    def in_range(self, x: int):
        return x >= self.source_start and x <= self.source_end


class MapHandler:
    def __init__(self):
        self.maps = []

    def __str__(self):
        return str(self.map)

    def __getitem__(self, key):
        return self.get_value(key)

    def get_value(self, input: int):
        for m in self.maps:
            if m.in_range(input):
                return m.map_function(input)

        return input

    def parse(self, lines: list[str]):
        for line in lines:
            dest_start, source_start, length = [int(d) for d in line.split() if d != ""]
            my_map = MapBase(source_start, dest_start, length)
            self.maps.append(my_map)


# why did I do this...
class SeedToSoilMap(MapHandler):
    def get_soil(self, seed: int):
        return self.get_value(seed)


class SoilToFertilizerMap(MapHandler):
    def get_fertilizer(self, soil: int):
        self.get_value(soil)


class FertilizerToWaterMap(MapHandler):
    def get_water(self, fertilizer: int):
        self.get_value(fertilizer)


class WaterToLightMap(MapHandler):
    def get_light(self, water: int):
        self.get_value(water)


class LightToTemperatureMap(MapHandler):
    def get_temperature(self, light: int):
        self.get_value(light)


class TemperatureToHumidityMap(MapHandler):
    def get_humidity(self, temperature: int):
        self.get_value(temperature)


class HumidityToLocationMap(MapHandler):
    def get_humidity(self, humidity: int):
        self.get_value(humidity)


def main(file: str, part: int):
    with open(file, "r", newline="") as f:
        lines = f.readlines()
        lines = [d.strip() for d in lines]

        maps = [
            SeedToSoilMap(),
            SoilToFertilizerMap(),
            FertilizerToWaterMap(),
            WaterToLightMap(),
            LightToTemperatureMap(),
            TemperatureToHumidityMap(),
            HumidityToLocationMap(),
        ]
        map_index = 0
        current_map = maps[map_index]
        line_pool = []
        for line in lines[3:]:
            if line == "":
                continue
            elif re.match("[a-z]", line) is not None:
                try:
                    current_map.parse(line_pool)
                    map_index += 1
                    current_map = maps[map_index]
                    line_pool.clear()
                except:
                    continue
            else:
                line_pool.append(line)

        # don't forget the last pool
        current_map.parse(line_pool)

        if part == 1:
            seeds = [int(d) for d in lines[0].split(":")[1].split()]
            locs = []
            for seed in seeds:
                locs.append(seed_to_location(maps, seed))
            print("min location (part 1):", min(locs))
        elif part == 2:
            seed_info = [int(d) for d in lines[0].split(":")[1].split()]
            seed_pairs = list(zip(seed_info[::2], seed_info[1::2]))
            min_location = 1e12
            for start, length in seed_pairs:
                end = start + length - 1
                low = start
                high = end

                while low <= high:
                    mid = int(low + (high - low) / 2)
                    location = seed_to_location(maps, mid)
                    if location < min_location:
                        min_location = location
                        high = mid - 1
                    else:
                        low = mid + 1

            print("min_location (part 2):", min_location)


def seed_to_location(maps: list[MapHandler], seed: int):
    soil = maps[0][seed]
    fertilizer = maps[1][soil]
    water = maps[2][fertilizer]
    light = maps[3][water]
    temperature = maps[4][light]
    humidity = maps[5][temperature]
    location = maps[6][humidity]

    # print("seed", seed)
    # print("\tsoil", soil)
    # print("\tfertalizer", fertilizer)
    # print("\twater", water)
    # print("\tlight", light)
    # print("\ttemperature", temperature)
    # print("\thumidity", humidity)
    # print("\tlocation", location)

    return location


if __name__ == "__main__":
    main("day_5_test_input.txt", 1)
    main("day_5_input.txt", 1)
    main("day_5_test_input.txt", 2)
    main("day_5_input.txt", 2)
