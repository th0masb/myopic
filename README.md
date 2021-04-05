# myopic chess engine

---

### game example

An example of the engine in action, the game can be viewed here: [lichess.org/kbX41mee](https://lichess.org/kbX41mee)
![Example game](https://th0masb-public-assets.s3.eu-west-2.amazonaws.com/myopic-example-game.gif)

---

### introduction

This repository contains a mixture of libraries and applications which combine
to form an amateur (not very good, myopic etc...)  chess engine which is
playable by anyone around the world at any time via the best free website for
playing chess [lichess.org](lichess.org). All of the application code is written in [Rust](rust-lang.org) which
has been an absolute joy to work with and has broadened my skills and outlook on
development in general. The infrastructure is provided by AWS and provisioned
programmatically using the typescript flavour of their [cloud development kit](https://aws.amazon.com/cdk/).
Documentation for the specifics of each component are separate and linked below
whereas the rest of this document provides more general information such as how
to actually challenge the bot!

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

---

### subproject documentation

Below is a list of links to the readmes for the notable subprojects.

| subproject | description |
| ---------- | ----------- |
| [core](https://github.com/th0masb/myopic/tree/master/core) | Core chess utility library |
| [board](https://github.com/th0masb/myopic/tree/master/board) | Chessboard library |
| [brain](https://github.com/th0masb/myopic/tree/master/brain) | Search and evaluation library |
| [move-lambda](https://github.com/th0masb/myopic/tree/master/move-lambda) | AWS Lambda application for computing the best move in a position |
| [game-lambda](https://github.com/th0masb/myopic/tree/master/game-lambda) | AWS Lambda application for playing a full game chess via the Lichess frontend | 
| [deploy](https://github.com/th0masb/myopic/tree/master/deploy) | AWS CDK application which provisions the necessary infrastructure |
| [pgn-extractor](https://github.com/th0masb/myopic/tree/master/openings/pgn-extractor) | Terminal application for extracting unique positions from .pgn files |
| [dynamodb-uploader](https://github.com/th0masb/myopic/tree/master/openings/dynamodb-uploader) | Terminal application which populates a DynamoDB table with chess openings |
| [event-stream](https://github.com/th0masb/myopic/tree/master/event-stream) | Terminal application which polls the Lichess api for challenges and triggers lambda functions |



