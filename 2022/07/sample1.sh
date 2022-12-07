#!/bin/bash
./preprocess.sed sample.txt | cut -c3- | sudo chroot chroot/ .busybox/sh
sudo chown -R 1000:1000 chroot
