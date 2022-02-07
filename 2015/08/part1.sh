#!/bin/bash

echo "sample1"
string=$(tr -d '\n' < sample1.txt |wc -c)
memory=$(sed -E 's/\\(\\|"|x..)/A/g' sample1.txt | sed -E 's/^"|"$//g' | tr -d '\n' | wc -c)
let "answer = $string - $memory"
echo $answer


echo "input"
string=$(tr -d '\n' < input.txt |wc -c)
memory=$(sed -E 's/\\(\\|"|x..)/A/g' input.txt | sed -E 's/^"|"$//g' | tr -d '\n' | wc -c)
let "answer = $string - $memory"
echo $answer
