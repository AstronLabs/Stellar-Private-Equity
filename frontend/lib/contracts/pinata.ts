/**
 * Pinata / IPFS upload helpers.
 * Uploads evidence files and metadata JSON to backend which proxies to Pinata.
 * 
 * NOTE: IPFS uploads now require x402 payment (0.005 USDC).
 * Use the functions in lib/api.ts for automatic payment handling.
 * 
 * @deprecated Use lib/api.ts uploadEvidenceFile instead for x402 support.
 */

import { parsePaymentRequirement, createPayment } from "../x402-client";

const BACKEND_URL = "";

export type UploadResult = {
  cid: string;
  url: string;
};

/** Upload a file to IPFS via the backend.
 * @deprecated Use uploadEvidenceFile from lib/api.ts instead
 */
export async function uploadFile(
  file: File,
  stellarAddress?: string,
  onPaymentRequired?: (amount: string) => void
): Promise<UploadResult> {
  const formData = new FormData();
  formData.append("file", file);

  const headers: Record<string, string> = {};
  if (stellarAddress) headers["x-stellar-address"] = stellarAddress;

  let res = await fetch(`${BACKEND_URL}/api/ipfs/upload`, {
    method: "POST",
    body: formData,
    headers,
  });

  // Handle x402 payment required
  if (res.status === 402 && stellarAddress) {
    const requirement = await parsePaymentRequirement(res);
    if (!requirement) {
      throw new Error("IPFS upload requires payment but no valid x402 requirement found");
    }
    
    onPaymentRequired?.(requirement.amount);
    
    // Create payment proof
    const proof = await createPayment(requirement);
    
    // Retry with payment proof
    const formRetry = new FormData();
    formRetry.append("file", file);
    
    res = await fetch(`${BACKEND_URL}/api/ipfs/upload`, {
      method: "POST",
      body: formRetry,
      headers: {
        "x-stellar-address": stellarAddress,
        "x-payment-proof": JSON.stringify(proof),
      },
    });
  }

  if (!res.ok) {
    const errBody = await res.json().catch(() => ({}));
    throw new Error(errBody.error || `Upload failed: ${res.statusText}`);
  }

  const data = await res.json();
  return {
    cid: data.data?.cid ?? data.cid ?? data.IpfsHash,
    url: data.data?.url ?? `https://gateway.pinata.cloud/ipfs/${data.data?.cid ?? data.cid ?? data.IpfsHash}`,
  };
}

/** Upload JSON metadata to IPFS via the backend */
export async function uploadMetadata(
  metadata: Record<string, unknown>
): Promise<UploadResult> {
  const res = await fetch(`${BACKEND_URL}/api/ipfs/upload-json`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(metadata),
  });

  if (!res.ok) {
    throw new Error(`Metadata upload failed: ${res.statusText}`);
  }

  const data = await res.json();
  return {
    cid: data.cid ?? data.IpfsHash,
    url: `https://gateway.pinata.cloud/ipfs/${data.cid ?? data.IpfsHash}`,
  };
}
