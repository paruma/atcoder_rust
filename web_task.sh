#!/bin/sh

if [ $# -ne 2 ]; then
    echo "usage: ./web_task.sh [contest_name] [task_name]" 1>&2
    exit 1
fi
open "https://atcoder.jp/contests/$1/tasks/$1_$2"