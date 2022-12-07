#!/bin/bash
find $1 -type f -print0 | du -b --files0-from=- -c --exclude chroot/.busybox --exclude chroot/.busybox/sh | tail -n 1
