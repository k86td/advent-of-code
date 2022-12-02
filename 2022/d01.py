#!/usr/bin/python3
# day 1 of advent of code

from puzzle import Test, Puzzle


def part1(data):

  data = data.split('\n\n')
  calories = None

  elf_num = 0
  biggest_cal = 0
  for d in data:
    elf_num = elf_num + 1

    # loop over every elf calories
    calories = d.split('\n')
    
    c_sum = 0
    for calorie in calories:
      c_sum = c_sum + int(calorie)

    if c_sum > biggest_cal:
      biggest_cal = c_sum

  print("Biggest calorie is: " + str(biggest_cal)) 

  return biggest_cal

def part2(data):

  data = data.split('\n\n')
  calories = None

  elf_num = 0
  biggest_cal = [0]
  for d in data:
    elf_num = elf_num + 1

    # loop over every elf calories
    calories = d.split('\n')
    
    c_sum = 0
    for calorie in calories:
      c_sum = c_sum + int(calorie)

    biggest_cal.append(c_sum)

  biggest_cal.sort()

  t_cal = 0
  for x in range(1, 4):
    t_cal = t_cal + biggest_cal[-x]
  
  print("Total Calories for the three elves is: " + str(t_cal))

  return t_cal

if __name__ == "__main__":
  p1 = Test("""1000
2000
3000

4000

5000
6000

7000
8000
9000

10000""", 24000, part1)

  p2 = Test("""1000
2000
3000

4000

5000
6000

7000
8000
9000

10000""", 45000, part2)

  day = Puzzle(1, 2022)
  day.add_test(p1, "a")
  day.add_test(p2, "b")

  day.run_tests()



