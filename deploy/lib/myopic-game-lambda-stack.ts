import * as cdk from '@aws-cdk/core';
import {Duration} from '@aws-cdk/core';
import * as lambda from '@aws-cdk/aws-lambda';
import * as iam from '@aws-cdk/aws-iam';
import * as path from 'path';

export interface MyopicGameLambdaStackProps extends cdk.StackProps {
  functionName: string
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

    // Add permissions for recursive invoking of the function
    const ps = new iam.PolicyStatement()
    ps.addAllResources()
    ps.addActions("lambda:InvokeFunction")
    gameHandler.addToRolePolicy(ps)
  }
}
