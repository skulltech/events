import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Events } from "../target/types/events";

describe("events", () => {
  const program = anchor.workspace.Events as Program<Events>;

  let eventsFired = [];

  it("Is initialized!", async () => {
    program.addEventListener("MyEvent", (event) => {
      console.log("event fired");
      eventsFired.push(event);
    });

    const tx = await program.rpc.initialize({});

    await program.provider.connection.confirmTransaction(tx);

    console.log(eventsFired);
  });
});
