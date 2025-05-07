import * as anchor from "@coral-xyz/anchor";
import { Vbw } from "../target/types/vbw";
import self from "./preset";

const program = anchor.workspace.Vbw as anchor.Program<Vbw>;
const provider = anchor.AnchorProvider.env();

anchor.setProvider(provider);
self.setENV(provider,program.programId);

const reqs={
  init:async(ignore)=>{
    const users=await self.init({balance:!ignore});
    
    self.output.start(`System initialization`);
    const pkey=users.manager.pair.publicKey.toString()
    const recipient=users.recipient.pair.publicKey.toString();
    const sign_init= await program.methods
      .init(pkey,recipient)
      .accounts({
        payer:users.root.pair.publicKey,
      })
      .signers([users.root.pair])
      .rpc()
      .catch((err)=>{
        self.output.hr("Got Error");
        console.log(err);
      });
    await self.info.whitelist();
    await self.info.modulecounter();
    await self.info.texturecounter();
    self.output.end(`Signature of init: ${sign_init}`);
  },
  create:async()=>{

  },
}

describe("VBW world functions test.",() => {
  it("Init system successful test.", async () => {
    await reqs.init(true);

  });

  it("Create a new world.", async () => {
    await reqs.create();

  });

});
