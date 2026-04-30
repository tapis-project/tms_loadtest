#!/bin/bash
# Very simple script to run getkey load test
# Before first use, set TMS_URL, e.g. TMS_URL=http://localhost:3001 or TMS_URL=https://tms-server-dev.tacc.utexas.edu:3000
#  and build using
#     cargo build --release

#
#set -xv
#./target/release/tms_loadtest --users 1 --iterations 1 --scenarios getkey --host $TMS_URL \
#  	--report-file ~/tms-loadtest_local_getkey_perf-debug$1.html
set -xv
./target/release/tms_loadtest --users 20 --iterations 10000 --scenarios getkey --host $TMS_URL \
  	--report-file ~/tms-loadtest_local_getkey_perf-debug$1.html
