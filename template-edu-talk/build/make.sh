#!/usr/bin/env bash


echo "Generating new index.html from body.md"
cat boiler.txt > ../index.html
redcarpet ../essay.md >> ../index.html
