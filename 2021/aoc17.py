"""
Advent of Code day 17: physics simulation! Plus some linear constraint programming.

Luckily I already had some code for this.
"""

#Box constants
UPPER_LEFT = (153, -75)
LOWER_RIGHT = (199, -114)

"""
Initial x velocity must be at least the number whose triangle number is the same is the position
of the box's leftmost face (otherwise it will never reach) and the position of the rightmost face
(otherwise it will overshoot in a single timestep)

In fact, so long as x has a triangle number > LHS and < RHS it doesn't actually matter.

y starting velocity must be as high as possible; however, once its downward velocity exceeds
the height of the box, we're into something trickier.

"""

import matplotlib.pyplot as plt
from matplotlib.patches import Rectangle
import numpy as np

class Probe():

    def __init__(self, v_x, v_y):
        self.v_x = v_x
        self.v_y = v_y
        self.x = 0
        self.y = 0

    def step(self):
        self.x += self.v_x
        self.y += self.v_y
        self.drag()
        self.gravity()
        return (self.x, self.y)

    def drag(self):
        self.v_x -= np.sign(self.v_x)

    def gravity(self):
        self.v_y -= 1

for y in range(37, 4000):
    probe = Probe(19,y)
    trajectory = []
    while probe.v_y >= -10 * y:
        trajectory.append(probe.step())
    if any(p[1] < -75 and p[1] > -114 for p in trajectory):
        print(f"solution {y}")
        print(max(p[1] for p in trajectory))
# _, ax = plt.subplots()
# ax.scatter([p[0] for p in trajectory], [p[1] for p in trajectory])
# ax.add_patch(Rectangle((153, -75), (199 - 153), (-114 + 75)))
# plt.show()
# print(max(p[1] for p in trajectory))
# if any(p[1] < -75 and p[1] > -114 for p in trajectory):
#     print("solution")
# else:
#     print("not solution")
