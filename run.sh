#!/bin/bash

(
	cd server || exit 1
	if [[ ! -d "node_modules" ]]; then
		echo "Installing server dependencies..."
		yarn
	fi
	npm start &
)

(
	cd client || exit 1
	cargo run
)
