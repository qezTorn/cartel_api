Current project status

Completed selections include tests that can be run with:
cargo test -- [test name] --nocapture
the tests since they're polling the real api dont have common asserts since stuff like cooldowns are always changing
so the tests instead pull deseralize and print the data multiple keys are assigned in the .env file\
qez_key=[primary test key]\
velthir_key=[secondary test key]\

Finished endpoints/features

[x] Client    - holds the api key and the http client\
[x] Basic     - returns and deseralizes the /api/type=basic\
[x] Advanced  - /api/type=advanced\
[x] Advanced  - /api/type=advanced\
[x] Battlestats - /api/type=battlestats\
[x] Cooldowns   - /api/type=cooldowns\
[x] Status      - /api/type=status\
[x] Cartel       - This should be easy havent done it yet though\
[x] Inventory    - Should be easy\
[x] Advanced Inv - Should be easy\

Pending 
[p] Range   - Partial working need to consider validation and some edge cases\
[p] Attacks - Works provided you dont use the range incorrectly\
[ ] Events  - Not yet implemented pending solving above problems\
[ ] Graph   - Same as above not implemented yet\
[p] Chat    - They added chat api to the game\
     ^- This is as implemented as is knowable since the documentation for the api is down\
[ ] Item    - This got added 10/7/2024 and needs quite a bit of work.

