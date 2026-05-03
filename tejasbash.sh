#!/bin/bash

SESSION_TOKEN="53616c7465645f5f3566aa8b4a799a5a2707a876dfec42cf8bc51759854edbe555772eb74842d945f5bf482c013c47614c2b216958d69960e3b3490001c3836c"

fetch_input() {
  local year=2015
  local day=$1
  local url="https://adventofcode.com/${year}/day/${day}/input"

  curl -s -H "Cookie: session=${SESSION_TOKEN}" "$url" -o "input.txt"

  if [[ $? -eq 0 ]]; then
    echo "done input.txt"
  else
    echo "fail for $day"
    exit 1
  fi
}

if [[ -z $1 ]]; then
  echo "Usage: $0 <day>"
  exit 1
fi

fetch_input $1
