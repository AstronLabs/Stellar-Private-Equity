import { Router, Request, Response } from "express";
import { whitelistInvestorOnChain } from "./stellar";

const router = Router();

// Mock Database for KYC submissions
interface KycSubmission {
  investorAddress: string;
  fullName: string;
  documentType: string;
  documentNumber: string;
  status: "PENDING" | "APPROVED" | "REJECTED";
  txHash?: string;
  submittedAt: Date;
}

const kycDb: Map<string, KycSubmission> = new Map();

// Health Check
router.get("/health", (req: Request, res: Response) => {
  res.json({ status: "ok", timestamp: new Date() });
});

// 1. Submit KYC (LP action)
router.post("/kyc/submit", (req: Request, res: Response) => {
  const { investorAddress, fullName, documentType, documentNumber } = req.body;

  if (!investorAddress || !fullName || !documentType || !documentNumber) {
    return res.status(400).json({ error: "Missing required fields" });
  }

  const submission: KycSubmission = {
    investorAddress,
    fullName,
    documentType,
    documentNumber,
    status: "PENDING",
    submittedAt: new Date(),
  };

  kycDb.set(investorAddress, submission);
  console.log(`KYC submitted for investor: ${investorAddress}`);

  return res.status(201).json({
    message: "KYC submitted successfully",
    submission,
  });
});

// 2. Get Pending KYC Submissions (GP action)
router.get("/kyc/pending", (req: Request, res: Response) => {
  const pending = Array.from(kycDb.values()).filter((s) => s.status === "PENDING");
  return res.json(pending);
});

// 3. Approve KYC & Trigger Whitelisting (GP action)
router.post("/kyc/approve", async (req: Request, res: Response) => {
  const { investorAddress } = req.body;

  if (!investorAddress) {
    return res.status(400).json({ error: "Missing investor address" });
  }

  const submission = kycDb.get(investorAddress);
  if (!submission) {
    return res.status(404).json({ error: "KYC submission not found" });
  }

  if (submission.status === "APPROVED") {
    return res.status(400).json({ error: "Investor is already approved" });
  }

  try {
    // Trigger on-chain whitelisting via the GP's key
    const txHash = await whitelistInvestorOnChain(investorAddress);

    submission.status = "APPROVED";
    submission.txHash = txHash;
    kycDb.set(investorAddress, submission);

    console.log(`Successfully whitelisted investor ${investorAddress}. Tx: ${txHash}`);

    return res.json({
      message: "KYC approved and investor whitelisted on-chain",
      txHash,
      submission,
    });
  } catch (error: any) {
    console.error(`Error whitelisting investor:`, error);
    return res.status(500).json({
      error: "Failed to whitelist investor on-chain",
      details: error.message,
    });
  }
});

// 4. Get Investor KYC Status
router.get("/investor/:address", (req: Request, res: Response) => {
  const { address } = req.params;
  const submission = kycDb.get(address);

  if (!submission) {
    return res.json({ status: "NONE" });
  }

  return res.json(submission);
});

export default router;
