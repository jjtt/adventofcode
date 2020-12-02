#!/bin/bash

sort $1 | awk '{print 2020 - $1}' | xargs -n 1 -I{} grep '^{}$' $1 | paste - - | awk '{print $1 * $2}'
