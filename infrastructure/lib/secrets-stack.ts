import { RemovalPolicy, SecretValue, Stack, StackProps } from "aws-cdk-lib";
import { Construct } from "constructs";
import * as secretsmanager from "aws-cdk-lib/aws-secretsmanager";

export class SecretsStack extends Stack {
  constructor(scope: Construct, id: string, props?: StackProps) {
    super(scope, id, props);

    new secretsmanager.Secret(this, "DiscordBot10MannerToken", {
      description: "The discord login token for the 10 manner bot",
      secretName: "DiscordBotToken10Manner",
      secretStringValue: new SecretValue(process.env.DISCORD_BOT_TOKEN),
      removalPolicy: RemovalPolicy.RETAIN,
    });
  }
}
