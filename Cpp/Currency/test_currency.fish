#!/usr/bin/env fish

if count $argv <1
    echo "Usage: " (status current-filename) "<total_attempts>"
    exit 1
end

set total_attempts $argv[1]

set success_count 0
set failure_count 0


for i in (seq $total_attempts)
    if ./test_currency >/dev/null 2>&1
        set success_count (math "$success_count + 1")
    else
        set failure_count (math "$failure_count + 1")
    end
end

echo "Total attempts: $total_attempts"
echo "Successful runs: $success_count"
echo "Failed runs: $failure_count"
