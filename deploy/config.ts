import { Duration } from "aws-cdk-lib";
import * as process from "process";

export type LambdaConfig = {
    readonly timeout: Duration
    readonly memoryMB: number
}

export const GameLambdaConfigValues: LambdaConfig = {
    memoryMB: 128,
    timeout: Duration.minutes(15)
}

export const BotLambdaConfigValues: LambdaConfig = {
    memoryMB: 1792,
    timeout: Duration.minutes(10)
}

export type OpeningTableConfig = {
    readonly tableName: string
    readonly positionAttributeName: string
    readonly readCapacity: number
    readonly writeCapacity: number
    readonly movesAttributeName: string
    readonly maxDepth: number
}

export const OpeningTableConfigValues: OpeningTableConfig = {
    tableName: "MyopicOpenings",
    positionAttributeName: "PositionFEN",
    readCapacity: 2,
    writeCapacity: 5,
    movesAttributeName: "Moves",
    maxDepth: 10
}

export type AccountAndRegion = {
    readonly account: string
    readonly region: string
}

export const AccountAndRegionValues: AccountAndRegion = {
    region: process.env.MYOPIC_AWS_REGION!,
    account: process.env.MYOPIC_AWS_ACCOUNT!,
}

