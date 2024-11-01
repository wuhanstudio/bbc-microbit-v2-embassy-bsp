# Imports go at the top
from microbit import *
import math

# Code in a 'while True:' loop repeats forever
while True:
    compass.heading()
    mx = compass.get_x()
    my = compass.get_y()
    print(math.atan2(my, mx) * 180 / math.pi + 180)
    sleep(10)
