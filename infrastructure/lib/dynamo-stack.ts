import { Stack, StackProps, RemovalPolicy } from "aws-cdk-lib";
import { Construct } from "constructs";
import * as dynamodb from "aws-cdk-lib/aws-dynamodb";

export class DynamoStack extends Stack {
  constructor(scope: Construct, id: string, props?: StackProps) {
    super(scope, id, props);

    const table = new dynamodb.Table(this, "tenmannerreactions", {
      tableName: "tenmannerreactions",
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
