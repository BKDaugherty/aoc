from dataclasses import dataclass
import sys
from typing import IO, NewType


ElfId = NewType("ElfId", int)


@dataclass
class FoodItem:
    calorie_count: float


# Load input from file into list of strings (careful not to collapse newline)
def load_input(fd: IO[str]) -> list[str]:
    lines = []
    for line in fd:
        lines.append(line.strip("\n"))
    return lines


# Parse through the input and load into a list of food items that each elf is carrying
def parse_input(items: list[str]) -> dict[ElfId, list[FoodItem]]:
    elf_index = 0
    elves_to_calories = {elf_index: []}
    for line in items:
        if line == "":
            elf_index += 1
            elves_to_calories[elf_index] = []
        else:
            elves_to_calories[ElfId(elf_index)].append(FoodItem(float(line)))
    return elves_to_calories


def get_calories_per_elf(elves_to_calories: dict[ElfId, list[FoodItem]]) -> dict[ElfId, float]:
    return {
        elf_id: sum([food_item.calorie_count for food_item in food_items])
        for elf_id, food_items in elves_to_calories.items()
    }


# Find the argmax of the dictionary
def find_max_calories(elves_to_calories: dict[ElfId, list[FoodItem]]) -> float:
    calories_per_elf = get_calories_per_elf(elves_to_calories)
    return max(calories_per_elf.values())


def find_calories_in_n_top_elves(elves_to_calories: dict[ElfId, list[FoodItem]], n: int) -> float:
    if n > len(elves_to_calories.keys()):
        raise RuntimeError("N is greater than the number of elves!")
    elf_calories = get_calories_per_elf(elves_to_calories).values()
    target_elf_calories = sorted(elf_calories)[-n:]
    return sum(target_elf_calories)


if __name__ == "__main__":
    input_data = load_input(sys.stdin)
    elves_to_calories = parse_input(input_data)
    n = 3
    max_calories = find_calories_in_n_top_elves(elves_to_calories, n)
    print(f"Total Calories in top {n} elves : {max_calories}")
    
