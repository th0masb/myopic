#!/usr/bin/env node
import 'source-map-support/register';
import * as cdk from '@aws-cdk/core';
import { MyopicLambdaStack } from '../lib/myopic-lambda-stack';
import { MyopicDatabaseStack } from "../lib/opening-db-stack";

require('dotenv').config()

const app = new cdk.App();
const envConfig = {
  region: process.env.MYOPIC_AWS_REGION!,
  account: process.env.MYOPIC_AWS_ACCOUNT!
}

new MyopicDatabaseStack(app, 'MyopicDatabaseStack', {
  openingsTableName: process.env.OPENINGS_TABLE_NAME!,
  positionAttributeName: process.env.POSITION_ATTRIBUTE_NAME!,
  readCapacity: Number.parseInt(process.env.READ_CAPACITY!),
  writeCapacity: Number.parseInt(process.env.WRITE_CAPACITY!),
  env: envConfig
});

new MyopicLambdaStack(app, 'MyopicLambdaStack', {
  openingsTableName: process.env.OPENINGS_TABLE_NAME!,
  env: envConfig,
  gameLambdaConfig: {
    assetName: process.env.GAME_HANDLER_ASSET_NAME!,
    functionName: process.env.GAME_HANDLER_FUNCTION_NAME!,
    timeout: cdk.Duration.minutes(Number.parseInt(process.env.GAME_HANDLER_FUNCTION_TIMEOUT_MINS!)),
    memoryLimit: Number.parseInt(process.env.GAME_HANDLER_MEMORY_SIZE!),
  },
  benchLambdaConfig: {
    assetName: process.env.BENCHMARK_ASSET_NAME!,
    functionName: process.env.BENCHMARK_FUNCTION_NAME!,
    timeout: cdk.Duration.minutes(Number.parseInt(process.env.BENCHMARK_FUNCTION_TIMEOUT_MINS!)),
    memoryLimit: Number.parseInt(process.env.BENCHMARK_MEMORY_SIZE!),
  },
  moveLambdaConfig: {
    assetName: process.env.MOVE_ASSET_NAME!,
    functionName: process.env.MOVE_FUNCTION_NAME!,
    timeout: cdk.Duration.minutes(Number.parseInt(process.env.MOVE_FUNCTION_TIMEOUT_MINS!)),
    memoryLimit: Number.parseInt(process.env.MOVE_MEMORY_SIZE!),
  }
});

