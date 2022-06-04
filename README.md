# haveibeenpwned

haveibeenpwned is a command-line application that uses [HaveIBeenPwned](https://haveibeenpwned.com/) service and can create and use Binary Fuse filter (which is smaller than Bloom filter or Cuckoo filter for the same false positive ratio) for efficient query at cost of false positives. This is still WIP.

## Roadmap
- [x] Interactively check compromised password using HIBP API (requires internet)
- [x] Download password file using HaveIBeenPwned queries. This can be more up to date than downloading passwords directly from HaveIBeenPwned website. According to Troy Hunt, passwords from ingestions are not included since a password version release in the download version. However, querying the password does contain the ingested passwords
- [ ] Interactively check compromised password using filter
- [x] Create filter (of 3 sizes) that allows you to query offline while consuming a fraction of the space. Does require existing downloaded password file (either from website or by using this tool) to create. However, downloadable filter files will eventually be provided.
- [ ] Check list of passwords in a file (using a filter) to see how many are compromised

## Compatibility
As haveibeenpwned is still in alpha, the design of the filter is not final.  Therefore, filter file compatibility will **not** be maintained between versions until haveibeenpwned is no longer an alpha version.