#!/bin/bash

egrep ^$(cat $1 | ./1.py)\$ $1 | wc
