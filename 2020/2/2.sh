#!/bin/bash

sed 's/^\(.*\)-\(.*\) \(.\): \(.*\)$/pw=\\" \4\\"; [[ ( ${pw:\1:1} =~ \3 || ${pw:\2:1} =~ \3 ) \&\& ! ( ${pw:\1:1} =~ \3 \&\& ${pw:\2:1} =~ \3 ) ]] \&\& echo ok/' $1 | xargs -I{} bash -c '{}' | wc
