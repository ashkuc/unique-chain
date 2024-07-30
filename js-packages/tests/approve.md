# Approve

The Approve feature in Unique Network allows token owners to delegate specific permissions to other addresses, enabling flexible and secure token management. Approved addresses can manage or transfer tokens on behalf of the token owner, which is essential for automated services or trusted delegates.

## Key Functionalities

### Approval of Tokens

- **Action**: Approve another address to manage or transfer tokens.
- **Who Can Perform**: Token owners.
- **Details**: Grants permission to another address to manage or transfer tokens on behalf of the owner. Useful for delegating token management to trusted third parties, such as marketplaces or automated services.

### Revoking Approval

- **Action**: Revoke previously granted approval.
- **Who Can Perform**: Token owners.
- **Details**: Ensures that a previously approved address can no longer manage or transfer tokens, maintaining security and control over the tokens.

### Checking Approval Status

- **Action**: Verify if an address has approval to manage tokens.
- **Who Can Perform**: Any user.
- **Details**: Allows users to check whether a specific address has been granted approval to manage or transfer tokens from another address. Useful for verifying the current approval status before initiating transactions or other token management actions.

### Transfer of Tokens by Approved Address

- **Action**: Transfer tokens on behalf of the token owner.
- **Who Can Perform**: Approved addresses.
- **Details**: Enables approved addresses to transfer tokens without requiring the direct involvement of the token owner for each transaction. Facilitates efficient and flexible token management by trusted parties.

### Differences in Token Types

- **Non-Fungible Tokens (NFTs)**:
    - **Parameters**: `tokenId` (or `itemId`)
    - **Details**: Approval allows the approved address to transfer specific NFTs. Each NFT is unique, and the approval pertains to managing these unique assets.

- **Refungible Tokens (RFTs)**:
    - **Parameters**: `tokenId` and `amount`
    - **Details**: RFTs are a type of token that can be split between multiple users while retaining unique properties. The approve functionality involves managing these split tokens and transferring them according to the rules set for RFTs.

- **Fungible Tokens (FTs)**:
    - **Parameters**: `amount`
    - **Details**: Approval for fungible tokens allows the approved address to manage or transfer a specific quantity of these interchangeable tokens. Similar to traditional ERC-20 tokens in the Ethereum ecosystem.

For further details and specific implementation examples, refer to the [approve.test.ts file](https://github.com/UniqueNetwork/unique-chain/blob/master/js-packages/tests/approve.test.ts).
