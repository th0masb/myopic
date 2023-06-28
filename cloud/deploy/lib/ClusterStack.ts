import {aws_ec2 as ec2, aws_ecs as ecs, Stack} from "aws-cdk-lib";
import {Construct} from "constructs";
import {InstanceClass, InstanceSize, InstanceType, SubnetType} from "aws-cdk-lib/aws-ec2";
import {AccountAndRegion} from "../config";

export class ClusterStack extends Stack {
    readonly cluster: ecs.Cluster

    constructor(
        scope: Construct,
        id: string,
        accountAndRegion: AccountAndRegion,
    ) {
        super(scope, id, {env: accountAndRegion});
        this.cluster = new ecs.Cluster(this, "ClusterNodes", {
            clusterName: "Myopic",
            vpc: new ec2.Vpc(this, "Vpc", {
                ipAddresses: ec2.IpAddresses.cidr("10.2.1.0/24"),
                maxAzs: 1,
                // Expensive!
                natGateways: 0,
                subnetConfiguration: [
                    {
                        name: "Public",
                        subnetType: SubnetType.PUBLIC
                    }
                ]
            }),
            capacity: {
                instanceType: InstanceType.of(InstanceClass.T3A, InstanceSize.NANO),
                maxCapacity: 2,
                allowAllOutbound: true,
            }
        })
    }
}
