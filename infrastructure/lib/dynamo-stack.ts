import { Stack, RemovalPolicy } from "aws-cdk-lib";
import { Construct } from "constructs";
import * as dynamodb from "aws-cdk-lib/aws-dynamodb";
import { DaddysStackProps } from "../types/environment";

export class DynamoStack extends Stack {
  constructor(scope: Construct, id: string, props?: DaddysStackProps) {
    super(scope, id, props);

    console.log({ table_name: props?.env.table_name });

    const table = new dynamodb.Table(this, "tenmannerreactions", {
      tableName: props?.env.table_name,
      billingMode: dynamodb.BillingMode.PAY_PER_REQUEST,
      removalPolicy: RemovalPolicy.DESTROY,
      partitionKey: { name: "post_id", type: dynamodb.AttributeType.STRING },
      sortKey: { name: "user_id", type: dynamodb.AttributeType.STRING },
    });

    console.log("Table name:", table.tableName);
    console.log("Table arn:", table.tableArn);

    // 👇 add local secondary index
    table.addLocalSecondaryIndex({
      indexName: "responseIndex",
      sortKey: { name: "response", type: dynamodb.AttributeType.STRING },
      projectionType: dynamodb.ProjectionType.ALL,
    });
  }
}
