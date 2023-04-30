#!/usr/bin/env node
import "source-map-support/register";
import { App, Duration } from 'aws-cdk-lib';
import { OpeningDatabase } from "../lib/opening-db";
import {Bot, OpeningTableConfig} from "../lib/bot";
import * as process from "process";
import {GameLambda} from "../lib/game-lambda";

require("dotenv").config();

const app = new App();
const envConfig = {
  region: process.env.MYOPIC_AWS_REGION!,
  account: process.env.MYOPIC_AWS_ACCOUNT!,
};

const tableConfig: OpeningTableConfig = {
  name: process.env.OPENINGS_TABLE_NAME!,
  region: envConfig.region,
  positionKey: process.env.POSITION_ATTRIBUTE_NAME!,
  moveKey: "Moves",
  maxDepth: 10,
}

new OpeningDatabase(app, "MyopicDatabaseStack", {
  openingsTableName: tableConfig.name,
  positionAttributeName: tableConfig.positionKey,
  readCapacity: Number.parseInt(process.env.READ_CAPACITY!),
  writeCapacity: Number.parseInt(process.env.WRITE_CAPACITY!),
  env: envConfig,
});

const bots = ["Myopic", "Hyperopic"]
    .map((name) => new Bot(app, name, {
      env: envConfig,
      openingTable: tableConfig,
      lambdaParams: {
        memory: 1792,
        timeout: Duration.minutes(10),
      },
    }))

new GameLambda(app, "LichessGameLambda", {
  env: envConfig,
  lambdaParams: {
    memory: 128,
    timeout: Duration.minutes(15)
  },
  botFunctions: bots.map((bot) => bot.moveLambdaName)
})
