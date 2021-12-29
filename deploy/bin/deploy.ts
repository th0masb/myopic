#!/usr/bin/env node
import "source-map-support/register";
import * as cdk from "@aws-cdk/core";
import { MyopicDatabaseStack } from "../lib/opening-db-stack";
import { BotStack, LambdaConfigType } from "../lib/bot-stack";

require("dotenv").config();

const app = new cdk.App();
const envConfig = {
  region: process.env.MYOPIC_AWS_REGION!,
  account: process.env.MYOPIC_AWS_ACCOUNT!,
};

new MyopicDatabaseStack(app, "MyopicDatabaseStack", {
  openingsTableName: process.env.OPENINGS_TABLE_NAME!,
  positionAttributeName: process.env.POSITION_ATTRIBUTE_NAME!,
  readCapacity: Number.parseInt(process.env.READ_CAPACITY!),
  writeCapacity: Number.parseInt(process.env.WRITE_CAPACITY!),
  env: envConfig,
});

new BotStack(app, "Myopic", {
  env: envConfig,
  openingTableName: process.env.OPENINGS_TABLE_NAME!,
  params: new Map([
    [ LambdaConfigType.Move, { memory: 1792, timeout: cdk.Duration.minutes(10) } ],
    [ LambdaConfigType.Game, { memory: 128, timeout: cdk.Duration.minutes(10) } ],
  ]),
});

new BotStack(app, "Hyperopic", {
  env: envConfig,
  openingTableName: process.env.OPENINGS_TABLE_NAME!,
  params: new Map([
    [ LambdaConfigType.Move, { memory: 1792, timeout: cdk.Duration.minutes(10) } ],
    [ LambdaConfigType.Game, { memory: 128, timeout: cdk.Duration.minutes(10) } ],
  ]),
});
