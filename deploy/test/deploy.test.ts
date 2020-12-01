import { expect as expectCDK, matchTemplate, MatchStyle } from '@aws-cdk/assert';
import * as cdk from '@aws-cdk/core';
import * as Deploy from '../lib/myopic-game-lambda-stack';

test('Empty Stack', () => {
    const app = new cdk.App();
    // WHEN
    const stack = new Deploy.MyopicGameLambdaStack(app, 'MyTestStack', {
      functionName: "MyTestFunction",
      timeout: cdk.Duration.minutes(15),
      memorySize: 1024
    });
    // THEN
    expectCDK(stack).to(matchTemplate({
      "Resources": {}
    }, MatchStyle.EXACT))
});
