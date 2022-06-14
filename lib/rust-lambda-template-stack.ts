import { Stack, StackProps } from 'aws-cdk-lib';
import * as s3 from 'aws-cdk-lib/aws-s3';
import { BlockPublicAccess } from 'aws-cdk-lib/aws-s3';
import { Construct } from 'constructs';
import { RustFunction, Settings } from 'rust.aws-cdk-lambda';
// uncomment for local testing
// import { RustFunction, Settings } from '../../../lib';

export class RustLambdaTemplateStack extends Stack {
    constructor(scope: Construct, id: string, props?: StackProps) {
        super(scope, id, props);

        // Set the base Cargo workspace directory
        Settings.WORKSPACE_DIR = 'lambda_workspaces';

        // Enable optional features and env variables at build (compile) time.
        Settings.FEATURES = [
            'my-dev-feature',
            // uncomment to see how the lambda output changes!
            // 'my-prod-feature',
        ];
        Settings.BUILD_ENVIRONMENT = {
            LEARN_RUST_URL: 'https://doc.rust-lang.org',
        };

        let myLambda1 = new RustFunction(this, 'MyRustLambda1', {
            package: 'my_lambda1',
            // Useful so library logs show up in CloudWatch
            setupLogging: true,
            environment: {
                ENV_STR: "bar",
            },
        });

        let _myLambda2 = new RustFunction(this, 'MyRustLambda2', {
            package: 'my_lambda2',
            setupLogging: true,
        });
    }
}
