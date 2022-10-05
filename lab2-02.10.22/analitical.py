from math import hypot, pi
from numbers import Number
from re import A
from typing import Iterable
import numpy as np

# Calculate distance between point and light source.
def R(point: Iterable[Number], light: Iterable[Number]) -> float:
    r = hypot(light[0]-point[0], light[1]-point[1], light[2]-point[2])
    print(f'расстояние между точкой и источником {r}')
    return r

# Calculate vector between point and light source.
def vec(point: Iterable[Number], light: Iterable[Number]) -> np.array:
    k = np.array(light) - np.array(point)
    print(f'вектор точка-источник {k}')
    return k

# Numerical value of cos between vector(between point and light source) and normal vector.
def cos(k: Iterable[Number], normal_vec: np.array) -> float:
    c = (k[0] * normal_vec[0] + k[1] * normal_vec[1] + k[2] * normal_vec[2]) / (hypot(*k) * hypot(*normal_vec))
    print(f'cos {c}')
    return c

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
for it in [(p1, 0.8114), (p2, 0.69148)]:
    Ep_lumicept = it[1]
    Ep_analytical = E(W, R(it[0], light), cos(vec(it[0], light), vecN)) # Illuminace in point analytical
    print(f'{Ep_analytical=} ; {Ep_lumicept=} ; ΔE = {ΔE(it[1], Ep_analytical)}')
    print('------------------------------')

#Emax