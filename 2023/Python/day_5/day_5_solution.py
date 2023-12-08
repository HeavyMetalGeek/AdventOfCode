from __future__ import annotations
import regex as re
import numpy as np


class MapBase:
    def __init__(self):
        self.map = {}

    def __str__(self):
        return f"{self.dest_start, self.source_start, self.length}"

    def __getitem__(self, key):
        return self.get_value(key)

    def get_value(self, input: int):
        try:
            return self.map[input]
        except:
            return input

    def parse(self, lines: list[str]):
        for line in lines:
            dest_start, source_start, length = [int(d) for d in line.split() if d != ""]

            dest = [
                d
                for d in np.linspace(
                    dest_start,
                    dest_start + length,
                    length,
                    dtype=int,
                )
            ]

            source = [
                d
                for d in np.linspace(
                    source_start,
                    source_start + length,
                    length,
                    dtype=int,
                )
            ]

            self.map = self.map | dict(zip(source, dest))


# why did I do this...
class SeedToSoilMap(MapBase):
    def get_soil(self, seed: int):
        return self.get_value(seed)


class SoilToFertilizerMap(MapBase):
    def get_fertilizer(self, soil: int):
        self.get_value(soil)


class FertilizerToWaterMap(MapBase):
    def get_water(self, fertilizer: int):
        self.get_value(fertilizer)


class WaterToLightMap(MapBase):
    def get_light(self, water: int):
        self.get_value(water)


class LightToTemperatureMap(MapBase):
    def get_temperature(self, light: int):
        self.get_value(light)


class TemperatureToHumidityMap(MapBase):
    def get_humidity(self, temperature: int):
        self.get_value(temperature)


class HumidityToLocationMap(MapBase):
    def get_humidity(self, humidity: int):
        self.get_value(humidity)


def map_generator():
    maps = [
        SeedToSoilMap(),
        SoilToFertilizerMap(),
        FertilizerToWaterMap(),
        WaterToLightMap(),
        LightToTemperatureMap(),
        TemperatureToHumidityMap(),
        HumidityToLocationMap(),
    ]

    for m in maps:
        yield m


def main(file: str, part: int):
    with open(file, "r", newline="") as f:
        lines = f.readlines()
        lines = [d.strip() for d in lines]

        maps = [d for d in map_generator()]
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

        seeds = [int(d) for d in lines[0].split(":")[1].split()]

        for seed in seeds:
            print(seed_to_location(maps, seed))
        # print(maps[0])
        # exit()

        # seed_to_soil_map = maps[0]
        # print(seed_to_soil_map.get_soil(79))
        # print(seed_to_soil_map.get_soil(14))
        # print(seed_to_soil_map.get_soil(55))
        # print(seed_to_soil_map.get_soil(13))


def seed_to_location(maps: list[MapBase], seed: int):
    soil = maps[0][seed]
    fertilizer = maps[1][soil]
    water = maps[2][fertilizer]
    light = maps[3][water]
    temperature = maps[4][light]
    humidity = maps[5][temperature]
    location = maps[6][humidity]

    print("\tseed", seed)
    print("\tsoil", soil)
    print("\tfertalizer", fertilizer)
    print("\twater", water)
    print("\tlight", light)
    print("\ttemperature", temperature)
    print("\thumidity", humidity)

    return location


if __name__ == "__main__":
    main("day_5_test_input.txt", 1)
    # main("day_5_input.txt", 1)
    # main("day_5_test_input.txt", 2)
    # main("day_5_input.txt", 2)
