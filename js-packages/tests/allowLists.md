# Allow Lists

The Allow Lists feature in Unique Network restricts specific actions (minting and transfers) to addresses included in an allow list. This feature enhances security and control over token management.

## Key Functionalities

### Adding to Allow List

- **Action**: Add addresses to the allow list of a collection.
- **Who Can Perform**: Owners and admins.
- **Note**: Adding addresses is allowed even if the allow list is not currently enabled.

### Removing from Allow List

- **Action**: Remove addresses from the allow list.
- **Who Can Perform**: Owners and admins.
- **Note**: Removing an address that is not on the list has no effect.

### Transfer Restrictions

- **Action**: Transfer tokens.
- **Condition**: Allowed only between addresses on the allow list when Public Access mode is set to AllowList.
- **Who Can Perform**: Any allowlisted address.
- **Restriction**: Non-allowlisted addresses cannot participate in transfers.

### Minting Restrictions

- **Action**: Mint new tokens.
- **Condition**: Only allowlisted addresses can mint tokens when minting is restricted to the allow list.
- **Who Can Perform**: Any allowlisted address with minting permissions.

### Public Access Mode Settings

- **Action**: Change Public Access mode to AllowList.
- **Who Can Perform**: Owners and admins.
- **Effect**: Limits all actions (transfers, minting) to allowlisted addresses only.
