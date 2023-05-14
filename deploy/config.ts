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

export type EventStreamConfig = {
    name: string
    authTokenVar: string
    config: {
        gameFunction: {
            id: { name: string }
            abortAfterSecs: number
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
        }
    }
}

export const EventStreamConfigValues: EventStreamConfig[] = [
    {
        name: "Hyperopic",
        authTokenVar: "HYPEROPIC_TOKEN",
        config: {
            gameFunction: {
                id: { name: "LichessGameLambda" },
                abortAfterSecs: 30
            },
            moveFunction: {
                name: "Hyperopic-Move"
            },
            lichessBot: {
                botId: "Hyperopic",
                userMatchers: [
                    {
                        include: true,
                        pattern: "^th0masb$"
                    }
                ]
            },
            eventLoop: {
                maxStreamLifeMins: 60,
                retryWaitDurationSecs: 5,
                statusPollGapSecs: 60,
            },
            rateLimits: {
                challengeTable: { name: "HyperopicChallenges" },
                maxDailyChallenges: 100,
                maxDailyUserChallenges: 100,
            }
        }
    },
    {
        name: "Myopic",
        authTokenVar: "MYOPIC_TOKEN",
        config: {
            gameFunction: {
                id: { name: "LichessGameLambda" },
                abortAfterSecs: 30
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
            }
        }
    }
]

