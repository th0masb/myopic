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
      billingMode: db.BillingMode.PROVISIONED,
      readCapacity: tableConfig.readCapacity,
      writeCapacity: tableConfig.writeCapacity,
      removalPolicy: RemovalPolicy.DESTROY,
      partitionKey: {
        name: tableConfig.positionAttributeName,
        type: db.AttributeType.STRING,
      },
    })
  }
}