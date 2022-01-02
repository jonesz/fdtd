#!/usr/bin/python
# Produce a plot of EZ values within a 2D simulation.
import json
import sys
import matplotlib.pyplot as plt
import numpy as np

f = open(sys.argv[1])
deserialized = json.load(f)

X = np.linspace(0, deserialized['x_sz'], deserialized['x_sz'])
Y = np.linspace(0, deserialized['y_sz'], deserialized['y_sz'])
X, Y = np.meshgrid(X, Y)

Z = np.array(deserialized['ez'])
# Convert from 1D to 2D.
# TODO: Shouldn't 'y_sz' and 'x_sz' be flipped?
Z = Z.reshape(deserialized['y_sz'], deserialized['x_sz'])

fig, ax = plt.subplots()
ax.pcolormesh(X, Y, Z, vmin=-3.0, vmax=0)
plt.show()
