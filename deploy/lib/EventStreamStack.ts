import {aws_ecs as ecs, aws_iam as iam, Stack} from "aws-cdk-lib";
import {Construct} from "constructs";
import {AccountAndRegion, EventStreamConfig} from "../config";
import {DockerImageAsset} from "aws-cdk-lib/aws-ecr-assets";
import * as path from "path";

export class EventStreamStack extends Stack {
    constructor(
        scope: Construct,
        id: string,
        accountAndRegion: AccountAndRegion,
        cluster: ecs.Cluster,
        gameLambdaArn: string,
        config: EventStreamConfig
    ) {
        super(scope, id, {env: accountAndRegion});
        const taskDefinition = new ecs.TaskDefinition(this, "EventStreamTaskDefinition", {
            compatibility: ecs.Compatibility.EC2,
        })
        taskDefinition.addToTaskRolePolicy(this.createLambdaInvokePolicy(gameLambdaArn))
        const eventStreamImage = this.eventStreamImage()
        taskDefinition.addContainer(config.name, {
            image: eventStreamImage,
            memoryLimitMiB: 210,
            environment: {
                LICHESS_AUTH_TOKEN: process.env[config.authTokenVar]!,
                APP_CONFIG: JSON.stringify(config.config)
            },
            logging: ecs.LogDrivers.awsLogs({
                streamPrefix: "EventStream",
                mode: ecs.AwsLogDriverMode.NON_BLOCKING,
            })
        })
        new ecs.Ec2Service(this, "EventStreamService", {
            cluster,
            taskDefinition,
            circuitBreaker: {rollback: true},
            desiredCount: 1,
            minHealthyPercent: 0,
            maxHealthyPercent: 100,
        })
    }

    private createLambdaInvokePolicy(functionArn: string) {
        const ps = new iam.PolicyStatement();
        ps.addActions("lambda:InvokeFunction");
        ps.addResources(functionArn)
        return ps
    }

    private eventStreamImage() {
        return ecs.ContainerImage.fromDockerImageAsset(
            new DockerImageAsset(this, `EventStreamImage`, {
                directory: path.join(__dirname, "..", ".."),
                file: path.join("tools", "lambda.dockerfile"),
                buildArgs: {
                    APP_DIR: "event-stream",
                    APP_NAME: "event-stream",
                },
            })
        )
    }
}
