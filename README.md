# haveibeenpwned

haveibeenpwned is a command-line application that uses [have i been pwned?](https://haveibeenpwned.com/) service and can create and use Binary Fuse filter for efficient query at cost of false positives. This is still WIP.

# Roadmap
[x] Interactively check compromised password using HIBP API (requires internet)
[x] Download password file using HaveIBeenPwned queries. This can be more up to date than downloading passwords directly from HaveIBeenPwned website. According to Troy Hunt, passwords from ingestions are not included since a password version release in the download version. However, querying the password does contain the ingested passwords
[ ] Interactively check compromised password using existing password file
[ ] Interactively check compromised password using filter
[ ] Create filter that allows you to query offline while consuming a fraction of the space. Does require existing downloaded password file (either from website or by using this tool) to create. However, downloadable filter files will eventually be provided.
[ ] Check list of passwords in a file to see how many are compromised