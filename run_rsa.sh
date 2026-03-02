#!/bin/bash
# Clean the input: remove newlines and extra spaces
CLEAN_MSG=$(echo -n "$1" | tr -d "\n\r" | xargs)
cd /data/data/com.termux/files/home
./rsa_gen "$CLEAN_MSG"