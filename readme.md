
# FYNDEM 📹
Fyndem is a piece of linux command line software that is used to gather open source intelligence about an individual over mutliple social media platforms, by streamlining the process of finding all registered accounts under a username we can quickly gather a large amount of data on someone, to be used for good or bad.
## How does it work?
Fyndem uses a mix of rust and node.js to call data from websites with the username given to see if that account exists, different websites call for different ways of gathering the data to see if the account is real, so, fyndem comes with a few ways to do so...
- Send a general request to the website and scan that HTML for data that may signify it's not an existing account (such as the title saying '404 error'. -> *This comes with the issue of adapting to JS, most websites are built on it (or TS) nowadays so important data that tells us if the account is there may not exists... So in comes the next method)*
- Send a request to a website using a node.js package called puppeteer to simulate running the JS to advance the webpage into what a normal user would see, then scanning THAT html responce for any data that may come back.

## Installation 🗒️

### Installing for USERS
Firstly, download [Fynder's most recent version](https://github.com/YourAva/Fyndem/releases)'s .zip, and unpack it. Once that's done, make sure you're in the directory ``fyndem-root`` is in (so you can see the unpacked folder) and run these commands.
``mv fyndem-root ~/.local/bin``
``mv ~/.local/bin/fyndem-root/fyndem ~/.local/bin``
``sudo snap insall chromium``
now, when you enter a command line interface, and run ``fyndem``, it should run fyndem. Good luck, and happy hacking!

### Installing for DEVELOPERS
If you want to get the straight download of the project to work, it's actually extremely simple thanks to the help of [Mellurboo](https://github.com/Mellurboo), go ahead and clone the repository and run setup.sh, once done, all dependencies should work. Enter the SRC of the cloned repo and then run ``cargo run [username]``, **INSIDE OF SRC, OR ELSE IT WON'T WORK** (It's a bug, will be fixed soon.)

## Can I help in any way?
Absolutely. Adding more and more social media platforms to Fyndem is ***no easy task***, it will take hours of work and most likely even more addition to the backend to make it adaptable to the differences of each website, so, if you want to go ahead and clone the repository to make some changes, do so, and submit a pull request. Do keep in mind I'm a solo dev that can't spend every day checking github for a pull request, I urge you to DM on Discord if you have any questions or would like to notify me you've made one. ``avalikesbread_``

## Troubleshooting
For some extra help diagnosing, run fyndem with -d to enter debug mode.
- **EVERYTHING RETURNS AS FOUND/NO HTML RETURNED?** (Developer) --- Make sure you're running the project inside of /src, or else you will encounter an error with the JS where no HTML is returned, therefore making everything return as found.
- **RUNNING FYNDEM IN MY CONSOLE DOES NOTHING** (User) --- You've installed Fyndem wrong. Re-read the installation process carefully, and make sure you reset your terminal after finishing all the steps.

*we will try to keep this  up to date with common user issues <3*
