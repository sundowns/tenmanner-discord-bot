#!/usr/bin/env node
import 'source-map-support/register';
import * as cdk from 'aws-cdk-lib';
import { ContainerStack, SecretsStack } from '../lib';

const app = new cdk.App();

const APP_NAME_PREFIX = "Discord10Manner"

const env = { account: process.env.CDK_DEFAULT_ACCOUNT, region: process.env.CDK_DEFAULT_REGION }

new SecretsStack(app, `${APP_NAME_PREFIX}SecretsStack`, {
  env
});

new ContainerStack(app, `${APP_NAME_PREFIX}ContainerStack`, {
  env
});
