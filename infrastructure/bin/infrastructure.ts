#!/usr/bin/env node
import 'source-map-support/register';
import * as cdk from 'aws-cdk-lib';
import { SecretsStack } from '../lib/secrets-stack';

const app = new cdk.App();

const APP_NAME_PREFIX = "Discord10Manner"

new SecretsStack(app, `${APP_NAME_PREFIX}SecretsStack`, {
  env: { account: process.env.CDK_DEFAULT_ACCOUNT, region: process.env.CDK_DEFAULT_REGION },
});