import * as cdk from '@aws-cdk/core';
import {Duration} from '@aws-cdk/core';
import * as lambda from '@aws-cdk/aws-lambda';
import * as iam from '@aws-cdk/aws-iam';
import * as path from 'path';

export interface MyopicGameLambdaStackProps extends cdk.StackProps {
  account: string
  region: string
  openingsTableName: string
  gameLambdaConfig: LambdaConfig
  benchLambdaConfig: LambdaConfig
}

export interface LambdaConfig {
  readonly functionName: string
  readonly memoryLimit: number,
  readonly timeout: Duration,
  readonly assetName: string
}

const HANDLER_NAME: string = "index.handler"

export class MyopicGameLambdaStack extends cdk.Stack {
  constructor(scope: cdk.Construct, id: string, props: MyopicGameLambdaStackProps) {
    super(scope, id, props);

    // Create the benchmarking function
    new lambda.Function(this, `${id}-BenchmarkFn`, {
      runtime: lambda.Runtime.PROVIDED_AL2,
      handler: HANDLER_NAME,
      code: lambda.Code.fromAsset(
        path.join(__dirname, '..', 'runtime', props.benchLambdaConfig.assetName)
      ),
      functionName: props.benchLambdaConfig.functionName,
      timeout: props.benchLambdaConfig.timeout,
      retryAttempts: 0,
      memorySize: props.benchLambdaConfig.memoryLimit,
    });

    // Create the game handler function
    const gameHandler = new lambda.Function(this, `${id}-Function`, {
      runtime: lambda.Runtime.PROVIDED_AL2,
      handler: HANDLER_NAME,
      code: lambda.Code.fromAsset(
        path.join(__dirname, '..', 'runtime', props.gameLambdaConfig.assetName)
      ),
      functionName: props.gameLambdaConfig.functionName,
      timeout: props.gameLambdaConfig.timeout,
      retryAttempts: 0,
      memorySize: props.gameLambdaConfig.memoryLimit
    });

    // Add permissions for recursive invoking of the function and access to the opening database
    const ps = new iam.PolicyStatement()
    ps.addActions("lambda:InvokeFunction", "dynamodb:GetItem")
    ps.addResources(
      `arn:aws:lambda:${props.region}:${props.account}:function:${props.gameLambdaConfig.functionName}`,
      `arn:aws:dynamodb:${props.region}:${props.account}:table/${props.openingsTableName}`
    )
    gameHandler.addToRolePolicy(ps)
  }
}
