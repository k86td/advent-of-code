#!/usr/bin/python3
# day 10 of advent of code

from puzzle import Test, Puzzle


def part1(data):
  data = data.splitlines()



  return 15

def part2(data):
  return 20

if __name__ == "__main__":
  p1 = Test("""[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]""", 26397, part1)


  day = Puzzle(10, 2021)
  day.add_test(p1, "a")

  day.run_tests()



