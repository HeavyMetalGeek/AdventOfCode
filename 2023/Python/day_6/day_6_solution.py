from __future__ import annotations
import regex as re
import scipy.optimize as opt
import random
import numpy as np
import math


def get_lines(file: str):
    with open(file, "r", newline="") as f:
        lines = f.readlines()
        lines = [d.strip() for d in lines]

        return lines


def compute_distance(race_duration: int, hold_time: int):
    if hold_time < 0 or hold_time > race_duration:
        return -1e9  # Penalty for going out of bounds

    speed = hold_time
    distance = (race_duration - hold_time) * speed

    return distance  # mm traveled


def main(file: str, part: int):
    lines = get_lines(file)
    # List comprehension to get two lists of integers
    # there's probably some zip method to do these both at the same time
    race_times = [int(d) for d in lines[0].split(":")[1].split()]
    record_distances = [int(d) for d in lines[1].split(":")[1].split()]

    if part == 1:
        # # Confirm I get the same output as the test input
        # for i in range(7):
        #     print(compute_distance(7, i))

        win_margins = []
        for race_duration, record_distance in zip(race_times, record_distances):
            # We want to maximize the distance, but optimize minimizes the function, so -1 to invert
            optimal_func = lambda x: compute_distance(race_duration, x) * -1

            # Search for the optimal hold time
            optimized_hold = opt.minimize(optimal_func, 1)

            # Get the integer hold time that results in the largest distance
            if optimal_func(math.floor(optimized_hold.x)) >= optimal_func(
                math.ceil(optimized_hold.x)
            ):
                optimized_hold = math.floor(optimized_hold.x)
            else:
                optimized_hold = math.ceil(optimized_hold.x)

            # split thee race on optimal time, and search the lower half for the record time
            low = 1
            high = optimized_hold - 0.1
            while low <= high:
                mid = low + (high - low) / 2
                distance = compute_distance(race_duration, mid)
                if record_distance - distance < 0:
                    high = mid - 0.000001
                else:
                    low = mid + 0.000001

            hold_lower = math.ceil(mid)

            # split the race on optimal time, and search the upper half for the record time
            low = optimized_hold + 0.1
            high = race_duration - 1
            while low <= high:
                mid = low + (high - low) / 2
                distance = compute_distance(race_duration, mid)
                if record_distance - distance > 0:
                    high = mid - 0.000001
                else:
                    low = mid + 0.000001

            hold_upper = math.floor(mid)

            # compute the win margin and store for later
            win_margins.append(hold_upper - hold_lower + 1)

        # multiply all the elements together to compute the answer
        print(f"Part 1 = {np.prod(win_margins)}")
    if part == 2:
        race_duration = int("".join(lines[0].split(":")[1].split()))
        record_distance = int("".join(lines[1].split(":")[1].split()))

        # copy pasta, because i'm lazy
        # We want to maximize the distance, but optimize minimizes the function, so -1 to invert
        optimal_func = lambda x: compute_distance(race_duration, x) * -1

        # Search for the optimal hold time
        optimized_hold = opt.minimize(optimal_func, 1)

        # Get the integer hold time that results in the largest distance
        if optimal_func(math.floor(optimized_hold.x)) >= optimal_func(
            math.ceil(optimized_hold.x)
        ):
            optimized_hold = math.floor(optimized_hold.x)
        else:
            optimized_hold = math.ceil(optimized_hold.x)

        # split thee race on optimal time, and search the lower half for the record time
        low = 1
        high = optimized_hold - 0.1
        while low <= high:
            mid = low + (high - low) / 2
            distance = compute_distance(race_duration, mid)
            if record_distance - distance < 0:
                high = mid - 0.000001
            else:
                low = mid + 0.000001

        hold_lower = math.ceil(mid)

        # split the race on optimal time, and search the upper half for the record time
        low = optimized_hold + 0.1
        high = race_duration - 1
        while low <= high:
            mid = low + (high - low) / 2
            distance = compute_distance(race_duration, mid)
            if record_distance - distance > 0:
                high = mid - 0.000001
            else:
                low = mid + 0.000001

        hold_upper = math.floor(mid)

        # multiply all the elements together to compute the answer
        print(f"Part 2 = {hold_upper - hold_lower + 1}")


if __name__ == "__main__":
    main("day_6_test_input.txt", 1)
    main("day_6_input.txt", 1)
    main("day_6_test_input.txt", 2)
    main("day_6_input.txt", 2)
