#!/bin/bash

APP_NAME="Rusty Lander"
mv "${APP_NAME}.app" build/mac/
create-dmg --volname "${APP_NAME}" --window-size 800 400 --icon-size 128 --icon "${APP_NAME}.app" 200 200 --hide-extension "${APP_NAME}.app" --app-drop-link 600 200 "rusty_lander.dmg" "build/mac/"
