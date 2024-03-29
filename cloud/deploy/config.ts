import { Duration } from "aws-cdk-lib";
import * as process from "process";

export type LambdaConfig = {
    timeout: Duration
    memoryMB: number
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
    tableName: string
    positionAttributeName: string
    readCapacity: number
    writeCapacity: number
    movesAttributeName: string
    maxDepth: number
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
    account: string
    region: string
}

export const AccountAndRegionValues: AccountAndRegion = {
    region: process.env.MYOPIC_AWS_REGION!,
    account: process.env.MYOPIC_AWS_ACCOUNT!,
}

export type BotConfig = {
    name: string
    authTokenVar: string
    challengerConfig?: BotChallengerConfig
    eventStreamConfig: {
        gameFunction: {
            id: { name: string }
            abortAfterSecs: number,
            maxRecursionDepth: number,
        }
        moveFunction: { name: string }
        lichessBot: {
            botId: string
            userMatchers?: {
                include: boolean
                pattern: string
            }[]
        }
        eventLoop?: {
            retryWaitDurationSecs: number
            statusPollGapSecs: number
            maxStreamLifeMins: number
        }
        rateLimits: {
            challengeTable: { name: string }
            maxDailyChallenges: number
            maxDailyUserChallenges: number
            excluded?: string[]
        }
    }
}

export type BotChallengerConfig = {
    ourUserId: string,
    token: string,
}

export const BotConfigValues: BotConfig[] = [
    {
        name: "Hyperopic",
        authTokenVar: "HYPEROPIC_TOKEN",
        challengerConfig: {
            ourUserId: "hyperopic",
            token: process.env["HYPEROPIC_TOKEN"]!,
        },
        eventStreamConfig: {
            gameFunction: {
                id: { name: "HyperopicGameLambda" },
                abortAfterSecs: 30,
                maxRecursionDepth: 3
            },
            moveFunction: {
                name: "Hyperopic-Move"
            },
            lichessBot: {
                botId: "hyperopic",
                userMatchers: [
                    {
                        include: true,
                        pattern: "^th0masb$"
                    }
                ]
            },
            rateLimits: {
                challengeTable: { name: "HyperopicChallenges" },
                maxDailyChallenges: 100,
                maxDailyUserChallenges: 5,
            }
        }
    },
    {
        name: "Myopic",
        authTokenVar: "MYOPIC_TOKEN",
        challengerConfig: {
            ourUserId: "myopic-bot",
            token: process.env["MYOPIC_TOKEN"]!,
        },
        eventStreamConfig: {
            gameFunction: {
                id: { name: "MyopicGameLambda" },
                abortAfterSecs: 30,
                maxRecursionDepth: 3
            },
            moveFunction: {
                name: "Myopic-Move"
            },
            lichessBot: {
                botId: "myopic-bot",
            },
            rateLimits: {
                challengeTable: { name: "MyopicChallenges" },
                maxDailyChallenges: 100,
                maxDailyUserChallenges: 5,
                excluded: ["hyperopic"]
            }
        }
    }
]

