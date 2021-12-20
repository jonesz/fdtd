#!/usr/bin/python
# Produce a waterfall plot of EZ values within the 1D simulation.
import json
import os
import sys
import matplotlib.pyplot as plt

dir_name = sys.argv[1]
names = os.listdir(dir_name)

plt.title("Waterfall plot of: {}".format(sys.argv[1]))
plt.xlabel("Space [spatial index]")
plt.ylabel("Time [snapshot number]")

for (i, name) in enumerate(names):
    f = open("{}/{}".format(dir_name, name))
    deserialized = json.load(f)
    ez = deserialized['ez']
    plt.plot([x for x in range(len(ez))], [x + i for x in ez])

plt.show()
