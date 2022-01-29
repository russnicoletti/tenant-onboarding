# Overview

This is a quick program to make onboarding tenants even faster.  A single command will get the majority setup for us.

The program will do a few things for the user.

1. Create a github team (-t)
2. Create a github repo from the sample repo using the supplied name (-r)
3. Setup the github webooks
4. Add the devstudio and tenant github teams to the repo
5. Update the basic files to have the proper names
6. Generate the necessary DB snippets needed (-d)
7. (delete coming later...)

# How to use

Either update a config file or pass in data through the CLI

## Dry run

Default mode is a dry run that will print out info about what happened to a log.  This way you can confirm you did the right thing before you really create a repo.

To actually execute for REAL, you'll need the extra flag.

to -r --execute

## Using the config file

YAML sucks...

config.json will hold specific info that will be used in the repo & files.

TO will look for it at the same level.

## Logs

Logs will be written out to to-TIMESTAMP.log

## DB snippets

To make life easier TO will generate the DB snippets you neee to use.

They will be stored in the subdirectory: dbscripts

Don't worry, we'll make the folder if you don't have one.

For now it will overwrite contents if you run it more than once.

## Example commands

### Get help (I'm not doing "Get Help")

to -h

### Full shabang

to -trdv

### Only DB files

to -d