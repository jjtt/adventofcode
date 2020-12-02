#!/bin/bash

grep -f <(awk '{print "^" (2020 - $1) "$"}' $1) $1 | paste - - | awk '{print $1 * $2}'
