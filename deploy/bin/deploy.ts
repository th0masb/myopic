#!/usr/bin/env node
import 'source-map-support/register';
import * as cdk from '@aws-cdk/core';
import { MyopicGameLambdaStack } from '../lib/myopic-game-lambda-stack';

require('dotenv').config()

const app = new cdk.App();
new MyopicGameLambdaStack(app, 'MyopicGameLambdaStack', {
  functionName: process.env.FUNCTION_NAME!,
  timeout: cdk.Duration.minutes(Number.parseInt(process.env.FUNCTION_TIMEOUT_MINS!)),
  env: {
    region: process.env.REGION!,
    account: process.env.ACCOUNT!
  }
});
