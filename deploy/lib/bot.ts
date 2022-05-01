import * as cdk from "@aws-cdk/core";
import * as path from "path";
import * as lambda from "@aws-cdk/aws-lambda";
import {CARGO_LAMBDAS, LambdaParameters, LambdaType} from "./common";
import * as iam from "@aws-cdk/aws-iam";

export interface OpeningTableConfig {
  readonly name: string,
  readonly region: string,
  readonly positionKey: string,
  readonly moveKey: string,
  readonly maxDepth: number,
}

export interface BotConfig extends cdk.StackProps {
  readonly lambdaParams: LambdaParameters,
  readonly openingTable: OpeningTableConfig,
}

export class Bot extends cdk.Stack {
  readonly moveLambdaName: string;
  private readonly id: string;

  constructor(scope: cdk.Construct, id: string, props: BotConfig) {
    super(scope, id, props);
    this.id = id;
    this.moveLambdaName = this.functionName(LambdaType.Move)
    for (const type of [LambdaType.Move, LambdaType.Benchmark]) {
      const cargoConfig = CARGO_LAMBDAS.get(type)!
      const name = this.functionName(type);
      const fn = new lambda.DockerImageFunction(this, name, {
        functionName: name,
        retryAttempts: 0,
        memorySize: props.lambdaParams.memory,
        timeout: props.lambdaParams.timeout,
        code: lambda.DockerImageCode.fromImageAsset(
          path.join(__dirname, "..", ".."),
          {
            file: path.join("tools", "lambda.dockerfile"),
            buildArgs: {
              APP_DIR: cargoConfig.cargoDir,
              APP_NAME: cargoConfig.cargoName,
              APP_CONFIG: JSON.stringify(props.openingTable)
            },
          }
        ),
      });
      const ps = new iam.PolicyStatement();
      ps.addActions("dynamodb:GetItem");
      const [region, account] = [props.env!.region, props.env!.account];
      ps.addResources(`arn:aws:dynamodb:${region}:${account}:table/${props.openingTable.name}`);
      fn.addToRolePolicy(ps);
    }
  }

  private functionName(type: LambdaType): string {
    return `${this.id}-${LambdaType[type]}`;
  }
}

