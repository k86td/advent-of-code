#!/usr/bin/python3
# day 4 of advent of code

from puzzle import Test, Puzzle

def is_contained (a, b):
  if int(a[0]) >= int(b[0]) and int(a[1]) <= int(b[1]):
    return True
  return False

def full_is_contained(a,b):
  return is_contained(a,b) or is_contained(b,a)

def part1(data):
  data = data.splitlines()

  total = 0
  for pair in data:
    e1, e2 = pair.split(',')

    e1 = e1.split('-')
    e2 = e2.split('-')

    e1 = [int(x) for x in e1]
    e2 = [int(x) for x in e2]


    if is_contained(e1, e2) or is_contained(e2, e1):
      total += 1

  return total

def overlaps (a, b):
  if (a[0] >= b[0] and a[0] <= b[1]) or (a[1] >= b[0] and a[1] <= b[1]): return True
  return False

def part2(data):
  data = data.splitlines()

  total = 0
  for pair in data:
    e1, e2 = pair.split(',')

    e1 = e1.split('-')
    e2 = e2.split('-')

    e1 = [int(x) for x in e1]
    e2 = [int(x) for x in e2]

    if overlaps(e1, e2) or full_is_contained(e1,e2):
      total += 1
  return total


if __name__ == "__main__":
  str_t = """2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"""
  p1 = Test(str_t, 2, part1)

  p2 = Test(str_t, 4, part2)

  day = Puzzle(4, 2022)
  day.add_test(p1, "a")
  day.add_test(p2, "b")

  day.run_tests()


