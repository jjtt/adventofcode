#!/bin/bash

egrep ^$(cat $1 patch.txt | ./2.py)\$ $1 | wc
