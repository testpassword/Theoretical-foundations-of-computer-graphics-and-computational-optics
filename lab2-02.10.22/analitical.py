from math import hypot, pi
from numbers import Number
from re import A
from typing import Iterable
import numpy as np

# Calculate distance between point and light source.
def R(point: Iterable[Number], light: Iterable[Number]) -> float:
    return hypot(light[0]-point[0], light[1]-point[1], light[2]-point[2])

# Calculate vector between point and light source.
def vec(point: Iterable[Number], light: Iterable[Number]) -> np.array:
    return np.array(light) - np.array(point)

# Numerical value of cos between vector(between point and light source) and normal vector.
def cos(k: Iterable[Number], normal_vec: np.array) -> float:
    return (k[0] * normal_vec[0] + k[1] * normal_vec[1] + k[2] * normal_vec[2]) / (hypot(*k) * hypot(*normal_vec))

# Find illumination in point.
def E(W: Number, R: Number, cos: Number) -> float: 
    return ((W / (4 * pi)) / R**2) * cos

# Find illumination calculation fault
def ΔE(lumicept_val, analytical_val) -> float:
    return abs((lumicept_val - analytical_val) / analytical_val * 100)

W = 100 # light flow power
vecN = np.array([0, -1, 0]) # normal vector of plane
light = [1, -2, 1.5] # light source coordinates
p1 = [0, 0, 0] # point1
p2 = [-1, 0, 1] # point2
pmin = [-1, 0, -1.5] # Emin
pmax = [1, 0, 1.5] # Emax
for it in [(p1, 0.8114), (p2, 0.69148), (pmin, 0.2342160), (pmax, 1.992036)]:
    Ep_lumicept = it[1]
    Ep_analytical = E(W, R(it[0], light), cos(vec(it[0], light), vecN)) # Illuminace in point analytical
    print(f'{Ep_analytical=} ; {Ep_lumicept=} ; ΔE = {ΔE(it[1], Ep_analytical)}')
    print('------------------------------')
