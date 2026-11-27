## Tiny Blockchain Node — Requirements

1. Build a single-process Rust program containing a `Node`, `Mempool`, and `Blockchain`.
2. A `Transaction` has a unique string `id`, `fee: u64`, and `weight: u64`.
3. Reject transactions with an empty ID, zero weight, or an ID already in the mempool.
4. The mempool has a configurable maximum total weight and rejects transactions that exceed it.
5. The mempool supports inserting, finding, removing, counting, and reporting total pending weight.
6. Callers submit transactions through `Node::submit_transaction`; the mempool remains private.
7. A `Block` contains its height, previous block hash, selected transactions, and its own hash.
8. The blockchain stores blocks in a `Vec<Block>`; blocks link logically through `previous_hash`.
9. Initialize every blockchain with one deterministic genesis block at height zero.
10. The blockchain exposes its current tip and validates every block before appending it.
11. A valid next block has `height == tip.height + 1` and `previous_hash == tip.hash`.
12. `Node::produce_block(max_weight)` selects pending transactions without first removing them.
13. Selection priority is fee-per-weight descending, then fee descending, then ID ascending.
14. Transactions that do not fit are skipped while selection continues through remaining candidates.
15. The selected transactions' combined weight must never exceed the requested block weight.
16. Produce, validate, and append the block before removing its transactions from the mempool.
17. If block creation or validation fails, the blockchain and mempool must remain unchanged.
18. Use typed `Result` errors and write tests for rejection, ordering, capacity, linking, and rollback.
19. Stop after an in-memory demo creates three blocks; do not add networking, persistence, mining, signatures, or consensus.