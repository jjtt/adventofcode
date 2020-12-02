#!/bin/bash

grep -f <(sort $1 | awk '{print "^" (2020 - $1) "$"}') $1 | paste - - | awk '{print $1 * $2}'
