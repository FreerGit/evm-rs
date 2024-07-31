# evm-rs
Got any of those opcodes??

[![CI](https://github.com/freergit/evm-rs/actions/workflows/ci.yml/badge.svg)][gh-ci]
[![License](https://img.shields.io/badge/License-MIT-orange.svg)][mit-license]

[gh-ci]: https://github.com/freergit/evm-rs/actions/workflows/ci.yml
[mit-license]: https://opensource.org/license/mit/


## Notes
https://ethereum.github.io/yellowpaper/paper.pdf

Important Block numbers:
| Version           | Block    |
| ----------------- | -------  |
| Homestead         | 1150000  |  
| TangerineWhistle  | 2463000  |  
| SpuriousDragon    | 2675000  |  
| Byzantium         | 4370000  |  
| Constantinople    | 7280000  |  
| Petersburg        | 7280000  |
| Istanbul          | 9069000  |
| MuirGlacier       | 9200000  |
| Berlin            | 12244000 |
| London            | 12965000 |
| ArrowGlacier      | 13773000 |
| GrayGlacier       | 15050000 |
| Paris             | 15537394 |
| Shanghai          | 17034870 |

Adress = 160 bit

state database =  Merkle Patricia tree</r>

account:

if the codeHash field is the keccak-256 hash of the empty tring the node represents a non-contrat.

A account is empty if it has no code, zero nonce and zero balance.

Transactions:

The sender of a transaction cannot be a contrat

The transaction type
