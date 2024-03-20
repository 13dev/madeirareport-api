#!/bin/bash

# Process name
PROCESS_NAME="port-forward"

# Check if the process is already running
if pgrep -f "$PROCESS_NAME" >/dev/null; then
    echo "Process $PROCESS_NAME is already running."
    exit 1
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
    exit 1
fi

# Enter the main loop
while true; do
    # Do whatever you want inside the loop
    sleep 5s
done