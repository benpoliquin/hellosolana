const anchor = require('@project-serum/anchor');

const { SystemProgram } = anchor.web3;

const main = async() => {
  console.log("Starting the test!")

  const provider = anchor.Provider.env()
  anchor.setProvider(provider)

  const program = anchor.workspace.Helloworld;
  const baseAccount = anchor.web3.Keypair.generate();

  const tx = await program.rpc.startStuff({
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
      systemProgram: SystemProgram.programId
    },
    signers: [baseAccount]
  });

  let account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log("ACCOUNT TOTAL --", account);

  // Call AddOne
  await program.rpc.addOne({
    accounts: {
      baseAccount: baseAccount.publicKey,
      // user: provider.wallet.publicKey,
    },
  });

  account = await program.account.baseAccount.fetch(baseAccount.publicKey)
  console.log("ACCOUNT TOTAL --", account);

  console.log("Sig --", tx);
}

const runMain = async () => {
  try {
    await main();
    process.exit(0);
  } catch(error) {
    console.error(error);
    process.exit(1)
  }
};

runMain();