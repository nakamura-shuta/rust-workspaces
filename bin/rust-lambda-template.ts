#!/usr/bin/env node
import * as cdk from 'aws-cdk-lib';
import { RustLambdaTemplateStack } from '../lib/rust-lambda-template-stack';

const app = new cdk.App();
new RustLambdaTemplateStack(app, 'RustLambdaTemplateStack');
