import * as cdk from "@aws-cdk/core";
import * as path from "path";
import * as lambda from "@aws-cdk/aws-lambda";
import * as iam from "@aws-cdk/aws-iam";

export interface BotStackConfig extends cdk.StackProps {
  readonly openingTableName: string;
  readonly params: Map<LambdaConfigType, LambdaParameters>;
}

export enum LambdaConfigType {
  Move,
  Game,
}

export interface LambdaParameters {
  readonly memory: number;
  readonly timeout: cdk.Duration;
}

export class BotStack extends cdk.Stack {
  private readonly id: string;

  constructor(scope: cdk.Construct, id: string, props: BotStackConfig) {
    super(scope, id, props);
    this.id = id;
    for (const [key, config] of configs2) {
      const name = this.functionName(key);
      const fn = new lambda.DockerImageFunction(this, name, {
        functionName: name,
        retryAttempts: 0,
        memorySize: props.params.get(config.type)!.memory,
        timeout: props.params.get(config.type)!.timeout,
        code: lambda.DockerImageCode.fromImageAsset(
          path.join(__dirname, "..", ".."),
          {
            file: path.join("tools", "lambda.dockerfile"),
            buildArgs: {
              APP_DIR: `${config.cargoDir}`,
              APP_NAME: `${config.cargoName}`,
            },
          }
        ),
      });
      if (key == LambdaType.Game) {
        const ps = new iam.PolicyStatement();
        ps.addActions("lambda:InvokeFunction", "dynamodb:GetItem");
        const [region, account] = [props.env!.region, props.env!.account];
        const fnPrefix = `arn:aws:lambda:${region}:${account}:function`;
        ps.addResources(
          // Recursively invoke itself
          `${fnPrefix}:${this.functionName(LambdaType.Game)}`,
          // Access the opening table
          `arn:aws:dynamodb:${region}:${account}:table/${props.openingTableName}`,
          // Access the move lambda for computations
          `${fnPrefix}:${this.functionName(LambdaType.Move)}`
        );
        fn.addToRolePolicy(ps);
      }
    }
  }

  private functionName(type: LambdaType): string {
    return `${this.id}-${LambdaType[type]}`;
  }
}

enum LambdaType {
  Benchmark,
  Move,
  Game,
}

interface AppConfig {
  readonly cargoDir: string;
  readonly cargoName: string;
  readonly type: LambdaConfigType;
}

const configs2: Map<LambdaType, AppConfig> = new Map([
  [
    LambdaType.Benchmark,
    {
      cargoDir: "lambda/benchmark",
      cargoName: "chessmove-benchmark-lambda",
      type: LambdaConfigType.Move,
    },
  ],
  [
    LambdaType.Move,
    {
      cargoDir: "lambda/chessmove",
      cargoName: "chessmove-lambda",
      type: LambdaConfigType.Move,
    },
  ],
  [
    LambdaType.Game,
    {
      cargoDir: "lambda/chessgame",
      cargoName: "chessgame-lambda",
      type: LambdaConfigType.Game,
    },
  ],
]);
