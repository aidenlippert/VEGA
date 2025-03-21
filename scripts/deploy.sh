#!/bin/bash
cd ~/vega
docker build -t vega-node .
docker run -d -p 4001:4001 vega-node
