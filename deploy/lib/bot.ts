import * as path from "path";
import {CARGO_LAMBDAS, LambdaType} from "./cargo";
import { Stack } from "aws-cdk-lib";
import { aws_lambda as lambda } from "aws-cdk-lib";
import { aws_iam as iam } from "aws-cdk-lib";
import { Construct } from "constructs";
import {AccountAndRegion, LambdaConfig, OpeningTableConfig} from "../config";

export class Bot extends Stack {
  readonly moveLambdaName: string;
  private readonly id: string;

  constructor(
      scope: Construct,
      id: string,
      accountAndRegion: AccountAndRegion,
      lambdaConfig: LambdaConfig,
      openingTableConfig: OpeningTableConfig,
  ) {
    super(scope, id, {env: accountAndRegion});
    this.id = id;
    this.moveLambdaName = this.functionName(LambdaType.Move)
    for (const type of [LambdaType.Move, LambdaType.Benchmark]) {
      const cargoConfig = CARGO_LAMBDAS.get(type)!
      const name = this.functionName(type);
      const fn = new lambda.DockerImageFunction(this, name, {
        functionName: name,
        retryAttempts: 0,
        memorySize: lambdaConfig.memoryMB,
        timeout: lambdaConfig.timeout,
        code: lambda.DockerImageCode.fromImageAsset(
          path.join(__dirname, "..", ".."),
          {
            file: path.join("tools", "lambda.dockerfile"),
            buildArgs: {
              APP_DIR: cargoConfig.cargoDir,
              APP_NAME: cargoConfig.cargoName,
              APP_CONFIG: JSON.stringify({
                name: openingTableConfig.tableName,
                region: accountAndRegion.region,
                positionKey: openingTableConfig.positionAttributeName,
                moveKey: openingTableConfig.movesAttributeName,
                maxDepth: openingTableConfig.maxDepth,
              })
            },
          }
        ),
      });
      const ps = new iam.PolicyStatement();
      ps.addActions("dynamodb:GetItem");
      const {region, account} = accountAndRegion;
      ps.addResources(`arn:aws:dynamodb:${region}:${account}:table/${openingTableConfig.tableName}`);
      fn.addToRolePolicy(ps);
    }
  }

  private functionName(type: LambdaType): string {
    return `${this.id}-${LambdaType[type]}`;
  }
}

