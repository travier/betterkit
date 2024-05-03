# Alternative to polkit based on DBus and systemd-run

## Design

- Define a set of actions (commands) that will be executed via [systemd-run].
- Define a set of rules that grant access to those actions to users based on
  various conditions (logged-in locally, system administrator, etc.).
- Run as a privileged system daemon that will execture the action for users if
  they match the rules.

## Security

- Limited parsing of untrusted data
- Declarative and easy to understand configuration syntax for actions and rules
- Actions are run fully un-privileged and confined by default: privileged
  access is opt-in
- No SUID binary
- Clear logging of which user requested which action and which rules enabled it
- Written in pure safe Rust

## Example

```
$ cat /usr/lib/betterkit/actions/rpm-ostree.conf
[Action]
Id=org.coreos.rpm-ostree.rollback
Description=Roolback to previous deployment with rpm-ostree
Command=rpm-ostree rollback
User=root
Access=dbus-system

$ cat /usr/lib/betterkit/rules/rpm-ostree.conf
[Rule]
Action=org.coreos.rpm-ostree.rollback
Group=wheel
Result=Accept

[Rule]
Action=org.coreos.rpm-ostree.rollback
Active=true
Result=Ask
```

[systemd-run]: https://www.freedesktop.org/software/systemd/man/systemd-run.html

## Current status

Currently only working with the session bus:

```
$ cargo run -- -v --user
[INFO  deadend] Received: Signal NameAcquired from org.freedesktop.DBus
[INFO  deadend] Received: Signal NameAcquired from org.freedesktop.DBus
[INFO  deadend] Writing MOTD with reason: Test
[INFO  deadend] Successfully wrote MOTD
```

```
$ busctl --user call org.betterkit /org/betterkit/betterkit1 org.betterkit.betterkit1 Run as 2 "ls" "-alh"
```
