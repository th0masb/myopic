export enum LambdaType {
    Benchmark,
    Move,
    LichessGame,
}

export const CargoBinNames: Map<LambdaType, string> = new Map([
    [
        LambdaType.Benchmark,
        "chessmove-benchmark-lambda",
    ],
    [
        LambdaType.Move,
        "chessmove",
    ],
    [
        LambdaType.LichessGame,
        "lichess-game-lambda",
    ],
]);
