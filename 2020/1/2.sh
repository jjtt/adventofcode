#!/bin/bash

xargs -n1 -I{} bash -c "echo {}; ./1.sh <(grep -v {} $1) \$((2020 - {}))" < $1

