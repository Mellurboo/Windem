
# WINDEM 🎥
Windem is an open source osint wrapper to gather usernames across various social media platforms quickly, based off and forked from Ava's Fyndem project [@YourAva](https://github.com/YourAva/fyndem) for Linux adapted to work for windows!

## Installation 🗒️
Windem requires [Nodejs](https://nodejs.org/en/download/package-manager) and [Rust](https://www.rust-lang.org/tools/install) to compile. unlike Lunix there is no standard terminal install command (yet) so you will have to do this yourself, sorry <3

### Installing for USERS
[Windem](https://github.com/Mellurboo/Windem/releases) On windows you can just run this.

### Installing for DEVELOPERS
Ensure all dependencies are installed, refer back to the installation paragraph for download links, then simply run Build.bat or `cargo run [targetusername]`. its as simple as that

## Can I help in any way?
adding a platform to windem gets easier and easier all the time, just add it to the list in targets.rs and call it in main.rs making sure you use the correct function call (html or title) failing to do so could result in false negitives. then go ahead and submit your pull request!

## Troubleshooting
For some extra help diagnosing, run windem with -d to enter debug mode.
- **EVERYTHING RETURNS AS FOUND/NO HTML RETURNED?** (Developer) --- Make sure you're running the project inside of /src, or else you will encounter an error with the JS where no HTML is returned, therefore making everything return as found.
- **RUNNING WINDEM IN MY CONSOLE DOES NOTHING** (User) --- You've installed Windem wrong. Re-read the installation process carefully, and make sure you reset your terminal after finishing all the steps.

*we will try to keep this up to date with common user issues <3*
if you need any help please contact me on discord, add me on "Mellurboo" and I'll try to help you!

*DO NOT CONTACT AVA FOR ISSUES ON WINDEM, SHE MAY NOT BE ABLE TO HELP YOU!*
