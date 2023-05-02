#!/usr/bin/env node
import "source-map-support/register";
import {App} from 'aws-cdk-lib';
import {OpeningDatabase} from "../lib/opening-db";
import {Bot} from "../lib/bot";
import {GameLambda} from "../lib/game-lambda";
import {Cluster} from "../lib/cluster";
import {
    AccountAndRegionValues,
    BotLambdaConfigValues,
    GameLambdaConfigValues,
    OpeningTableConfigValues
} from "../config";

const app = new App();

new OpeningDatabase(
    app,
    "MyopicDatabaseStack",
    AccountAndRegionValues,
    OpeningTableConfigValues,
);

const bots = ["Myopic", "Hyperopic"].map((name) =>
    new Bot(
        app,
        name,
        AccountAndRegionValues,
        BotLambdaConfigValues,
        OpeningTableConfigValues
    )
)

const gameFunction = new GameLambda(
    app,
    "LichessGameLambda",
    AccountAndRegionValues,
    GameLambdaConfigValues,
    bots.map((bot) => bot.moveLambdaName)
)

new Cluster(
    app,
    "Cluster",
    AccountAndRegionValues,
    gameFunction.functionArn
)
