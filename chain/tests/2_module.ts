import * as anchor from "@coral-xyz/anchor";
import { Vbw } from "../target/types/vbw";
import self from "./preset";

const program = anchor.workspace.Vbw as anchor.Program<Vbw>;
const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);
self.setENV(provider,program.programId);

const reqs={
  add:async (ipfs,index)=>{
    const users=await self.init({balance:true});
    self.output.start(`Add new module.`);
    await self.info.modulecounter();
    const sign_init= await program.methods
      .addModule(ipfs,index)
      .accounts({
        payer:users.manager.pair.publicKey,
      })
      .signers([users.manager.pair])
      .rpc()
      .catch((err)=>{
        self.output.hr("Got Error");
        console.log(err);
      });
      //await self.info.moduledata(index);
      self.output.end(`Signature of "addModule": ${sign_init}`);
  },
}

describe("VBW module functions test.",() => {

  it("Add a new module ( IPFS ).", async () => {
    // const ipfs="bafkreicl7rl7d6bkgyzxc67jdfoythbthikk7bnt4m22zjd6e7jx5hoerb";
    // const index=1;
    // await reqs.add(ipfs,index);
  });
});
