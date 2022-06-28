# ic_signer
To sign a message with the keys stored in the Internet Computer.

## Running the project locally

If you want to test the project locally, you can use the following commands:

```bash
# Install tools
sh -ci "$(curl -fsSL https://smartcontracts.org/install.sh)"

# Verify that dfx properly installed
dfx --version

# Clone this project
git clone https://github.com/dizzy27/ic_signer.git

# Start the replica, running in the background
cd ic_signer
dfx start --background

# Download the internet identity and deploys it locally, you can skip this for mainnet
# After deploying, copy the canister ID fom the Internet Identity canister
# And paste it into webpack/dfinity.js in this project on the LOCAL_II_CANISTER variable on line 45.
cd ..
git clone https://github.com/dfinity/internet-identity.git
cd ../internet-identity
rm -rf .dfx/local
II_FETCH_ROOT_KEY=1 II_DUMMY_CAPTCHA=1  dfx deploy --no-wallet --argument '(null)'

# Deploy ic_signer with frontend
cd ../ic_signer
npm install
dfx deploy
```

Once the job completes, ic_signer will be available at `http://localhost:8000?canisterId={asset_canister_id}`.

To learn more, see the following documentation available online:

- [Quick Start](https://smartcontracts.org/docs/quickstart/quickstart-intro.html)
- [SDK Developer Tools](https://smartcontracts.org/docs/developers-guide/sdk-guide.html)
- [Rust Canister Devlopment Guide](https://smartcontracts.org/docs/rust-guide/rust-intro.html)
- [ic-cdk](https://docs.rs/ic-cdk)
- [ic-cdk-macros](https://docs.rs/ic-cdk-macros)
- [Candid Introduction](https://smartcontracts.org/docs/candid-guide/candid-intro.html)
- [JavaScript API Reference](https://erxue-5aaaa-aaaab-qaagq-cai.raw.ic0.app)