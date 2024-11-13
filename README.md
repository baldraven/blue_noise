# blue_noise

Point generation on a rectangle.

## Quickstart

The program places points on a box `X*Y` sized. It can either work with the number of point to place `N`, or the minimal distance between points `d`. 
The program take 4 arguments as input :
- int : mode
- float : n_or_d
- float : X
- float : Y

Current modes available :
- 1 : as a grid, with N
- 2 : as a grid, with d
- 3 : with fast Poisson disk sampling algorithm, with d

It offers HTML visualization of the generated points and the associated Jump Flooding Voronoi diagram, and output in points.csv.
