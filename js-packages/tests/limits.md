# Limits

The Limits feature in Unique Network allows setting specific constraints on token ownership and sponsorship durations to maintain control and balance within the network.

## Key Functionalities

### Setting Collection Limits

- **Action**: Set limits for a collection.
- **Who Can Perform**: Collection owners.
- **Details**: Collection owners can specify limits on the number of tokens an address can own and the sponsorship duration. These limits help manage resource allocation and prevent excessive accumulation of tokens by a single address.

### Token Ownership Limits

- **Action**: Set and enforce limits on the number of tokens per address.
- **Who Can Perform**: Collection owners.
- **Details**: Limits can be set on how many tokens an address can own within a collection. If a collection's limit is higher than the chain's limit, the chain's limit is enforced.

### Sponsorship Timeout Limits

- **Action**: Set and enforce limits on sponsorship duration.
- **Who Can Perform**: Collection owners.
- **Details**: Limits can be set on how long a sponsorship lasts. If a collection's sponsorship timeout is longer than the chain's limit, the chain's limit is enforced. Setting a sponsorship timeout to zero means no limit is applied, allowing for indefinite sponsorships.

### Account Token Ownership Limit

- **Action**: Limit the number of tokens an address can own.
- **Who Can Perform**: Collection owners.
- **Details**: Ensures that no single address can accumulate an excessive number of tokens within a collection.

### Sponsored Data Size

- **Action**: Limit the size of data that can be sponsored.
- **Who Can Perform**: Collection owners.
- **Details**: Controls the maximum size of data for which sponsorship can cover the transaction fees.

### Sponsored Data Rate Limit

- **Action**: Limit the rate of sponsored data transactions.
- **Who Can Perform**: Collection owners.
- **Details**: Ensures that sponsored data transactions do not exceed a specified rate, preventing abuse of sponsorship.

### Token Limit

- **Action**: Set a maximum limit on the total number of tokens in a collection.
- **Who Can Perform**: Collection owners.
- **Details**: Prevents the creation of more tokens than the specified limit within a collection.

### Sponsor Transfer Timeout

- **Action**: Set a timeout for sponsored transfers.
- **Who Can Perform**: Collection owners.
- **Details**: Limits the duration for which sponsorship can cover transfer transactions, ensuring timely usage of sponsorships.

### Sponsor Approve Timeout

- **Action**: Set a timeout for sponsored approvals.
- **Who Can Perform**: Collection owners.
- **Details**: Limits the duration for which sponsorship can cover approval transactions, encouraging efficient token management.

### Owner Can Transfer

- **Action**: Allow or restrict the owner's ability to transfer tokens.
- **Who Can Perform**: Collection owners.
- **Details**: Provides flexibility to enable or disable the owner's ability to transfer tokens within a collection.

### Owner Can Destroy

- **Action**: Allow or restrict the owner's ability to destroy tokens.
- **Who Can Perform**: Collection owners.
- **Details**: Allows collection owners to specify whether token destruction is permitted, enhancing control over token lifecycle.

### Transfers Enabled

- **Action**: Enable or disable transfers within a collection.
- **Who Can Perform**: Collection owners.
- **Details**: Provides control over whether tokens in the collection can be transferred, ensuring that transferability aligns with the collection's purpose and policies.

For further details and specific implementation examples, refer to the [limits.test.ts file](https://github.com/UniqueNetwork/unique-chain/blob/master/js-packages/tests/limits.test.ts).
