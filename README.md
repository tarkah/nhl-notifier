# NHL Notifier

Get live game updates via SMS for your favorite NHL team

- [NHL Notifier](#nhl-notifier)
  - [Install](#install)
  - [CLI Output](#cli-output)
  - [Automatically start with timer](#automatically-start-with-timer)
  - [Config](#config)


## Install

- Clone repo
- `cargo install --path .` will install binary to `~/.cargo/bin`

## CLI Output

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

```
[2019-11-24T22:00:18Z INFO  nhl_notifier::config] Using config file: "./config.yml"
[2019-11-24T22:00:18Z INFO  nhl_notifier::game] There are 13 games today, Tuesday, 19 November, 2019
[2019-11-24T22:00:18Z INFO  nhl_notifier::game] There is 1 team with an active subscription
[2019-11-24T22:00:18Z INFO  nhl_notifier::game] There is 1 subscribable game
[2019-11-24T22:00:18Z INFO  nhl_notifier::game] Running Game(2019020329) - Vegas Golden Knights vs. Toronto Maple Leafs @ Tue, 19 Nov 2019 19:00:00 -0800...
[2019-11-24T22:00:18Z INFO  nhl_notifier::game] Game(2019020329) - Got preview: Vegas goalie Fleury eyes NHL win No. 450; Toronto 0-4-1 in past five
[2019-11-24T22:00:19Z INFO  nhl_notifier::game] Game(2019020329) - Notification sent for: +15555555555
[2019-11-24T22:00:21Z INFO  nhl_notifier::game] Game(2019020329) - Golden Knights score, 11:53 2nd, VGK 1 - TOR 0, Cody Glass (4) Wrist Shot, assists: Max Pacioretty (13), Nate Schmidt (6)
[2019-11-24T22:00:21Z INFO  nhl_notifier::game] Game(2019020329) - Notification sent for: +15555555555
[2019-11-24T22:00:21Z INFO  nhl_notifier::game] Game(2019020329) - Maple Leafs score, 12:34 3rd, VGK 1 - TOR 1, Jason Spezza (3) Wrist Shot, assists: Ilya Mikheyev (8)
[2019-11-24T22:00:21Z INFO  nhl_notifier::game] Game(2019020329) - Notification sent for: +15555555555
[2019-11-24T22:00:22Z INFO  nhl_notifier::game] Game(2019020329) - Golden Knights score, 11:53 3rd, VGK 2 - TOR 1, Tomas Nosek (4) Backhand, assists: none
[2019-11-24T22:00:22Z INFO  nhl_notifier::game] Game(2019020329) - Notification sent for: +15555555555
[2019-11-24T22:00:22Z INFO  nhl_notifier::game] Game(2019020329) - Golden Knights score, 09:38 3rd, VGK 3 - TOR 1, Mark Stone (10) Wrist Shot, assists: Cody Glass (5), Shea Theodore (9)
[2019-11-24T22:00:22Z INFO  nhl_notifier::game] Game(2019020329) - Notification sent for: +15555555555
[2019-11-24T22:00:22Z INFO  nhl_notifier::game] Game(2019020329) - Maple Leafs score, 07:13 3rd, VGK 3 - TOR 2, Zach Hyman (1) Snap Shot, assists: Jason Spezza (4), Tyson Barrie (7)
[2019-11-24T22:00:22Z INFO  nhl_notifier::game] Game(2019020329) - Notification sent for: +15555555555
[2019-11-24T22:00:23Z INFO  nhl_notifier::game] Game(2019020329) - Golden Knights score, 00:21 3rd, VGK 4 - TOR 2, Cody Eakin (2) Wrist Shot, assists: Max Pacioretty (14), Jonathan Marchessault (11)
[2019-11-24T22:00:23Z INFO  nhl_notifier::game] Game(2019020329) - Notification sent for: +15555555555
[2019-11-24T22:00:33Z INFO  nhl_notifier::game] Game(2019020329) - Golden Knights win. Final score: VGK 4 - TOR 2
[2019-11-24T22:00:33Z INFO  nhl_notifier::game] Game(2019020329) - Notification sent for: +15555555555
```

## Automatically start with timer

- Copy `.service` & `.timer` files to `/etc/systemd/system/`
- Place configuration file at `~/.config/nhl-notifier/config.yml` (see below or run `nhl-notifier generate` to create skeleton file)
- Update `ExecStart` in service file if binary / config placed elsewhere
- Update `OnCalendar` in timer file to change when program is started each day
- Run `systemctl enable nhl-notifier@youruser.timer` to enable timer

## Config

```yaml
# Populate config with your own values

# The earliest time a notification will be sent that your team plays today: HH:MM:SS
earliest_notification_time: 10:00:00

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