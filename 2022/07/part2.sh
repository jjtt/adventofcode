#!/bin/bash
./preprocess.sed input.txt | cut -c3- | sudo chroot chroot/ .busybox/sh
sudo chown -R 1000:1000 chroot
TOTAL=$(./du.sh chroot | sort -n | head -n 29 | cut -f1)
REMAINING=$(echo 30000000-70000000+$TOTAL | bc)

echo We need: $REMAINING
echo Root: $TOTAL

find chroot/* -type d -exec ./du.sh {} \; | sort -n | cut -f1 | tail -n7 | head -n1



echo "that ^ should be 5469168"
