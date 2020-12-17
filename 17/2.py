#!/usr/bin/env python

import sys

neighbours = set([(x,y,z) for x in range(-1,2) for y in range(-1,2) for z in range(-1,2)]) - set([(0,0,0)])

def isval(cubes, x, y, z, val):
    c = cubes.get((x,y,z), False)
    return c == val

def newstate(cubes, x, y, z):
    global neighbours
    
    num_active_neighbours = sum([isval(cubes, x+xn, y+yn, z+zn, True) for xn,yn,zn in neighbours])
    if cubes.get((x,y,z), False):
        return True if num_active_neighbours in [2,3] else False
    else:
        return True if num_active_neighbours == 3 else False

def newcube(cubes):
    minx = min([x for x,_,_ in cubes.keys()])
    maxx = max([x for x,_,_ in cubes.keys()])
    miny = min([y for _,y,_ in cubes.keys()])
    maxy = max([y for _,y,_ in cubes.keys()])
    minz = min([z for _,_,z in cubes.keys()])
    maxz = max([z for _,_,z in cubes.keys()])
    newcube = {}
    for x in range(minx-1, maxx+2):
        for y in range(miny-1, maxy+2):
            for z in range(minz-1, maxz+2):
                if newstate(cubes,x,y,z):
                    newcube[(x,y,z)] = True
            
    return newcube

lines = [l.rstrip() for l in sys.stdin.readlines()] 

cubes = {}
z = 0
for y in range(len(lines)):
    for x in range(len(lines[0])):
        if lines[y][x] == '#':
            cubes[(x,y,z)] = True

print(cubes)
for s in range(6):
    cubes = newcube(cubes)
    print(cubes)
print(len(cubes))
