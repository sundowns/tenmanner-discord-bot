# Discord Signup Bot

A discord bot, written in Rust with [serenity](https://github.com/serenity-rs/serenity), to create signup posts that users can interact with and indicate attendance. Originally created to organise CS:GO 10 man lobbies, but can really be applied to anything.

![lobby signup sheet](/media/signup.png)

## Commands

- `/lobby <when>` - Create a new signup post
- `/delete <id>` - Deletes a signup post using the provided ID. The ID of a post is displayed in the footster of a signup post

---

## Configuring the bot

There are two configuration files, one for the bot and one for infrastructure. The latter is only required to use the AWS CDK deployment bundled, but the bot can be deployed through any other means.

Make a copy of `.env.template` file under `/bot` in the same directory and rename it to `.env`. Provide values for each of the environment variables.

## Running the bot

`cargo run`

Optionally a value for the ENVIRONMENT env var can be provided to instead pull config values from `.env.prod` or `.env.dev`.

`ENVIRONMENT=prod cargo run`
`ENVIRONMENT=dev cargo run`

When not provided, the default `.env` file is used.

## Deploying with the AWS CDK

Populate the values in `/infrastructure/.env`.

Run the [AWS CDK](https://docs.aws.amazon.com/cdk/v2/guide/home.html)
`cdk deploy`

---

## Outstanding work / TODO

- [MVP] Deploy infra (ecr & secrets) via AWS CDK
- [MVP] Some sort of CI/CD
- [MVP] Update how we remove/insert records to not use spaces.
- [BUG] Records seem to be changing / disappearing - this is probably a concurrency problem. We can't use the post to maintain state if we're allowing concurrent update. Consider storing in a table (dynamo, or even key-value)
- [JUICE] New slash command `/start <id>` to @ reactors about gaming time
