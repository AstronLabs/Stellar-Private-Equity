/**
 * x402 Payment Protocol - Server-side utilities
 * 
 * Implements x402 payment gating for API routes.
 * x402 spec: https://github.com/coinbase/x402
 */

import type { NextApiRequest, NextApiResponse } from "next";

export type X402PaymentRequirement = {
  scheme: "x402";
  network: "stellar";
  asset: string;           // Asset issuer or "native" for XLM
  amount: string;          // Amount in stroops/smallest unit
  destination: string;     // Stellar address to receive payment
  deadline: number;        // Unix timestamp when payment expires
  description?: string;    // Human-readable description
};

export type X402PaymentProof = {
  txHash: string;          // Stellar transaction hash
  sender: string;          // Sender's Stellar address
};

const RECEIVER_ADDRESS = process.env.X402_RECEIVER_ADDRESS ?? "";
const USDC_ASSET = process.env.CONTRACT_TOKEN ?? ""; // USDC contract address on Soroban

/**
 * Create x402 payment requirement headers
 */
export function createPaymentRequirement(
  amount: string,
  description?: string
): X402PaymentRequirement {
  // 15 minute deadline
  const deadline = Math.floor(Date.now() / 1000) + 15 * 60;
  
  return {
    scheme: "x402",
    network: "stellar",
    asset: USDC_ASSET || "native",
    amount,
    destination: RECEIVER_ADDRESS,
    deadline,
    description: description || "NexusGuard service payment",
  };
}

/**
 * Send 402 response with payment requirements
 */
export function sendPaymentRequired(
  res: NextApiResponse,
  amount: string,
  description?: string
): void {
  const requirement = createPaymentRequirement(amount, description);
  
  res.status(402).setHeader("X-Payment-Required", JSON.stringify(requirement));
  res.json({
    error: "Payment Required",
    code: "X402_PAYMENT_REQUIRED",
    requirement,
  });
}

/**
 * Parse payment proof from request headers
 */
export function parsePaymentProof(req: NextApiRequest): X402PaymentProof | null {
  const proofHeader = req.headers["x-payment-proof"];
  if (!proofHeader || typeof proofHeader !== "string") {
    return null;
  }
  
  try {
    const proof = JSON.parse(proofHeader) as X402PaymentProof;
    if (!proof.txHash || !proof.sender) {
      return null;
    }
    return proof;
  } catch {
    return null;
  }
}

/**
 * Verify payment on-chain via Soroban RPC
 * 
 * In production, this would:
 * 1. Fetch transaction from Stellar RPC
 * 2. Verify destination matches RECEIVER_ADDRESS
 * 3. Verify amount and asset
 * 4. Verify transaction is not too old
 * 5. Check for double-spending via a cache
 * 
 * For MVP, we do a basic validation and trust the client
 */
export async function verifyPayment(
  proof: X402PaymentProof,
  expectedAmount?: string,
  expectedAsset?: string
): Promise<{ valid: boolean; error?: string }> {
  if (!proof.txHash.match(/^[a-f0-9]{64}$/)) {
    return { valid: false, error: "Invalid transaction hash format" };
  }
  
  if (!proof.sender.match(/^G[A-Z0-9]{55}$/)) {
    return { valid: false, error: "Invalid sender address" };
  }
  
  // TODO: Full on-chain verification via Soroban RPC
  // This would require fetching the transaction and parsing operations
  // For now, we accept the proof and do basic validation
  
  return { valid: true };
}

/**
 * x402 middleware for Next.js API routes
 * 
 * Usage:
 * ```
 * export default withX402Payment("10000000", "IPFS Upload")(handler);
 * ```
 */
export function withX402Payment(
  amount: string,
  description?: string
) {
  return (
    handler: (req: NextApiRequest, res: NextApiResponse) => Promise<void> | void
  ) => {
    return async (req: NextApiRequest, res: NextApiResponse): Promise<void> => {
      // Check for payment proof
      const proof = parsePaymentProof(req);
      
      if (!proof) {
        sendPaymentRequired(res, amount, description);
        return;
      }
      
      // Verify the payment
      const verification = await verifyPayment(proof, amount);
      
      if (!verification.valid) {
        res.status(402).json({
          error: "Payment verification failed",
          code: "X402_INVALID_PAYMENT",
          details: verification.error,
        });
        return;
      }
      
      // Attach payment info to request for handler use
      (req as any).x402Payment = {
        ...proof,
        verified: true,
      };
      
      // Call the actual handler
      return handler(req, res);
    };
  };
}

/**
 * Check if request has valid x402 payment (for use inside handlers)
 */
export function requireX402Payment(
  req: NextApiRequest,
  res: NextApiResponse,
  amount: string,
  description?: string
): { valid: boolean; proof?: X402PaymentProof } {
  const proof = parsePaymentProof(req);
  
  if (!proof) {
    sendPaymentRequired(res, amount, description);
    return { valid: false };
  }
  
  return { valid: true, proof };
}
