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
    EventStreamConfigValues,
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
);

const bots = ["Myopic", "Hyperopic"].map((name) =>
    new BotStack(
        app,
        name,
        AccountAndRegionValues,
        BotLambdaConfigValues,
        OpeningTableConfigValues
    )
)

const gameFunction = new GameLambdaStack(
    app,
    "LichessGameLambda",
    AccountAndRegionValues,
    GameLambdaConfigValues,
    bots.map((bot) => bot.moveLambdaName)
)

const cluster = new ClusterStack(
    app,
    "Cluster",
    AccountAndRegionValues,
)


EventStreamConfigValues.forEach((config) => {
    new ChallengesTableStack(
        app,
        `${config.name}Challenges`,
        AccountAndRegionValues,
    )
    new EventStreamStack(
        app,
        `${config.name}EventStream`,
        AccountAndRegionValues,
        cluster.cluster,
        gameFunction.functionArn,
        config,
    )
})
