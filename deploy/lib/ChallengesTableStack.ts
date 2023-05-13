import {aws_dynamodb as db, RemovalPolicy, Stack} from "aws-cdk-lib";
import {Construct} from "constructs";
import {AccountAndRegion} from "../config";
import {BillingMode} from "aws-cdk-lib/aws-dynamodb";

export class ChallengesTableStack extends Stack {
    readonly tableArn: string

    constructor(scope: Construct, id: string, accountAndRegion: AccountAndRegion) {
        super(scope, id, {env: accountAndRegion});
        const table = new db.Table(this, `${id}-Table`, {
            billingMode: BillingMode.PAY_PER_REQUEST,
            tableName: `${id}-Table`,
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
            indexName: `${id}-DayIndex`,
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