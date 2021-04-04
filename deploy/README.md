# Myopic AWS infrastructure

This subproject is a cdk app which provisions the aws infrastructure required
to run the engine. Currently there are two stacks, one for the three lambdas 
and one for the openings table.

## Lambda stack deployment

 * Create a 'runtime' subdirectory in the same dir as this readme
 * Ensure the three lambda projects - bench, game, move - have been compiled
   for the target architecture x86_64-unknown-linux-gnu
 * Source the deploy/zip-runtime.sh script which provides the function "zip_runtime"
 * For each lambda project run `zip_runtime <path to binary> <path to deploy>/runtime`
 * Deploy the 'MyopicLambdaStack' as usual using cdk supplying account and region
   via env variables, e.g. `ACCOUNT=<account id> REGION=<region> cdk synth MyopicLambdaStack`

## Table deployment

This is just a standard cdk deployment for the 'MyopicDatabaseStack' stack. Make
sure to supply the account and region like in the lambda deployment.