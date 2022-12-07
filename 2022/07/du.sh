#!/bin/bash
find $1 -type f -print0 | du -b --files0-from=- -c | tail -n 1
