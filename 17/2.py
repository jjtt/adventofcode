#!/usr/bin/env python

import sys

neighbours = set([(x,y,z,w) for x in range(-1,2) for y in range(-1,2) for z in range(-1,2) for w in range(-1,2)]) - set([(0,0,0,0)])

def isval(cubes, x, y, z, w, val):
    c = cubes.get((x,y,z,w), False)
    return c == val

def newstate(cubes, x, y, z, w):
    global neighbours
    
    num_active_neighbours = sum([isval(cubes, x+xn, y+yn, z+zn, w+wn, True) for xn,yn,zn,wn in neighbours])
    if cubes.get((x,y,z,w), False):
        return True if num_active_neighbours in [2,3] else False
    else:
        return True if num_active_neighbours == 3 else False

def newcube(cubes):
    minx = min([x for x,_,_,_ in cubes.keys()])
    maxx = max([x for x,_,_,_ in cubes.keys()])
    miny = min([y for _,y,_,_ in cubes.keys()])
    maxy = max([y for _,y,_,_ in cubes.keys()])
    minz = min([z for _,_,z,_ in cubes.keys()])
    maxz = max([z for _,_,z,_ in cubes.keys()])
    minw = min([w for _,_,_,w in cubes.keys()])
    maxw = max([w for _,_,_,w in cubes.keys()])
    newcube = {}
    for x in range(minx-1, maxx+2):
        for y in range(miny-1, maxy+2):
            for z in range(minz-1, maxz+2):
                for w in range(minw-1, maxw+2):
                    if newstate(cubes,x,y,z,w):
                        newcube[(x,y,z,w)] = True
            
    return newcube

lines = [l.rstrip() for l in sys.stdin.readlines()] 

cubes = {}
z = 0
w = 0
for y in range(len(lines)):
    for x in range(len(lines[0])):
        if lines[y][x] == '#':
            cubes[(x,y,z,w)] = True

for s in range(6):
    cubes = newcube(cubes)
print(len(cubes))
