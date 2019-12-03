# NHL Notifier
[![Build Status](https://dev.azure.com/tarkah/nhl-notifier/_apis/build/status/tarkah.nhl-notifier?branchName=master)](https://dev.azure.com/tarkah/nhl-notifier/_build/latest?definitionId=1&branchName=master)

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
[2019-12-02T18:00:10Z INFO  nhl_notifier::config] Using config file: "/home/tarkah/.config/nhl-notifier/config.yml"
[2019-12-02T18:00:10Z INFO  nhl_notifier::game] There are 5 games today, Monday, 2 December, 2019
[2019-12-02T18:00:10Z INFO  nhl_notifier::game] There is 1 team with an active subscription
[2019-12-02T18:00:10Z INFO  nhl_notifier::game] There is 1 subscribable game
[2019-12-02T18:00:10Z INFO  nhl_notifier::game] Running Game(2019020420) - New York Rangers vs. Vegas Golden Knights @ Mon, 02 Dec 2019 16:00:00 -0800...
[2019-12-02T19:00:12Z INFO  nhl_notifier::game] Game(2019020420) - Got preview: New York eyes point in six straight games; Vegas has won past two
[2019-12-02T19:00:12Z INFO  nhl_notifier::game] Game(2019020420) - Notification sent for: +15555555555
[2019-12-03T00:20:20Z INFO  nhl_notifier::game] Game(2019020420) - Golden Knights score, 18:26 1st, NYR 0 - VGK 1, Alex Tuch (3) Wrist Shot, assists: Jonathan Marchessault (13), Brayden McNabb (4)
[2019-12-03T00:20:20Z INFO  nhl_notifier::game] Game(2019020420) - Notification sent for: +15555555555
[2019-12-03T00:20:20Z INFO  nhl_notifier::game] Game(2019020420) - Highlight, Alex Tuch scores against New York Rangers to make it 1-0, https://hlslive-wsczoominwestus.med.nhl.com/publish/31827937-bee0-4c16-a10d-ceb1a392b03f.mp4
[2019-12-03T00:20:21Z INFO  nhl_notifier::game] Game(2019020420) - Notification sent for: +15555555555
[2019-12-03T00:22:11Z INFO  nhl_notifier::game] Game(2019020420) - Golden Knights score, 16:10 1st, NYR 0 - VGK 2, Alex Tuch (4) Tip-In, assists: none
[2019-12-03T00:22:12Z INFO  nhl_notifier::game] Game(2019020420) - Notification sent for: +15555555555
[2019-12-03T00:25:13Z INFO  nhl_notifier::game] Game(2019020420) - Highlight, Alex Tuch scores a power-play goal against New York Rangers to make it 2-0, https://hlslive-wsczoominwestus.med.nhl.com/publish/5b849070-336b-4003-bbe1-ca2a4f10dbe3.mp4
[2019-12-03T00:25:13Z INFO  nhl_notifier::game] Game(2019020420) - Notification sent for: +15555555555
[2019-12-03T01:13:29Z INFO  nhl_notifier::game] Game(2019020420) - Golden Knights score, 15:16 2nd, NYR 0 - VGK 3, Reilly Smith (12) Backhand, assists: none
[2019-12-03T01:13:30Z INFO  nhl_notifier::game] Game(2019020420) - Notification sent for: +15555555555
[2019-12-03T01:13:30Z INFO  nhl_notifier::game] Game(2019020420) - Highlight, Reilly Smith scores against New York Rangers to make it 3-0, https://hlslive-wsczoominwestus.med.nhl.com/publish/ae093aaf-f306-4754-97c4-f988e6dbecfa.mp4
[2019-12-03T01:13:30Z INFO  nhl_notifier::game] Game(2019020420) - Notification sent for: +15555555555
[2019-12-03T01:14:30Z INFO  nhl_notifier::game] Game(2019020420) - Golden Knights score, 13:40 2nd, NYR 0 - VGK 4, Max Pacioretty (10) Snap Shot, assists: none
[2019-12-03T01:14:31Z INFO  nhl_notifier::game] Game(2019020420) - Notification sent for: +15555555555
[2019-12-03T01:18:32Z INFO  nhl_notifier::game] Game(2019020420) - Highlight, Max Pacioretty scores a power-play goal against New York Rangers to make it 4-0, https://hlslive-wsczoominwestus.med.nhl.com/publish/2aa39d30-f528-4832-908f-af163ffa3ace.mp4
[2019-12-03T01:18:33Z INFO  nhl_notifier::game] Game(2019020420) - Notification sent for: +15555555555
[2019-12-03T01:30:47Z INFO  nhl_notifier::game] Game(2019020420) - Rangers score, 04:45 2nd, NYR 1 - VGK 4, Brendan Lemieux (4) Tip-In, assists: Jacob Trouba (11), Mika Zibanejad (9)
[2019-12-03T01:30:48Z INFO  nhl_notifier::game] Game(2019020420) - Notification sent for: +15555555555
[2019-12-03T01:34:49Z INFO  nhl_notifier::game] Game(2019020420) - Highlight, Brendan Lemieux scores against Vegas Golden Knights to make it 4-1, https://hlslive-wsczoominwestus.med.nhl.com/publish/f1e4d96e-5b6c-411c-82bb-f95dd2a2a6dd.mp4
[2019-12-03T01:34:50Z INFO  nhl_notifier::game] Game(2019020420) - Notification sent for: +15555555555
[2019-12-03T02:31:19Z INFO  nhl_notifier::game] Game(2019020420) - Golden Knights win. Final score: NYR 1 - VGK 4
[2019-12-03T02:31:19Z INFO  nhl_notifier::game] Game(2019020420) - Notification sent for: +15555555555
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
