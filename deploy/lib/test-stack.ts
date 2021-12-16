import * as cdk from '@aws-cdk/core';
import { DockerImageAsset } from '@aws-cdk/aws-ecr-assets';

export class TestStack extends cdk.Stack {
  constructor(scope: cdk.Construct, id: string, props: cdk.StackProps) {
    super(scope, id, props);
  }
}
