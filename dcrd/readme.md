A fully asynchronous, [futures](https://github.com/rust-lang/futures-rs)-enabled client
library for Rust based on [Dcrd](https://github.com/decred/dcrd) with true asynchronous programming.

## Ported Functionalities
- [ ] RPC Client
    - [ ] Websocket Notification
        - [x] Notify On Block Connected
        - [x] Notify On Block Disconnected
        - [x] Notify On Winning Tickets
        - [x] Notify On Work
        - [x] Notify On Transaction Accepted
        - [x] Notify On Transaction Accepted Verbose
        - [x] Notify On Stake Difficulty
        - [x] Notify On Reorganization
        - [ ] Notify On Relevant Transaction Spent
        - [ ] Notify On Spent And Missed Tickets
        - [ ] Notify On New Tickets

    - [ ] RPC Commands
        |               Command                |  Websocket supported |   HTTP Supported   |
        |:------------------------------------:|:--------------------:|:------------------:|
        | Add Node                             |                      |                    |
        | Create Raw SSTX                      |                      |                    |
        | Create Raw SSRTX                     |                      |                    |
        | Create Raw Transaction               |                      |                    |
        | Debug Level                          |                      |                    |
        | Decode Raw Transaction               |                      |                    |
        | Decode Script                        |                      |                    |
        | Estimate Fee                         |                      |                    |
        | Estimate Smart Fee                   |                      |                    |
        | Estimate Stake Diff                  |                      |                    |
        | Exists Address                       |                      |                    |
        | Exists Addresses                     |                      |                    |
        | Exists Expired Tickets               |                      |                    |
        | Exists Live Ticket                   |                      |                    |
        | Exists Live Tickets                  |                      |                    |
        | Exists Mempool Transactions          |                      |                    |
        | Exists Missed Tickets                |                      |                    |
        | Generate                             |                      |                    |
        | Get Added Node Info                  |                      |                    |
        | Get Best Block                       |                      |                    |
        | Get Best Block Hash                  |                      |                    |
        | Get Block                            |                      |                    |
        | Get Blockchain Info                  |  :white_check_mark:  | :white_check_mark: |
        | Get Block Count                      |  :white_check_mark:  | :white_check_mark: |
        | Get Block Hash                       |  :white_check_mark:  | :white_check_mark: |
        | Get Block Header                     |                      |                    |
        | Get Block Subsidy                    |                      |                    |
        | Get Block Verbose                    |  :white_check_mark:  | :white_check_mark: |
        | Get Cfilter V2                       |                      |                    |
        | Get Chain Tips                       |                      |                    |
        | Get Coin Supply                      |                      |                    |
        | Get Connection Count                 |                      |                    |
        | Get Current Network                  |                      |                    |
        | Get Difficulty                       |                      |                    |
        | Get Generate                         |                      |                    |
        | Get Hash Per Sec                     |                      |                    |
        | Get Headers                          |                      |                    |
        | Get Info                             |                      |                    |
        | Get Mempool Info                     |                      |                    |
        | Get Mining Info                      |                      |                    |
        | Get Network Totals                   |                      |                    |
        | Get Network Hash Per Sec             |                      |                    |
        | Get Network Info                     |                      |                    |
        | Get Peer Info                        |                      |                    |
        | Get Raw Mempool                      |                      |                    |
        | Get Raw Transaction                  |                      |                    |
        | Get Staked Difficulty                |                      |                    |
        | Get Staked Version Info              |                      |                    |
        | Get Staked Versions                  |                      |                    |
        | Get Ticket Pool Value                |                      |                    |
        | Get Treasury Balance                 |                      |                    |
        | Get Treasury Spend Votes             |                      |                    |
        | Get Transaction Output               |                      |                    |
        | Get Transaction Output Set Info      |                      |                    |
        | Get Vote Info                        |                      |                    |
        | Get Work                             |                      |                    |
        | Help                                 |                      |                    |
        | Invalidate Block                     |                      |                    |
        | Live Tickets                         |                      |                    |
        | Missed Tickets                       |                      |                    |
        | Node                                 |                      |                    |
        | Ping                                 |                      |                    |
        | Reconsider Block                     |                      |                    |
        | Regenerate Template                  |                      |                    |
        | Search Raw Transactions              |                      |                    |
        | Send Raw Transactions                |                      |                    |
        | Set Generate                         |                      |                    |
        | Stop                                 |                      |                    |
        | Submit Block                         |                      |                    |
        | Ticket Fee Info                      |                      |                    |
        | Tickets For Address                  |                      |                    |
        | Ticket Volume Weighted Average Price |                      |                    |
        | Transaction Fee Information          |                      |                    |
        | Validate Address                     |                      |                    |
        | Verify Chain                         |                      |                    |
        | Verify Message                       |                      |                    |
        | Version                              |                      |                    |

- [ ] Chaincfg
    - [x] Chain hash

- [ ] Dcr Utilities
    - [x] Get App Data Directory
    - [x] DCR Amount
