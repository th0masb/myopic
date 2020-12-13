Idea here is to take .pgn files from pgnmentor.com containing master games
with a particular opening then take the first n moves and look at all unique
positions and associate the set of moves which were made. We write a program
to do this and store the results in a mongodb instance running locally and
then a separate program to transfer this data from mongodb to dynamodb.

