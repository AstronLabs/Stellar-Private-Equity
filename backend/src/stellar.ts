import {
  rpc,
  Keypair,
  Networks,
  TransactionBuilder,
  Contract,
  Address,
} from "@stellar/stellar-sdk";
import dotenv from "dotenv";

dotenv.config();

const RPC_URL = process.env.STELLAR_RPC_URL || "https://soroban-testnet.stellar.org";
const NETWORK_PASSPHRASE =
  process.env.STELLAR_NETWORK === "mainnet"
    ? Networks.PUBLIC
    : Networks.TESTNET;

export const server = new rpc.Server(RPC_URL);

// Load GP signer for admin tasks (e.g. whitelisting after KYC approval)
const getGpKeypair = (): Keypair | null => {
  const secret = process.env.GP_SECRET_KEY;
  if (!secret || secret.startsWith("SAXX")) {
    console.warn("GP_SECRET_KEY not configured. Administrative contract calls will be simulated.");
    return null;
  }
  return Keypair.fromSecret(secret);
};

export const gpKeypair = getGpKeypair();
const FUND_CONTRACT_ID = process.env.FUND_CONTRACT_ID || "";

/**
 * Whitelists an investor on the Fund smart contract.
 * This is triggered by the backend after successful KYC verification.
 */
export async function whitelistInvestorOnChain(investorAddress: string): Promise<string> {
  if (!gpKeypair) {
    console.log(`[Simulation] Whitelisted investor ${investorAddress} on-chain.`);
    return "simulation_tx_hash_" + Math.random().toString(36).substring(2, 15);
  }

  const gpAddress = gpKeypair.publicKey();
  const contract = new Contract(FUND_CONTRACT_ID);

  console.log(`Preparing whitelisting transaction for investor: ${investorAddress}`);

  // 1. Fetch GP account
  const account = await server.getAccount(gpAddress);

  // 2. Build the Soroban invocation
  const tx = new TransactionBuilder(account, {
    fee: "100000",
    networkPassphrase: NETWORK_PASSPHRASE,
  })
    .addOperation(
      contract.call(
        "whitelist_investor",
        Address.fromString(gpAddress).toScVal(), // GP authorizes the call
        Address.fromString(investorAddress).toScVal() // Investor to whitelist
      )
    )
    .setTimeout(30)
    .build();

  // 3. Simulate the transaction to estimate footprint and fees
  const simulation = await server.simulateTransaction(tx);
  if (rpc.Api.isSimulationError(simulation)) {
    throw new Error(`Simulation failed: ${simulation.error}`);
  }

  // 4. Assemble transaction with simulation results and build it
  const assembledTx = rpc.assembleTransaction(tx, simulation).build();

  // 5. Sign and submit
  assembledTx.sign(gpKeypair);
  const response = await server.sendTransaction(assembledTx);

  if (response.status === "PENDING") {
    // Wait for transaction to be mined
    let statusResponse = await server.getTransaction(response.hash);
    while (statusResponse.status === rpc.Api.GetTransactionStatus.NOT_FOUND) {
      await new Promise((resolve) => setTimeout(resolve, 1000));
      statusResponse = await server.getTransaction(response.hash);
    }
    if (statusResponse.status === rpc.Api.GetTransactionStatus.SUCCESS) {
      return response.hash;
    } else {
      throw new Error(`Transaction failed with status: ${statusResponse.status}`);
    }
  } else {
    throw new Error(`Failed to submit transaction: ${response.status}`);
  }
}
