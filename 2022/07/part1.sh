#!/bin/bash
./preprocess.sed input.txt | cut -c3- | sudo chroot chroot/ .busybox/sh
sudo chown -R 1000:1000 chroot
find chroot/* -type d -exec ./du.sh {} \; | sort -n | head -n 29 | cut -f1 | paste -s -d+ | bc

echo "that ^ should be 1391690"
