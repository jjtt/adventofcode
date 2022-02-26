#!/bin/sh
egrep '\-?[0-9]+' -o input.txt | paste -sd+|bc
