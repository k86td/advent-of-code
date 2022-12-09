
from aocd import get_data, submit

class bcolors:
    HEADER = '\033[95m'
    OKBLUE = '\033[94m'
    OKCYAN = '\033[96m'
    OKGREEN = '\033[92m'
    WARNING = '\033[93m'
    DARKGREY = '\033[90m'
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
      return (True, result)
    return (False, result)

class Puzzle:
  def __init__(self, day, year):
    self.day = day
    self.year = year
    self._data = get_data(day=day, year=year)
    self.tests = {"a": [], "b": []}

  def add_test(self, test, part):
    self.tests[part].append((test, part))

  def run_tests(self):


    test_number = 0
    invalid_test = False
    for part in "ab":
      all_test_success = False

      if len(self.tests[part]) == 0:
        continue

      for test in self.tests[part]:
        test_number = test_number + 1

        print(bcolors.DARKGREY + "Running test #{}... ".format(test_number) + bcolors.ENDC, end="")

        if invalid_test:
          print(bcolors.DARKGREY + "skipping".format(test_number) + bcolors.ENDC)
          break

        test_result = test[0].exec()
        if test_result[0]:
          print(bcolors.UNDERLINE + bcolors.OKGREEN + "success!" + bcolors.ENDC)
        else:
          invalid_test = True
          print(bcolors.UNDERLINE + bcolors.FAIL + "failed!" + bcolors.ENDC)
          print(bcolors.WARNING + "┃\tWe got:\t\t{}[{}]\n┃\tShould be:\t{}[{}]\n┻".format(test_result[1], type(test_result[1]).__name__, test[0].output, type(test[0].output).__name__) + bcolors.ENDC)

      if not invalid_test:
        print(bcolors.DARKGREY + " ▶ trying to send part " + test[1], end="... " + bcolors.ENDC)
        answer = test[0].solver(self._data)
        response = submit(answer, day=self.day, year=self.year, part=test[1], quiet=True)

        if response == "good answer":
          print(bcolors.UNDERLINE + bcolors.OKGREEN + "done" + bcolors.ENDC)
        elif response == "bad answer":
          print(bcolors.UNDERLINE + bcolors.FAIL + "fail" + bcolors.ENDC)
          print(bcolors.WARNING + f"    we sent: {answer}" + bcolors.ENDC)
        elif "solved" in response:
          print(bcolors.UNDERLINE + bcolors.OKCYAN + "already solved" + bcolors.ENDC)







