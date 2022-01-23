#!/bin/bash

expr $(grep "(" input.txt -o | wc -l) - $(grep ")" input.txt -o | wc -l)
