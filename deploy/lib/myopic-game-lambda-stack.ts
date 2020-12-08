import * as cdk from '@aws-cdk/core';
import {Duration} from '@aws-cdk/core';
import * as lambda from '@aws-cdk/aws-lambda';
import * as iam from '@aws-cdk/aws-iam';
import * as path from 'path';

export interface MyopicGameLambdaStackProps extends cdk.StackProps {
  account: string
  region: string
  functionName: string
  openingsTableName: string
  memorySize: number
  timeout: Duration
}

export class MyopicGameLambdaStack extends cdk.Stack {
  constructor(scope: cdk.Construct, id: string, props: MyopicGameLambdaStackProps) {
    super(scope, id, props);

    const gameHandler = new lambda.Function(this, `${id}-Function`, {
      runtime: lambda.Runtime.PROVIDED_AL2,
      handler: 'index.handler',
      code: lambda.Code.fromAsset(
        path.join(__dirname, '..', 'runtime', 'lambda.zip')
      ),
      functionName: props.functionName,
      timeout: props.timeout,
      retryAttempts: 0,
      memorySize: props.memorySize
    });

    // Add permissions for recursive invoking of the function and access to the opening database
    const ps = new iam.PolicyStatement()
    ps.addActions("lambda:InvokeFunction", "dynamodb:GetItem")
    ps.addResources(
      `arn:aws:lambda:${props.region}:${props.account}:function:${props.functionName}`,
      `arn:aws:dynamodb:${props.region}:${props.account}:table/${props.openingsTableName}`
    )
    gameHandler.addToRolePolicy(ps)
  }
}
