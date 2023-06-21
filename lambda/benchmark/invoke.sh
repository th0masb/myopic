#!/usr/bin/env bash

aws lambda invoke \
  --function-name 'arn:aws:lambda:eu-west-2:918538493915:function:Myopic-Benchmark' \
  --invocation-type 'Event' --log-type 'Tail' \
  --payload file://lambda/benchmark/payload.json \
  --cli-binary-format raw-in-base64-out \
  --region 'eu-west-2' /tmp/out.json
