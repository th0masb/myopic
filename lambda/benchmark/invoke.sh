#!/usr/bin/env bash

set -e -u -o pipefail

aws lambda invoke \
  --function-name "arn:aws:lambda:eu-west-2:918538493915:function:$1-Benchmark" \
  --invocation-type 'Event' --log-type 'Tail' \
  --payload file://lambda/benchmark/payload.json \
  --cli-binary-format raw-in-base64-out \
  --region 'eu-west-2' /tmp/out.json
