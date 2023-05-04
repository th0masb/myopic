import {aws_dynamodb as db, RemovalPolicy, Stack} from "aws-cdk-lib";
import {Construct} from "constructs";
import {AccountAndRegion, AccountAndRegionValues, OpeningTableConfig} from "../config";

export class OpeningDatabaseStack extends Stack {
  constructor(
      scope: Construct,
      id: string,
      accountAndRegion: AccountAndRegion,
      tableConfig: OpeningTableConfig
  ) {
    super(scope, id, {env: AccountAndRegionValues});
    new db.Table(this, `${id}-Openings`, {
      tableName: tableConfig.tableName,
      billingMode: db.BillingMode.PAY_PER_REQUEST,
      removalPolicy: RemovalPolicy.RETAIN,
      partitionKey: {
        name: tableConfig.positionAttributeName,
        type: db.AttributeType.STRING,
      },
    })
  }
}