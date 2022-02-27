#!/bin/sh
echo part1
egrep '\-?[0-9]+' -o input.txt | paste -sd+|bc
jq '..|select(type=="number")' input.txt| paste -sd+ | bc
jq '..|select(type=="number")' input.txt| jq -s add

echo part2
jq 'walk(if type=="object" and (to_entries[]|.value!="red") then empty else . end)|..|select(type=="number")' input.txt|paste -sd+ | bc
echo 22912 is too low


jq . input.txt | grep -o '".*": "red"'| sort -u
jq . input.txt | grep -o '".*": "red"'| sort -u | sed 's/"\(.\)": "red"/.\1=="red"/' | paste -sd_ | sed 's/_/ or /g'



<input.txt jq 'walk(if type=="object" and (.a=="red" or .b=="red" or .c=="red" or .d=="red" or .e=="red" or .f=="red" or .g=="red" or .h=="red" or .i=="red" or .j=="red") then 0 else . end) | .. | numbers' | jq -s add
