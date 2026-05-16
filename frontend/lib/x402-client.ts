/**
 * x402 Payment Protocol - Client-side utilities
 * 
 * Handles x402 payment requirements from server and creates payments via Freighter.
 */

import {
  TransactionBuilder,
  Asset,
  Operation,
  Networks,
  BASE_FEE,
} from "@stellar/stellar-sdk";
import { signTransaction, requestAccess } from "@stellar/freighter-api";

export type X402Requirement = {
  scheme: "x402";
  network: "stellar";
  asset: string;
  amount: string;
  destination: string;
  deadline: number;
  description?: string;
};

export type X402Proof = {
  txHash: string;
  sender: string;
};

const FACILITATOR_URL = process.env.NEXT_PUBLIC_X402_FACILITATOR_URL ?? "";

/**
 * Parse x402 payment requirement from response
 */
export async function parsePaymentRequirement(
  response: Response
): Promise<X402Requirement | null> {
  if (response.status !== 402) {
    return null;
  }
  
  const header = response.headers.get("X-Payment-Required");
  if (!header) {
    return null;
  }
  
  try {
    return JSON.parse(header) as X402Requirement;
  } catch {
    return null;
  }
}

/**
 * Create payment transaction using Freighter
 * 
 * Returns the signed transaction XDR and hash
 */
export async function createPayment(
  requirement: X402Requirement
): Promise<X402Proof> {
  // Get user's address from Freighter
  const access = await requestAccess();
  const sender = access.address;
  
  if (!sender) {
    throw new Error("No wallet connected");
  }
  
  // Create a simple payment transaction
  // In production, this might use Path Payment Strict Send for USDC
  const transaction = new TransactionBuilder(
    // Dummy account - we'll use Freighter to build the actual transaction
    // This is a simplified approach; production would use Soroban contract calls for token transfers
    { accountId: () => sender } as any,
    {
      networkPassphrase: Networks.TESTNET,
      fee: BASE_FEE,
    }
  )
    .addOperation(
      Operation.payment({
        destination: requirement.destination,
        asset: requirement.asset === "native" 
          ? Asset.native() 
          : new Asset("USDC", requirement.asset),
        amount: (BigInt(requirement.amount) / BigInt(10000000)).toString(), // Convert stroops to units
      })
    )
    .setTimeout(30)
    .build();
  
  // Sign with Freighter
  const signedResult = await signTransaction(transaction.toXDR(), {
    networkPassphrase: Networks.TESTNET,
  });
  
  if (!signedResult.signedTxXdr) {
    throw new Error("Failed to sign payment transaction");
  }
  
  // Extract transaction hash (simplified)
  const signedTx = TransactionBuilder.fromXDR(
    signedResult.signedTxXdr,
    Networks.TESTNET
  );
  
  return {
    txHash: signedTx.hash().toString("hex"),
    sender,
  };
}

/**
 * Fetch with x402 payment support
 * 
 * Automatically handles 402 responses and retries with payment
 */
export async function fetchWithX402(
  url: string,
  options: RequestInit = {},
  stellarAddress: string
): Promise<Response> {
  // First attempt
  let response = await fetch(url, {
    ...options,
    headers: {
      ...options.headers,
      "x-stellar-address": stellarAddress,
    },
  });
  
  // If payment required, create payment and retry
  if (response.status === 402) {
    const requirement = await parsePaymentRequirement(response);
    
    if (!requirement) {
      throw new Error("Server requires payment but no x402 requirement found");
    }
    
    // Create payment
    const proof = await createPayment(requirement);
    
    // Retry with payment proof
    response = await fetch(url, {
      ...options,
      headers: {
        ...options.headers,
        "x-stellar-address": stellarAddress,
        "x-payment-proof": JSON.stringify(proof),
      },
    });
  }
  
  return response;
}

/**
 * Upload file with x402 payment (for IPFS uploads)
 */
export async function uploadFileWithX402(
  file: File,
  stellarAddress: string,
  onPaymentRequired?: (amount: string) => void
): Promise<{ cid: string; url: string }> {
  const formData = new FormData();
  formData.append("file", file);
  
  // First attempt without payment
  let response = await fetch("/api/ipfs/upload", {
    method: "POST",
    body: formData,
    headers: {
      "x-stellar-address": stellarAddress,
    },
  });
  
  // Handle x402 payment required
  if (response.status === 402) {
    const requirement = await parsePaymentRequirement(response);
    
    if (!requirement) {
      throw new Error("IPFS upload requires payment");
    }
    
    onPaymentRequired?.call(null, requirement.amount);
    
    // Create and submit payment
    const proof = await createPayment(requirement);
    
    // Retry with payment proof
    response = await fetch("/api/ipfs/upload", {
      method: "POST",
      body: formData,
      headers: {
        "x-stellar-address": stellarAddress,
        "x-payment-proof": JSON.stringify(proof),
      },
    });
  }
  
  if (!response.ok) {
    const error = await response.json();
    throw new Error(error.error || "Upload failed");
  }
  
  const result = await response.json();
  return result.data;
}
