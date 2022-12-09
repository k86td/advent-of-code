#!/usr/bin/python3
# day 2 of advent of code

"""
Opponent play:
  A: Rock
  B: Paper
  C: Scissor

Me play:
  X: Rock
  Y: Paper
  Z: Scissor
"""

from puzzle import Test, Puzzle

MOVES = {
  "A": (1, 2),
  "B": (2, 3),
  "C": (3, 1),
  # my moves
  "X": 1,
  "Y": 2,
  "Z": 3
}

def parse_round1(opponent_move, my_move):
  # calc points of both player and see who wins

  opponent_val, target_win_move = MOVES[opponent_move]
  my_val = MOVES[my_move]

  if opponent_val == my_val:  # draw
    return 3 + my_val
  elif my_val == target_win_move: # win
    return 6 + my_val
  else: # lose
    return 0 + my_val

COMPLEX_MOVE = {
  "A": {
    6: 2,
    3: 1,
    0: 3
  },
  "B": {
    6: 3,
    3: 2,
    0: 1
  },
  "C": {
    6: 1,
    3: 3,
    0: 2
  },
}

OUTCOME = {
  "X": 0, # lose
  "Y": 3, # draw
  "Z": 6  # win
}

def parse_round2(opponent_move, target_outcome):

  opponent_val, target_win_move = MOVES[opponent_move]
  outcome = OUTCOME[target_outcome]

  my_play = COMPLEX_MOVE[opponent_move][outcome]

  return outcome + my_play

def part1(data):
  data = data.splitlines()

  win_sum = 0
  for d in data:
    he, me = d.split(' ')
    win_sum = win_sum + parse_round1(he, me)

  return win_sum

def part2(data):
  data = data.splitlines()

  win_sum = 0
  for d in data:
    he, target = d.split(' ')
    win_sum = win_sum + parse_round2(he, target)

  return win_sum

if __name__ == "__main__":
  p1 = Test("""A Y
B X
C Z""", 15, part1)

  p2 = Test("""A Y
B X
C Z""", 12, part2)

  day = Puzzle(2, 2022)
  day.add_test(p1, "a")
  day.add_test(p2, "b")

  day.run_tests()


