import { aws_dynamodb as db } from "aws-cdk-lib";
import { Stack, StackProps, RemovalPolicy } from "aws-cdk-lib";
import { Construct } from "constructs";

export interface OpeningDatabaseProps extends StackProps {
  readonly openingsTableName: string,
  readonly positionAttributeName: string
  readonly readCapacity: number
  readonly writeCapacity: number
}

export class OpeningDatabase extends Stack {

  constructor(scope: Construct, id: string, props: OpeningDatabaseProps) {
    super(scope, id, props);

    new db.Table(this, `${id}-Openings`, {
      tableName: props.openingsTableName,
      billingMode: db.BillingMode.PROVISIONED,
      readCapacity: props.readCapacity,
      writeCapacity: props.writeCapacity,
      removalPolicy: RemovalPolicy.DESTROY,
      partitionKey: {
        name: props.positionAttributeName,
        type: db.AttributeType.STRING,
      },
    })
  }

}