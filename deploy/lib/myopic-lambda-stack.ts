import * as cdk from '@aws-cdk/core';
import {Duration} from '@aws-cdk/core';
import * as lambda from '@aws-cdk/aws-lambda';
import * as iam from '@aws-cdk/aws-iam';
import * as path from 'path';

export interface MyopicLambdaStackProps extends cdk.StackProps {
  account: string
  region: string
  openingsTableName: string
  gameLambdaConfig: LambdaConfig
  benchLambdaConfig: LambdaConfig
  moveLambdaConfig: LambdaConfig
}

export interface LambdaConfig {
  readonly functionName: string
  readonly memoryLimit: number,
  readonly timeout: Duration,
  readonly assetName: string
}

const HANDLER_NAME: string = "index.handler"

export class MyopicLambdaStack extends cdk.Stack {
  constructor(scope: cdk.Construct, id: string, props: MyopicLambdaStackProps) {
    super(scope, id, props);

    // Create the move function
    new lambda.Function(this, `${id}-MoveFn`, {
      runtime: lambda.Runtime.PROVIDED_AL2,
      handler: HANDLER_NAME,
      code: lambda.Code.fromAsset(
        path.join(__dirname, '..', 'runtime', props.moveLambdaConfig.assetName)
      ),
      functionName: props.moveLambdaConfig.functionName,
      timeout: props.moveLambdaConfig.timeout,
      retryAttempts: 0,
      memorySize: props.moveLambdaConfig.memoryLimit,
    })

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
    const gameHandler = new lambda.Function(this, `${id}-GameFn`, {
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

    const ps = new iam.PolicyStatement()
    ps.addActions("lambda:InvokeFunction", "dynamodb:GetItem")
    ps.addResources(
      // Recursively invoke itself
      `arn:aws:lambda:${props.region}:${props.account}:function:${props.gameLambdaConfig.functionName}`,
      // Access the opening table
      `arn:aws:dynamodb:${props.region}:${props.account}:table/${props.openingsTableName}`,
      // Access the move lambda for computations
        `arn:aws:lambda:${props.region}:${props.account}:function:${props.moveLambdaConfig.functionName}`,
    )
    gameHandler.addToRolePolicy(ps)
  }
}
