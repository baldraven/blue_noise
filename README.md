# blue_noise

Point generation on a rectangle.

## Quickstart

The program places points on a box `X*Y` sized. It can either work with the number of point to place `N`, or the minimal distance between points `d`. 
The program take 4 arguments as input :
- int : mode
- float/int : N ou d (selon le mode)
- float : X
- float : Y

Current modes available :
- 1 : as a grid, with N
- 2 : as a grid, with d