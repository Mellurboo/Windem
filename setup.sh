#!/usr/bin/env bash

RED="\e[31m"
GREEN="\e[32m"
PRIMARY="\e[1;32m"
ERROR="\e[3;31m" #unused but its here if anyone ever needs it xx
ENDCOLOR="\e[0m"

echo " "
echo -e "${RED}FYNDEM By AvaLikesBread (https://github.com/YourAva)${ENDCOLOR}"
echo -e "${GREEN}Installing npm and packages${ENDCOLOR}"
echo " "

sudo apt install npm -y

echo -e "${PRIMARY}Installing npm packages${ENDCOLOR}"

# Add to the list as needed, this isnt a great soloution.
NPM_PACKAGES=("puppeteer")

for package in "${NPM_PACKAGES[@]}"; do
  echo -e "${GREEN}Installing ${package}${ENDCOLOR}"
  npm install "$package" -y
done

echo -e "${PRIMARY}Installing Chromium${ENDCOLOR}"

sudo snap install chromium

echo " "
echo -e "${RED}Install helper script by Mellurboo"
