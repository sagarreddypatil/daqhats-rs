#!/bin/bash

docker build -t daqhats-env .
docker run -it -v $PWD:/work -w /work daqhats-env bash
