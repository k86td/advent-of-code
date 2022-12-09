#!/usr/bin/python3
# day 3 of advent of code

from puzzle import Test, Puzzle

def calc_val1(item_array):
  total = 0

  for letter in item_array:
    ascii_val = ord(letter)

    if letter.isupper():
      ascii_val -= 38
    else:
      ascii_val -= 96

    total += ascii_val

  return total

def part1(data):
  data = data.splitlines()

  incorrect_item = []
  for d in data:
    
    bag_len = len(d)

    comp1 = d[0:int(bag_len/2)]
    comp2 = d[int(bag_len/2):bag_len]

    bag1 = {}
    bag2 = {}

    for char in comp1:
      if char in bag1:
        bag1[char] += 1
      else:
        bag1[char] = 1

    for char in comp2:
      if char in bag2:
        bag2[char] += 1
      else:
        bag2[char] = 1

    for key in bag1.keys():
      if key in bag2.keys():
        incorrect_item.append(key)

  return calc_val1(incorrect_item)

def part2(data):
  def get_bag(s):
    bag_len = len(s)
    bag = set()

    for char in s:
      bag.add(char)

    return bag

  data = data.splitlines()

  incorrect_item = []

  for ind, d in enumerate(data):

    if not ind % 3 == 0: continue
    if len(data) - 3 < ind: break

    bag1 = get_bag(d)
    bag2 = get_bag(data[ind+1])
    bag3 = get_bag(data[ind+2])

    bags = [bag1, bag2, bag3]

    common = bag1

    for bag in bags[1:]:
      common.intersection_update(bag)

    incorrect_item.append(list(common)[0])

  return calc_val1(incorrect_item)

if __name__ == "__main__":
  p1 = Test("""vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw""", 157, part1)

  p2 = Test("""vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw""", 70, part2)

  day = Puzzle(3, 2022)
  day.add_test(p1, "a")
  day.add_test(p2, "b")

  day.run_tests()

  
