
from aocd import get_data, submit

class bcolors:
    HEADER = '\033[95m'
    OKBLUE = '\033[94m'
    OKCYAN = '\033[96m'
    OKGREEN = '\033[92m'
    WARNING = '\033[93m'
    FAIL = '\033[91m'
    ENDC = '\033[0m'
    BOLD = '\033[1m'
    UNDERLINE = '\033[4m'

class Test:
  def __init__(self, input, output, solver):
    self.input = input
    self.output = output
    self.solver = solver

  def exec(self):
    result = self.solver(self.input)
    if result == self.output:
      return (true, result)
    return (false, result)

class Puzzle:
  def __init__(self, day, year):
    self._data = get_data(day=day, year=year)
    self.tests = []

  def add_test(self, test, part):
    self.tests.append((test, part))

  def run_tests(self):
    test_number = 0
    for test in self.tests:
      test_number = test_number + 1
      if test.exec():
        print(bcolors.UNDERLINE + bcolors.OKGREEN + "The test was a success!" + bcolors.ENDC)





