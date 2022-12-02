from aoc_2022.problem_1 import load_input, parse_input, FoodItem, find_max_calories

input_data = load_input(open("aoc_2022/input_1.txt"))
elves = parse_input(input_data[0:5])
should_get = {
    0: [FoodItem(calorie_count=17034.0)],
    1: [
        FoodItem(calorie_count=13495.0),
        FoodItem(calorie_count=7368.0),
        FoodItem(calorie_count=13905.0),
    ],
}
assert (elves == should_get)
assert (find_max_calories(elves) == (13495 + 7368 + 13905))
