# Discord Signup Bot

A discord bot, written in Rust with serenity, to create signup posts that users can interact with and indicate attendance. Originally created to organise CS:GO 10 man lobbies, but can really be applied to anything.

![lobby signup sheet](/media/signup.png)

## Commands

- `/lobby <when>` - Create a new signup post
- `/delete <id>` - Deletes a signup post using the provided ID. The ID of a post is displayed in the footster of a signup post

# Outstanding work / TODO

- [CORE] Host the bot in AWS
- [CORE] Some sort of deployment/restart so i dont have to ssh in
- [JUICE] New slash command `/start <id>` to @ reactors about gaming time
