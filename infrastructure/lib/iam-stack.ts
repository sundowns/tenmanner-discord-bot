import { Stack } from "aws-cdk-lib";
import { Construct } from "constructs";
import * as iam from "aws-cdk-lib/aws-iam";
import { DaddysStackProps } from "../types/environment";
import { PolicyStatement, PolicyStatementProps } from "aws-cdk-lib/aws-iam";

export class IamStack extends Stack {
  constructor(scope: Construct, id: string, props?: DaddysStackProps) {
    super(scope, id, props);

    const user = new iam.User(this, "TenmannerDiscordBotUser", {
      userName: "TenmannerDiscordBotUser",
    });

    console.log("User arn:", user.userArn);

    const user_policy = new iam.Policy(this, "TenmannerBotPolicy", {});

    user_policy.addStatements(
      new PolicyStatement({
        sid: "DynamoTableAccess",
        resources: [
          `arn:aws:dynamodb:${props?.env.region}:${props?.env.account}:table/${props?.env.table_name}`,
        ],
        actions: ["dynamodb:Describe*", "dynamodb:Put*", "dynamodb:Query"],
      })
    );
    user_policy.attachToUser(user);
  }
}
