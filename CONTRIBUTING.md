# Add an instruction

The interface parses instruction data and account lists for the auction program.
The API defines the instruction numbers and data layouts.

1. Add and export the interface parser.
2. Enforce the required account signatures, owners, writable permissions, and fixed addresses before returning the parsed accounts.
3. Keep rules about stored authorities, deadlines, and payments in the program processor.
4. Add a passing case to the complete `AuctionInstruction` match in `tests/instructions.rs`.
5. Add rejection cases for each requirement and each supported optional account or data format.
6. Run `cargo +1.98.1 test --locked --all-targets --all-features` from this repository.

The match has no catch-all arm. A new API instruction fails compilation until it has a parser and a test case.
List an instruction as disabled only when the program rejects it in both production and development builds.
Keep production-disabled policy initialization in the tests because the development build accepts it.

# Coordinate API changes

API pull requests run these tests with their proposed API code.
The API workflow uses the interface `main` branch.

1. Pin the proposed API commit in the interface pull request when both repositories need changes.
2. Merge the interface pull request after `Interface tests` passes.
3. Rerun `Interface compatibility` on the API pull request.
4. Merge the API pull request after that check passes.

Changes to the interface take effect in the auction program after its dependency revision changes and its binary is deployed.
