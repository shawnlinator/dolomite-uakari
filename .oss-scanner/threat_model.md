# Threat model

## What this project does and where untrusted input enters
A backup server (C, Proxmox Backup Server). Untrusted input: the REST API (JSON), backup/dedup
data streams, and authentication (including the daemon's client certificates). Parsing of
untrusted data streams and the REST layer are the main surfaces.

## Components that matter most / least
- most: the REST/JSON layer, the backup data-stream parsing, and the auth/token handling.
- less: the web UI, docs.

## How to exercise it
Exercise the REST API with crafted JSON; feed crafted backup/dedup data streams; test auth.

## How you rate severity
- Memory-safety bugs reachable via the API or data streams: critical (RCE). Auth bypass:
  critical. Logic bugs: high. DoS: medium.

## Anything to leave alone
- Do not report issues only reachable as the datastore admin as high severity.
