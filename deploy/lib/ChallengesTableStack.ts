import {aws_dynamodb as db, RemovalPolicy, Stack} from "aws-cdk-lib";
import {Construct} from "constructs";
import {AccountAndRegion, EventStreamConfig} from "../config";
import {BillingMode} from "aws-cdk-lib/aws-dynamodb";

export class ChallengesTableStack extends Stack {
    readonly tableArn: string

    constructor(
        scope: Construct,
        id: string,
        accountAndRegion: AccountAndRegion,
        eventStreamConfig: EventStreamConfig,
    ) {
        super(scope, id, {env: accountAndRegion});
        const table = new db.Table(this, `${id}-Table`, {
            billingMode: BillingMode.PAY_PER_REQUEST,
            tableName: eventStreamConfig.config.rateLimits.challengeTable.name,
            removalPolicy: RemovalPolicy.DESTROY,
            timeToLiveAttribute: "Expiry",
            partitionKey: {
                name: "ChallengerID",
                type: db.AttributeType.STRING
            },
            sortKey: {
                name: "ChallengeID",
                type: db.AttributeType.STRING
            },
        })
        this.tableArn = table.tableArn
        // The secondary index allows us to efficiently query the challenges of any particular day
        table.addGlobalSecondaryIndex({
            indexName: "EpochDayIndex",
            partitionKey: {
                name: "ChallengeDay",
                type: db.AttributeType.NUMBER,
            },
            sortKey: {
                name: "ChallengeID",
                type: db.AttributeType.STRING
            },
        })
    }
}