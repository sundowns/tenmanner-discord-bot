AWS_DEFAULT_REGION='ap-southeast-2'
STACK_NAME='DiscordBot10MannerDeployment'

set -e
echo "Creating $STACK_NAME"
aws cloudformation create-stack --stack-name $STACK_NAME --template-body=file://pipeline.yml --capabilities='CAPABILITY_NAMED_IAM'
echo "Waiting for stack to deploy..."
aws cloudformation wait stack-create-complete --stack-name $STACK_NAME
echo "Stack deployed!"