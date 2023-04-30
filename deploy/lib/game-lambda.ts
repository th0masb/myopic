import {CARGO_LAMBDAS, LambdaParameters, LambdaType} from "./common";
import * as path from "path";
import { Stack, StackProps } from "aws-cdk-lib";
import { aws_lambda as lambda } from "aws-cdk-lib";
import { aws_iam as iam } from "aws-cdk-lib";
import { Construct } from "constructs";

export interface GameLambdaConfig extends StackProps {
    readonly lambdaParams: LambdaParameters,
    readonly botFunctions: string[]
}

export class GameLambda extends Stack {
    constructor(scope: Construct, id: string, props: GameLambdaConfig) {
        super(scope, id, props);
        const cargoConfig = CARGO_LAMBDAS.get(LambdaType.LichessGame)!
        const fn = new lambda.DockerImageFunction(this, id, {
            functionName: id,
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
                        APP_CONFIG: ""
                    },
                }
            ),
        });
        const ps = new iam.PolicyStatement();
        ps.addActions("lambda:InvokeFunction");
        const [region, account] = [props.env!.region, props.env!.account];
        const fnPrefix = `arn:aws:lambda:${region}:${account}:function`;
        ps.addResources(...props.botFunctions.map((bot) => `${fnPrefix}:${bot}`))
        fn.addToRolePolicy(ps);
    }
}
