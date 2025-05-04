import * as anchor from "@coral-xyz/anchor";
import md5 from "md5";
import { BN } from "bn.js";

import { Vbw } from "../target/types/vbw";

const program = anchor.workspace.Luckysig as anchor.Program<Vbw>;
const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);
//self.setENV(provider,program.programId);   

describe("VBW world init test.",() => {

  it("Init system successful test.", async () => {
    
    
  });

});
