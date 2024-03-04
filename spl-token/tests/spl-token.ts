import * as anchor from "@coral-xyz/anchor";  
import { Program } from "@coral-xyz/anchor";
import { SplToken } from "../target/types/spl_token";
import { TOKEN_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";
import { createAccount } from "@solana/spl-token";
import { assert, expect } from "chai";

describe("spl-token", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider);

  const program = anchor.workspace.SplToken as Program<SplToken>
  

  const mintToken = anchor.web3.Keypair.generate()

  const associateTokenProgram = new anchor.web3.PublicKey("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL")

  const tokenAccount =anchor.utils.token.associatedAddress({mint:mintToken.publicKey,owner:provider.publicKey})

  const ta = anchor.web3.PublicKey.findProgramAddressSync(
    [provider.publicKey.toBuffer(),TOKEN_PROGRAM_ID.toBuffer(),mintToken.publicKey.toBuffer()],
    associateTokenProgram
  )[0]


  it("Testing Create token...", async () => {

    console.log(mintToken.publicKey.toBase58())
    console.log(tokenAccount.toBase58())

    try {
      const tx = await program.methods.createToken(9,new anchor.BN(10**9*100))
        .accounts({
          mintToken:mintToken.publicKey,
          tokenAccount:tokenAccount,
          associateTokenProgram,
        })
        .signers([mintToken])
        .rpc();
        console.log("Your transaction signature", tx);
    } catch (error) {
        console.log(error)
    }
  });

  it("Testing token transfer...", async () =>{

    let reciever = anchor.web3.Keypair.generate()

    // We request 1 SOL for transactions
    const signature = await provider.connection.requestAirdrop(reciever.publicKey,anchor.web3.LAMPORTS_PER_SOL)
    await provider.connection.confirmTransaction(signature)

    let recieverTokenAccountKeypair = anchor.web3.Keypair.generate()
    await createAccount(provider.connection,reciever,mintToken.publicKey,reciever.publicKey,recieverTokenAccountKeypair);
    const tx = await program.methods.transerToken(new anchor.BN(10**9*90))
    .accounts({
      mintToken:mintToken.publicKey,
      fromAccount:tokenAccount,
      toAccount:recieverTokenAccountKeypair.publicKey,
      associateTokenProgram
    })
    .signers([])
    .rpc()
    
    assert(tx !== undefined, "error: transaction failed")
    console.log("Your transaction signature", tx);
  })
});