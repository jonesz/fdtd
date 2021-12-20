#!/usr/bin/python
# Produce a scatter plot of EZ values within the 1D simulation.
import json
import sys
import matplotlib.pyplot as plt

f = open(sys.argv[1])
deserialized = json.load(f)
ez = deserialized['ez']

# TODO: Axes limits.

fig, ax = plt.subplots()
plt.title(sys.argv[1])
plt.xlabel('Spatial Step')
plt.ylabel('Ez (V/m)')

ax.plot([x for x in range(len(ez))], ez)
ax.set_ylim(0, 1)
plt.show()
