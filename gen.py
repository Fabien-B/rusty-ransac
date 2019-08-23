#!/usr/bin/python3
from sklearn.datasets import make_regression
from matplotlib import pyplot
import sys

def generate_regression(n_samples=100, noise=10):
    # generate regression dataset
    X, Y = make_regression(n_samples=n_samples, n_features=1, noise=noise, bias = 0.0)
    Y=Y**2
    # plot regression dataset
    pyplot.scatter(X,Y)
    pyplot.show()
    
    with open("test.csv", "w") as f:
        f.write("x,y\n")
        for ax, y in zip(X,Y):
            x=ax[0]
            #print(x,y)
            f.write("{},{}\n".format(x,y))

if __name__ == '__main__':
    n = int(sys.argv[1])
    noise = int(sys.argv[2])
    generate_regression(n, noise)
