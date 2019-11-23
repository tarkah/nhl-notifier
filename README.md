# NHL Notifier

Get live game updates via SMS for your favorite NHL team

```
nhl-notifier 0.1.0
tarkah <admin@tarkah.dev>
Get live game updates via SMS for your favorite NHL team.

USAGE:
    nhl-notifier <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    generate    Generate an empty config.yml file to the current directory
    help        Prints this message or the help of the given subcommand(s)
    run         Run the program
```

## Config

```yaml
# Populate config with your own values

# The earliest time a notification will be sent that your team plays today: HH:MM:SS
earliest_notification_time: 07:00:00

# Subscriptions are declared as a team id, then a list of phone numbers
# that should receive notifications for that team.
#
# Team id's can be referenced from https://statsapi.web.nhl.com/api/v1/teams
#
# Phone numbers must be stored as "+15555555"
subscriptions:
  - team: 1
    numbers:
      - "+15555555"
      - "+15555678"
  - team: 54
    numbers:
      - "+15557890"
```