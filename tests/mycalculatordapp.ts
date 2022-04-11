import assert from "assert";
import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Mycalculatordapp } from "../target/types/mycalculatordapp";

describe("mycalculatordapp", () => {
    // Configure the client to use the local cluster.
    const provider = anchor.Provider.env();
    anchor.setProvider(provider);

    const calculator = anchor.web3.Keypair.generate();
    const program = anchor.workspace.Mycalculatordapp as Program<Mycalculatordapp>;

    it("Creates a calculator", async () => {
        await program.rpc.create("Hello Solana", {
            accounts: {
                calculator: calculator.publicKey,
                user: provider.wallet.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId
            },
            signers: [calculator]
        });

        const account = await program.account.calculator.fetch(calculator.publicKey);
        assert.ok(account.greeting === "Hello Solana");
    });

    it("Adds two numbers", async () => {
        await program.rpc.add(new anchor.BN(3), new anchor.BN(4), {
            accounts: {
                calculator: calculator.publicKey
            }
        });

        const account = await program.account.calculator.fetch(calculator.publicKey);
        assert.ok(account.result.eq(new anchor.BN(7)));
    });

    it("Substracts two numbers", async () => {
        await program.rpc.substract(new anchor.BN(7), new anchor.BN(3), {
            accounts: {
                calculator: calculator.publicKey
            }
        });

        const account = await program.account.calculator.fetch(calculator.publicKey);
        assert.ok(account.result.eq(new anchor.BN(4)));
    });

    it("Multiply two numbers", async () => {
        await program.rpc.multiply(new anchor.BN(7), new anchor.BN(2), {
            accounts: {
                calculator: calculator.publicKey
            }
        });

        const account = await program.account.calculator.fetch(calculator.publicKey);
        assert.ok(account.result.eq(new anchor.BN(14)));
    });

    it("Divide two numbers", async () => {
        await program.rpc.divide(new anchor.BN(9), new anchor.BN(2), {
            accounts: {
                calculator: calculator.publicKey
            }
        });

        const account = await program.account.calculator.fetch(calculator.publicKey);
        assert.ok(account.result.eq(new anchor.BN(4)));
        assert.ok(account.remainder.eq(new anchor.BN(1)));
    });
});
