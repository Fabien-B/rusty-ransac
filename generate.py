#!/usr/bin/python3
import random

def add_noise(x, sig):
    noise = sig*(random.random() - 0.5)
    if random.random() < 0.2:
        noise = 20*sig*(random.random() - 0.5)
    return x + noise

def get_y(x, factors):
    y = 0
    for i, factor in enumerate(factors):
        y += factor * x**i
    return y

def write_csv(filename):
    x = 0.0
    with open(filename, 'w') as f:
        f.write("x,y\n")
        while x < 10:
            line = "{},{}\n".format(x, add_noise(get_y(x, [3, 2]), 3))
            f.write(line)
            #print(line)
            x+=0.5

if __name__ == "__main__":
    write_csv("plop.csv")