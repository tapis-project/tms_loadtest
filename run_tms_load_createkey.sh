#!/bin/bash
# Very simple script to run createkey load test
# Before first use, set TMS_URL, e.g. TMS_URL=http://localhost:3001 or TMS_URL=https://tms-server-dev.tacc.utexas.edu:3000
#  and build using
#     cargo build --release

#
set -xv
./target/release/tms_loadtest --users 10 --iterations 20000 --scenarios createkey \
  	--host $TMS_URL --report-file ~/tms-loadtest_local_createkey_perf-debug$1.html
