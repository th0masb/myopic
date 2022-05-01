import * as cdk from "@aws-cdk/core";

export interface LambdaParameters {
    readonly memory: number;
    readonly timeout: cdk.Duration;
}

export enum LambdaType {
    Benchmark,
    Move,
    LichessGame,
}

export interface CargoIdentifier {
    readonly cargoDir: string;
    readonly cargoName: string;
}

export const CARGO_LAMBDAS: Map<LambdaType, CargoIdentifier> = new Map([
    [
        LambdaType.Benchmark,
        {
            cargoDir: "lambda/benchmark",
            cargoName: "chessmove-benchmark-lambda",
        },
    ],
    [
        LambdaType.Move,
        {
            cargoDir: "lambda/chessmove2",
            cargoName: "chessmove2",
        },
    ],
    [
        LambdaType.LichessGame,
        {
            cargoDir: "lambda/chessgame",
            cargoName: "lichess-game-lambda",
        },
    ],
]);
