#!/bin/bash

mkdir -p output

# docker build -t rspec-timing-splitter .
# docker run --rm -v ${PWD}/output:/code/output rspec-timing-splitter "mv /code/target/release/rspec-timing-tool /code/output/rspec-timing-tool"
docker run --rm -v ${PWD}/output:/code/output rspec-timing-splitter bash -c "mv target/release/rspec-timing-tool output"
