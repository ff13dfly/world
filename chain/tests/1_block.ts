import * as anchor from "@coral-xyz/anchor";
import { Vbw } from "../target/types/vbw";

const program = anchor.workspace.Luckysig as anchor.Program<Vbw>;
const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);
//self.setENV(provider,program.programId);   

describe("VBW block functions test.",() => {

  it("Mint block.", async () => {
    
    
  });

});
