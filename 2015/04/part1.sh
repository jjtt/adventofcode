#!/bin/bash
for i in $(seq 1 400000); do echo -n iwrupvqb$i | md5sum | tr -d '\n'; echo num: $i; done | grep "^00000"
