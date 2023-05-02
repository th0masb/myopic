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
            cargoDir: "lambda/chessmove",
            cargoName: "chessmove",
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
