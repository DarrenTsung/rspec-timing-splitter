#!/bin/bash

mkdir -p output

docker build -t rspec-timing-splitter .
docker run --rm -v ${PWD}/output:/code/output rspec-timing-splitter bash -c "mv target/release/rspec-timing-tool output"

echo ""
echo "Built binary at output/rspec-timing-splitter!"
