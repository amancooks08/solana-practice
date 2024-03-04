# solana-practice
I am to start with the Solana Ecosystem, this repository is my journal for the journey.

# Setup
There are various things to have in prior while starting development in the Solana Ecosystem. These are:
1. First of all make sure to install Node.js, you can find the directions [here](https://github.com/nodesource/distributions/blob/master/README.md).
2. Install Rust, refer the official docs [here](https://www.rust-lang.org/tools/install).
3. Now we will install Solana, refer to the docs [here](https://docs.solanalabs.com/cli/install).
4. Setting up Phantom & Solana Validator Node. 
  - For Phantom refer [here](https://phantom.app/).
  - For Solana Validator Node refer [here](https://docs.solanalabs.com/operations/setup-a-validator).
5. For installing and setting up Anchor, click [here](https://www.anchor-lang.com/docs/installation).
6. Now, clone this repository using the following command:
```
git clone https://github.com/amancooks08/solana-practice.git
```
7. Now, navigate to the cloned repository and install the dependencies using the following command:
```
yarn install
```
8. Now, to run the tests, or to use the current programs, you need to enter the particular directory and build the program using the following command:
```
anchor build
```
9. Now, you can run the tests using the following command:
```
anchor test
```
