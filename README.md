# RISC TLS 1.3

Built for the ETH Denver 2024 Hackathon.

## About

RISC TLS 1.3 leverages RISC Zero and multiparty computation to let users securely prove communication with a webserver.

Our solution aims to address the challenge of verifying the authenticity of encrypted conversations (such as those over HTTPS) in a decentralized and privacy-preserving manner. It leverages RISC Zero proofs to enable smart contracts to interact with Web3 actions based on potentially private Web2 data.

The core problem revolves around proving the integrity of encrypted communications without revealing sensitive information to third parties. For instance, if you wish to validate a conversation with an AI model like ChatGPT without exposing the content of the conversation itself, traditional methods fall short. This is because the encryption keys required to decrypt the communication are the same ones needed to fabricate it, posing a credibility issue.

Our system employs Multi-Party Computation to establish trust without revealing content.  The key components that make it work are:

Key Generation: A group of notaries collaborates to generate a private key. The client interacts with a website or API over HTTPS using this key.

Content Commitment: Before gaining access to the decryption keys, the client commits to the hash of the message contents. This commitment ensures that the client cannot modify the message after decryption.

Notary Verification: The notaries, unaware of the message content, verify the hash commitment. They sign the hash to attest to its authenticity.

RISC Zero Proofs: Using the notary-signed hash and other relevant data, RISC Zero proofs are constructed to verify the authenticity of the conversation and any specific properties of interest, like server authentication or specific API interactions.

Verification: These proofs can be verified on-chain or off-chain, enabling trustless validation of encrypted communications.

For our demo we prove a ChatGPT response, but any Web2 data could be proved and put onchain.

## Run

In root path, run
```shell
cargo run --release
```
