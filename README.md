# On-Chain Leaderboard — Solana Program

A traditional backend leaderboard system rebuilt as a Solana program in Rust using Anchor.

## Live Deployment (Devnet)
- **Program ID:** `A2DNJjEaxWNeuCHPFJqGQxm7Uqsztnub5uYgsdZD3QXM`
- **Explorer:** https://explorer.solana.com/address/A2DNJjEaxWNeuCHPFJqGQxm7Uqsztnub5uYgsdZD3QXM?cluster=devnet
- **Deploy Tx:** `3okjDyUKf47BCUXwTADCyJUjLJoVo6LBjcQMpcxHUrzkWv2xz7w29d2ioG6vV4rJMUgg64svTatfYGvgtnUy1NgQ`

## Devnet Transaction Links
- https://explorer.solana.com/tx/881hsu9M6hGVKsEx6KuusPytZRGpRb6Wgkqb6zP1eExmovGFr35cn6aqdaTP1pszNHCrgEuMcECqUvMh6mU3V2X?cluster=devnet
- https://explorer.solana.com/tx/2yPRnYuyazjwibEAE5kcCVPaXvK48CPtNs61wDbumLWMKWmsh1RXH9HQd43RfELP3sXEagvyWSC1ApforLrxHsdF?cluster=devnet
- https://explorer.solana.com/tx/55HgpKB4SbZ3HXu96xmnkyjtM5g2NKEt3zSBvK2ML5mzHvJYYCpk9JgXTX8gn7oBbHVX4pA2h2BBeMAqz6kvWbDA?cluster=devnet

---

## How This Works in Web2

A traditional leaderboard is a simple backend system:
- A database table with columns: `player_id`, `name`, `score`, `timestamp`
- A REST API to submit scores, update scores, and fetch rankings
- An admin role to reset or moderate the board
- Trust is centralised — the server owner controls the data

**Problems with Web2 approach:**
- Scores can be manipulated by the server admin
- No transparency — users must trust the operator
- Single point of failure
- No ownership — players don't control their own entries

---

## How This Works on Solana

On Solana, the leaderboard is a program (smart contract) with on-chain state:

### Account Model
| Account | Description |
|---|---|
| `LeaderboardState` | Stores the board name, authority (admin), and entry count |
| `PlayerEntry` | One PDA per player per leaderboard — stores name, score, timestamp |

### Instructions
| Instruction | Who can call | What it does |
|---|---|---|
| `initialize` | Anyone | Creates a new leaderboard |
| `submit_score` | Any player | Creates a new entry with initial score |
| `update_score` | Entry owner only | Updates score — only if new score is higher |
| `reset_leaderboard` | Authority only | Resets the entry count |

### Key Differences from Web2
- **Permissionless:** Anyone can submit a score — no account registration
- **Self-sovereign:** Players own their entries via PDA derived from their wallet
- **Tamper-proof:** Scores can only be updated by the player who owns the entry
- **Transparent:** All scores are publicly readable on-chain
- **Trustless:** No admin can modify another player's score

---

## Tradeoffs & Constraints

| | Web2 | Solana |
|---|---|---|
| Cost | Free reads/writes | Every write costs SOL (rent + tx fee) |
| Speed | Milliseconds | ~400ms per transaction |
| Sorting | SQL ORDER BY | Must sort client-side (no on-chain sort) |
| Storage | Unlimited rows | Each account has fixed size at creation |
| Flexibility | Easy to change schema | Schema changes require program upgrade |
| Access control | Role-based (JWT) | Cryptographic (wallet signatures) |

---

## Setup & Run

### Prerequisites
- Rust, Solana CLI, Anchor CLI
- Node.js v20+

### Build
```bash
anchor build
```

### Deploy to Devnet
```bash
anchor deploy --provider.cluster devnet
```

### Run Client
```bash
ANCHOR_PROVIDER_URL=https://api.devnet.solana.com \
ANCHOR_WALLET=~/.config/solana/id.json \
npx ts-node client.ts
```

### Expected Output
```
Authority: CXEgEPKodjFfqtN2UB6m2ojMfL9n18bD1S9JiVYQv5iB
1. Initializing leaderboard...
Leaderboard created: EQMmfCqTUHp66fynptTz3JQDoVkGXqv3Y2joNHuzzdH1
2. Submitting score...
Score submitted!
3. Leaderboard entry:
  Player: Player1
  Score: 9500
  Timestamp: 2026-03-16T06:40:40.000Z
4. Updating score to 12000...
Updated score: 12000
All tests passed!
```

---

## Program Architecture
```
leaderboard/
├── programs/leaderboard/src/lib.rs   # Main Rust program
├── client.ts                          # TypeScript test client
├── Anchor.toml                        # Anchor config
└── README.md                          # This file
```
