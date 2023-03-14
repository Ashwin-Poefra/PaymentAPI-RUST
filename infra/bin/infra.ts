#!/usr/bin/env node
import "source-map-support/register";
import * as cdk from "aws-cdk-lib";
import { ApiStack } from "../lib/api-stack";

const app = new cdk.App();
new ApiStack(app, "api-stack", {
  env: { account: "582250362323", region: "us-east-1" },
});
