#!/usr/bin/env bash

this_dir="$(realpath "$(dirname "$0")")"

aws lambda invoke \
  --function-name 'arn:aws:lambda:eu-west-2:918538493915:function:HyperopicChallenger' \
  --invocation-type 'Event' \
  --log-type 'Tail' \
  --payload "file://$this_dir/payload.json" \
  --cli-binary-format raw-in-base64-out \
  --region 'eu-west-2' /tmp/out.json