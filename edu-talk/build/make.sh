#!/usr/bin/env bash


echo "Generating new index.html from body.md"
cat boiler.txt > ../index.html
bundle exec redcarpet ../essay.md >> ../index.html
