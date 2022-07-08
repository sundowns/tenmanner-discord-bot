import { Environment, StackProps } from "aws-cdk-lib";

export interface DaddysStackProps extends StackProps {
  env: DaddysLittleEnvironment;
}

export interface DaddysLittleEnvironment extends Environment {
  table_name: string | undefined;
}
