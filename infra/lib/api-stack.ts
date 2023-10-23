import { Construct } from "constructs";
import * as cdk from "aws-cdk-lib";
import * as lambda from "aws-cdk-lib/aws-lambda";
import * as path from "path";
import * as apigwv2 from "@aws-cdk/aws-apigatewayv2-alpha";
import * as kms from "aws-cdk-lib/aws-kms";
import * as dynamodb from "aws-cdk-lib/aws-dynamodb";
import * as integration from "@aws-cdk/aws-apigatewayv2-integrations-alpha";

export class ApiStack extends cdk.Stack {
  public readonly key: kms.IKey;
  public readonly table: dynamodb.ITable;
  public readonly fn: lambda.IFunction;
  public readonly api: apigwv2.HttpApi;

  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    this.table = new dynamodb.Table(this, "rust_payment_table", {
      tableName: "wallet",
      partitionKey: { name: "pk", type: dynamodb.AttributeType.STRING },
      sortKey: { name: "sk", type: dynamodb.AttributeType.STRING },
      billingMode: dynamodb.BillingMode.PAY_PER_REQUEST,
    });

    this.key = new kms.Key(this, "rust_payment_key", {
      alias: "rust_payment_key",
      keySpec: kms.KeySpec.RSA_4096,
      keyUsage: kms.KeyUsage.ENCRYPT_DECRYPT,
    });

    this.fn = new lambda.Function(this, "rust_payment_api_lambda", {
      functionName: "rust_payment_api_lambda",
      runtime: lambda.Runtime.PROVIDED_AL2,
      architecture: lambda.Architecture.ARM_64,
      handler: "bootstrap",
      code: new lambda.AssetCode(
        path.join(__dirname, "../../target/lambda/rust-api/bootstrap.zip")
      ),
      environment: {
        DYNAMODB_ENDPOINT: "https://dynamodb.us-east-1.amazonaws.com",
        PAYMENTS_TABLE_NAME: this.table.tableName,
        KMS_ENDPOINT: "https://kms.us-east-1.amazonaws.com",
        KMS_RSA_KEY_ID: this.key.keyId,
        GOOGLE_OAUTH_CLIENT_ID:
          "104079326466-rqglh7msot9iphnvv3qeodqs4j29i8ld.apps.googleusercontent.com",
      },
    });

    this.table.grantReadWriteData(this.fn);
    this.key.grantEncryptDecrypt(this.fn);
    this.key.grant(this.fn, 'kms:GetPublicKey')

    this.api = new apigwv2.HttpApi(this, "rust_payment_http_api");
    this.api.addRoutes({
      path: "/",
      methods: [apigwv2.HttpMethod.ANY],
      integration: new integration.HttpLambdaIntegration(
        "http-lambda-integration",
        this.fn
      ),
    });

    this.api.addRoutes({
      path: "/{proxy+}",
      methods: [apigwv2.HttpMethod.ANY],
      integration: new integration.HttpLambdaIntegration(
        "http-lambda-integration",
        this.fn
      ),
    });
  }
}
