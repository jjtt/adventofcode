#!/bin/bash

sed 's/^\(.*\)-\(.*\) \(.\): \(.*\)$/[[ $(echo -n ö; (echo -n \4 | tr -c -d \3); echo -n ö) =~ [^\3]\3{\1,\2}[^\3] ]] \&\& echo ok/' $1 | xargs -I{} bash -c '{}' | wc
