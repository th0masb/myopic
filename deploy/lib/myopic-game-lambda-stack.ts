import * as cdk from '@aws-cdk/core';
import * as lambda from '@aws-cdk/aws-lambda';
import * as path from 'path';
import {Duration} from "@aws-cdk/core";

export interface MyopicGameLambdaStackProps extends cdk.StackProps {
  functionName: string
  timeout: Duration
}

export class MyopicGameLambdaStack extends cdk.Stack {
  constructor(scope: cdk.Construct, id: string, props: MyopicGameLambdaStackProps) {
    super(scope, id, props);

    new lambda.Function(this, `${id}-Function`, {
      runtime: lambda.Runtime.PROVIDED_AL2,
      handler: 'index.handler',
      code: lambda.Code.fromAsset(
        path.join(__dirname, '..', 'runtime', 'lambda.zip')
      ),
      functionName: props.functionName,
      timeout: props.timeout,
      retryAttempts: 0,
    })
  }
}
