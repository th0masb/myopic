# myopic chess engine

---

### introduction

This repository contains a mixture of libraries and applications which combine
to form an amateur (not very good, myopic etc...)  chess engine which is
playable by anyone around the world at any time via the best free website for
playing chess [lichess.org](lichess.org). All of the application code is written
in [Rust](rust-lang.org). The infrastructure is provided by AWS and provisioned 
programmatically using the typescript flavour of their [cloud development kit](https://aws.amazon.com/cdk/).

The engine deployment is mostly serverless and uses a couple of lambda functions
for computation alongside a dynamodb table for opening moves. However the Lichess
API model is pull based and requires a process polling an event stream constantly
to detect and respond to challenges, so this is running on a tiny ECS cluster.

---

### challenging the bot

First the fun bit! How do you play against the bot? Well firstly you need an
account on lichess.org which is completely free and just requires an email 
address. Then follow the following steps starting from the home screen:

![Challenge how-to](https://th0masb-public-assets.s3.eu-west-2.amazonaws.com/myopic-challenge-how-to.gif)

Some things to note about the parameters of the game:

 - Only the "Standard" variant is supported 
 - You can only play "Real time" games against the bot, i.e. games with a
   clock, to constrain the use of AWS resources to keep within the free tier
 - The minutes per side supported is 1-10 inclusive and the increment supported
   is 0-5 inclusive 
