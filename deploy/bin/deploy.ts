#!/usr/bin/env node
import "source-map-support/register";
import {App} from 'aws-cdk-lib';
import {OpeningDatabaseStack} from "../lib/OpeningDatabaseStack";
import {BotStack} from "../lib/BotStack";
import {GameLambdaStack} from "../lib/GameLambdaStack";
import {ClusterStack} from "../lib/ClusterStack";
import {
    AccountAndRegionValues,
    BotLambdaConfigValues,
    BotConfigValues,
    GameLambdaConfigValues,
    OpeningTableConfigValues
} from "../config";
import {EventStreamStack} from "../lib/EventStreamStack";
import {ChallengesTableStack} from "../lib/ChallengesTableStack";

const app = new App();

new OpeningDatabaseStack(
    app,
    "MyopicDatabaseStack",
    AccountAndRegionValues,
    OpeningTableConfigValues,
)

const cluster = new ClusterStack(
    app,
    "Cluster",
    AccountAndRegionValues,
)

BotConfigValues.forEach((config) => {
    const bot = new BotStack(
        app,
        config.name,
        AccountAndRegionValues,
        BotLambdaConfigValues,
        OpeningTableConfigValues
    )
    const gameFunction = new GameLambdaStack(
        app,
        `${config.name}GameLambda`,
        AccountAndRegionValues,
        GameLambdaConfigValues,
        config.eventStreamConfig.gameFunction.id.name,
        bot.moveLambdaName
    )
    const challengesTable = new ChallengesTableStack(
        app,
        `${config.name}Challenges`,
        AccountAndRegionValues,
        config,
    )
    new EventStreamStack(
        app,
        `${config.name}EventStream`,
        AccountAndRegionValues,
        cluster.cluster,
        gameFunction.functionArn,
        challengesTable.tableArn,
        config,
    )
})
