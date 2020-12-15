class SpeakingGame:
    def __init__(self, target, seed):
        self.target = target
        self.seed = seed
        self.spoken_numbers = {}

    def run(self):
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

seed = [7,14,0,17,11,1,2]
s = SpeakingGame(2020, seed)
print(s.run())
s = SpeakingGame(30000000, seed)
print(s.run())
