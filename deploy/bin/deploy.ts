#!/usr/bin/env node
import 'source-map-support/register';
import * as cdk from '@aws-cdk/core';
import { MyopicGameLambdaStack } from '../lib/myopic-game-lambda-stack';
import { MyopicDatabaseStack } from "../lib/opening-db-stack";

require('dotenv').config()

const app = new cdk.App();

new MyopicDatabaseStack(app, 'MyopicDatabaseStack', {
  openingsTableName: process.env.OPENINGS_TABLE_NAME!,
  positionAttributeName: process.env.POSITION_ATTRIBUTE_NAME!,
  readCapacity: Number.parseInt(process.env.READ_CAPACITY!),
  writeCapacity: Number.parseInt(process.env.WRITE_CAPACITY!),
  env: {
    region: process.env.REGION!,
    account: process.env.ACCOUNT!,
  }
});

new MyopicGameLambdaStack(app, 'MyopicGameLambdaStack', {
  assetName: process.env.MYOPIC_ASSET_NAME!,
  functionName: process.env.MYOPIC_FUNCTION_NAME!,
  openingsTableName: process.env.OPENINGS_TABLE_NAME!,
  timeout: cdk.Duration.minutes(Number.parseInt(process.env.MYOPIC_FUNCTION_TIMEOUT_MINS!)),
  memorySize: Number.parseInt(process.env.MYOPIC_MEMORY_SIZE!),
  region: process.env.REGION!,
  account: process.env.ACCOUNT!,
  env: {
    region: process.env.REGION!,
    account: process.env.ACCOUNT!
  }
});
