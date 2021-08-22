#!/bin/bash

set -ev

. ./bootstrap.sh &&\


./build.sh &&\

basic-http-server ./ -a 0.0.0.0:1337
