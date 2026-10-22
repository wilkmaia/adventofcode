#!/usr/bin/env bash
#
# Fetch an Advent of Code puzzle input and write it verbatim to stdout.
#
# Usage:
#   AOC_SESSION=<session-cookie> ./scripts/get-input.sh YEAR DAY > path/to/input.txt
#
# AOC_SESSION must contain the value of the "session" cookie from
# adventofcode.com (not the whole cookie header).

set -euo pipefail

usage() {
    cat >&2 <<EOF
Usage: $(basename "$0") YEAR DAY

Fetch the Advent of Code input for YEAR and DAY and write it to stdout.

Arguments:
  YEAR  Four-digit puzzle year (2015 or later)
  DAY   Puzzle day (1-25)

Environment:
  AOC_SESSION  Value of the "session" cookie from adventofcode.com (required)

Example:
  AOC_SESSION=... $(basename "$0") 2025 1 > 2025/day01/input.txt
EOF
}

die() {
    printf 'error: %s\n' "$*" >&2
    exit 1
}

if (( $# != 2 )); then
    usage
    exit 2
fi

year_arg=$1
day_arg=$2

if [[ ! $year_arg =~ ^[0-9]{4}$ ]]; then
    die "YEAR must be a four-digit number (got '$year_arg')"
fi

if [[ ! $day_arg =~ ^[0-9]{1,2}$ ]]; then
    die "DAY must be a number between 1 and 25 (got '$day_arg')"
fi

# Normalize leading zeroes (e.g. 01 -> 1) for the URL and validation.
year=$((10#$year_arg))
day=$((10#$day_arg))

if (( year < 2015 )); then
    die "YEAR must be 2015 or later (got '$year_arg')"
fi

if (( day < 1 || day > 25 )); then
    die "DAY must be between 1 and 25 (got '$day_arg')"
fi

if [[ -z ${AOC_SESSION:-} ]]; then
    die "AOC_SESSION is not set. Export the value of the 'session' cookie from adventofcode.com."
fi

if ! command -v curl >/dev/null 2>&1; then
    die "curl is required but was not found in PATH."
fi

url="https://adventofcode.com/${year}/day/${day}/input"
referer="https://adventofcode.com/${year}/day/${day}"
user_agent='Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/155.0.0.0 Safari/537.36'

tmp_file=$(mktemp "${TMPDIR:-/tmp}/aoc-input.XXXXXX")
cleanup() {
    rm -f -- "$tmp_file"
}
trap cleanup EXIT

# Keep the response body in a temporary file so that errors are not written to
# stdout. The body is only copied to stdout on a successful (HTTP 200) request.
if ! http_code=$(curl \
    --silent \
    --show-error \
    --output "$tmp_file" \
    --write-out '%{http_code}' \
    --cookie "session=${AOC_SESSION}" \
    --user-agent "$user_agent" \
    --header 'Accept: text/plain' \
    --referer "$referer" \
    --url "$url"); then
    die "failed to download input from $url"
fi

case $http_code in
    200)
        # cat preserves the response body exactly, including (or excluding) any
        # trailing newline.
        cat -- "$tmp_file"
        ;;
    301|302|303|307|308)
        die "authentication failed (HTTP $http_code); check that AOC_SESSION is valid and has not expired."
        ;;
    401|403)
        die "access denied (HTTP $http_code); check that AOC_SESSION is valid."
        ;;
    404)
        die "input for $year day $day is not available (HTTP 404); the puzzle may not be unlocked yet."
        ;;
    *)
        die "request for $year day $day failed with HTTP status $http_code."
        ;;
esac
