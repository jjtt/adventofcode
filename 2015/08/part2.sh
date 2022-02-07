#!/bin/bash

echo "sample1"
string=$(tr -d '\n' < sample1.txt |wc -c)
encoded=$(sed -E 's/\\|"/AA/g' sample1.txt | sed -E 's/$/BB/g' | tr -d '\n' | wc -c)
let "answer = $encoded - $string"
echo $answer


echo "input"
string=$(tr -d '\n' < input.txt |wc -c)
encoded=$(sed -E 's/\\|"/AA/g' input.txt | sed -E 's/$/BB/g' | tr -d '\n' | wc -c)
let "answer = $encoded - $string"
echo $answer

