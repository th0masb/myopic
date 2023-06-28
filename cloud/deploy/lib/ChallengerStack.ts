import {aws_lambda as lambda, Duration, Stack} from "aws-cdk-lib";
import {Construct} from "constructs";
import {AccountAndRegion, BotChallengerConfig} from "../config";
import * as path from "path";

export class ChallengerStack extends Stack {
    constructor(
        scope: Construct,
        id: string,
        accountAndRegion: AccountAndRegion,
        config: BotChallengerConfig,
    ) {
        super(scope, id, {env: accountAndRegion});
        new lambda.DockerImageFunction(this, id, {
            functionName: id,
            retryAttempts: 0,
            memorySize: 128,
            timeout: Duration.minutes(3),
            code: lambda.DockerImageCode.fromImageAsset(
                path.join(__dirname, "..", "..", ".."),
                {
                    file: path.join("tools", "workspace.dockerfile"),
                    buildArgs: {
                        APP_NAME: "challenge",
                        APP_CONFIG: JSON.stringify({
                            token: config.token,
                            ourUserId: config.ourUserId
                        })
                    },
                }
            ),
        });
    }
}
