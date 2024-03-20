#!/bin/bash

# Process name
PROCESS_NAME="port-forward"

# Check if another instance of this script is already running
LOCK_FILE="/tmp/port-forward.lock"
if [ -f "$LOCK_FILE" ]; then
    echo "Another instance of the script is already running."
    exit 0
else
    touch "$LOCK_FILE"
fi

# Start the port-forward process
kubectl port-forward service/postgres-service 5432:5432 &
FORWARD_PID=$!  # Capture the PID of the port-forward process

# Wait for a brief moment to allow the process to start
sleep 1s

# Check if the port-forward process is still running
if ps -p $FORWARD_PID >/dev/null; then
    echo "Port-forward process is now running."
else
    echo "Failed to start port-forward process."
    rm "$LOCK_FILE"
    exit 0
fi

# Enter the main loop
while true; do
    sleep 5s
done

# Remove the lock file when the script exits
trap 'rm "$LOCK_FILE"' EXIT