"""
Advent of code day 15: playing a memory game with elves.
"""
class SpeakingGame:
    """
    Class to hold the seed numbers and game history. Dict-based.
    """
    def __init__(self, target, seed):
        self.target = target
        self.seed = seed
        self.spoken_numbers = {}

    def run(self) -> int:
        """
        Runs the game.
        """
        for i in range(1, self.target+1):
            if self.seed != []:
                last = self.seed[0]
                self.seed = self.seed[1:]
            else:
                if isinstance(self.spoken_numbers[last], tuple):
                    last = self.spoken_numbers[last][0] - self.spoken_numbers[last][1]
                else:
                    last = 0
            self.update(last, i)
        return last

    def update(self, last, i):
        if last in self.spoken_numbers:
            if isinstance(self.spoken_numbers[last], tuple):
                self.spoken_numbers[last] = (i, self.spoken_numbers[last][0])
            else:
                self.spoken_numbers[last] = (i, self.spoken_numbers[last])
        else:
            self.spoken_numbers[last] = i

#setup
seed = [7,14,0,17,11,1,2]

def part_1():
    s = SpeakingGame(2020, seed)
    return s.run()

def part_2():
    s = SpeakingGame(30000000, seed)
    return s.run()

if __name__ == "__main__":
    print(part_1())
    print(part_2())
