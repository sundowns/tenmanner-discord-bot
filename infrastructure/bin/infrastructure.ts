#!/usr/bin/env node
import "source-map-support/register";
import * as cdk from "aws-cdk-lib";
import { ContainerStack, DynamoStack, IamStack, SecretsStack } from "../lib";
import { DaddysLittleEnvironment } from "../types/environment";

const app = new cdk.App();

const APP_NAME_PREFIX = "Discord10Manner";

const env: DaddysLittleEnvironment = {
  account: process.env.CDK_DEFAULT_ACCOUNT,
  region: process.env.CDK_DEFAULT_REGION,
  table_name: "tenmannerreactions",
};

new DynamoStack(app, `${APP_NAME_PREFIX}StorageStack`, {
  env,
});

new IamStack(app, `${APP_NAME_PREFIX}IamStack`, { env });

new SecretsStack(app, `${APP_NAME_PREFIX}SecretsStack`, {
  env,
});

new ContainerStack(app, `${APP_NAME_PREFIX}ContainerStack`, {
  env,
});
