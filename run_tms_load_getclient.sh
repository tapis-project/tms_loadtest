#!/bin/bash
# Very simple script to run getclient load test that causes the server to hang.
# Before first use, set TMS_URL, e.g. TMS_URL=http://localhost:3001 or TMS_URL=https://tms-server-dev.tacc.utexas.edu:3000
#  and build using
#     cargo build --release

# ./target/release/tms_loadtest --host $TMS_URL --users 20 --iterations 10000 --scenarios getversion --report-file ~/tms-loadtest_local_getversion.html

#./target/release/tms_loadtest --users 10 --iterations 20000 --scenarios getclient --host $TMS_URL --report-file ~/tms-loadtest_local_getclient.html

# 10 users
#./target/release/tms_loadtest --users 10 --iterations 20000 --scenarios getclient --host $TMS_URL \
#  	--report-file ~/tms-loadtest_local_getclient_perf-debug$1.html
set -xv
./target/release/tms_loadtest --users 20 --iterations 20000 --scenarios getclient --host $TMS_URL \
  	--report-file ~/tms-loadtest_local_getclient_perf-debug$1.html
