import * as anchor from "@coral-xyz/anchor";
import { Vbw } from "../target/types/vbw";
import self from "./preset";

const program = anchor.workspace.Vbw as anchor.Program<Vbw>;
const provider = anchor.AnchorProvider.env();

anchor.setProvider(provider);
self.setENV(provider,program.programId);

const reqs={
  mint:async (x,y,world)=>{
    self.output.start(`Mint new block`);
    const users=await self.init({balance:false});
    
    const sign_init= await program.methods
      .mintBlock(x,y,world)
      .accounts({
        payer:users.creator.pair.publicKey,
      })
      .signers([users.creator.pair])
      .rpc()
      .catch((err)=>{
        self.output.hr("Got Error");
        console.log(err);
      });

      await self.info.blockdata(x,y,world);
      self.output.end(`Signature of "mintBlock": ${sign_init}`);
  },
}

describe("VBW block functions test.",() => {

  it("Mint block.", async () => {
    const x=2025,y=502,world=0;
    await reqs.mint(x,y,world);
  });

});
