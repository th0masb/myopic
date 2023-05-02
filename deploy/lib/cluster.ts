import {aws_ec2 as ec2, aws_ecs as ecs, aws_iam as iam, Stack} from "aws-cdk-lib";
import {Construct} from "constructs";
import {InstanceClass, InstanceSize, InstanceType} from "aws-cdk-lib/aws-ec2";
import {DockerImageAsset} from "aws-cdk-lib/aws-ecr-assets";
import * as path from "path";
import {AccountAndRegion} from "../config";

export class Cluster extends Stack {
    constructor(
        scope: Construct,
        id: string,
        accountAndRegion: AccountAndRegion,
        gameLambdaArn: string
    ) {
        super(scope, id, {env: accountAndRegion});
        const cluster = new ecs.Cluster(this, "ClusterNodes", {
            clusterName: "Myopic",
            vpc: new ec2.Vpc(this, "Vpc", {
                ipAddresses: ec2.IpAddresses.cidr("10.1.1.0/24"),
                maxAzs: 1
            }),

            capacity: {
                instanceType: InstanceType.of(InstanceClass.T3A, InstanceSize.NANO),
                maxCapacity: 2,
                allowAllOutbound: true,
            }
        })
        const taskDefinition = new ecs.TaskDefinition(this, "EventStreamTaskDefinition", {
            compatibility: ecs.Compatibility.EC2,
        })
        taskDefinition.addToTaskRolePolicy(this.createLambdaInvokePolicy(gameLambdaArn))
        const eventStreamImage = this.eventStreamImage()
        for (const bot of EVENT_STREAM_CONFIG) {
            taskDefinition.addContainer(bot.name, {
                image: eventStreamImage,
                memoryLimitMiB: 210,
                environment: {
                    LICHESS_AUTH_TOKEN: process.env[bot.authVar]!,
                    APP_CONFIG: JSON.stringify(bot.config)
                },
                logging: ecs.LogDrivers.awsLogs({
                    streamPrefix: "EventStream",
                    mode: ecs.AwsLogDriverMode.NON_BLOCKING,
                })
            })
        }
        new ecs.Ec2Service(this, "EventStreamService", {
            cluster,
            taskDefinition,
            circuitBreaker: { rollback: true },
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
            new DockerImageAsset(this, "EventStreamImage", {
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

const EVENT_STREAM_CONFIG = [
    {
        name: "Hyperopic",
        authVar: "HYPEROPIC_TOKEN",
        config: {
            "gameFunction": {
                "id": {"name": "LichessGameLambda"},
                "abortAfterSecs": 30
            },
            "moveFunction": {
                "name": "Hyperopic-Move"
            },
            "lichessBot": {
                "botId": "Hyperopic",
                "userMatchers": [
                    {
                        "include": true,
                        "pattern": "^th0masb$"
                    }
                ]
            }
        }
    },
    {
        name: "Myopic",
        authVar: "MYOPIC_TOKEN",
        config: {
            "gameFunction": {
                "id": {"name": "LichessGameLambda"},
                "abortAfterSecs": 30
            },
            "moveFunction": {
                "name": "Myopic-Move"
            },
            "lichessBot": {
                "botId": "myopic-bot",
                "userMatchers": [
                    {
                        "include": true,
                        "pattern": "^th0masb$"
                    }
                ]
            }
        }
    }
]
