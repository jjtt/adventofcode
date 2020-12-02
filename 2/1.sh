#!/bin/bash

sed 's/^\(.*\)-\(.*\) \(.\): \(.*\)$/[[ $(echo -n \4 | tr -c -d \3) =~ \3{\1,\2} ]] \&\& echo ok/' $1 | xargs -I{} bash -c '{}' | wc
