// ── Claim Types ─────────────────────────────────────────────────

export enum ClaimStatus {
  Pending = 'Pending',
  Approved = 'Approved',
  Rejected = 'Rejected',
  PaidOut = 'PaidOut',
}

export interface Claim {
  id: number;
  poolId: number;
  claimant: string;
  amount: bigint;
  description: string;
  evidence: string; // IPFS hash or URL
  status: ClaimStatus;
  createdAt: number;
  resolvedAt: number | null;
  votesFor: number;
  votesAgainst: number;
  votesAbstain: number;
}

export interface SubmitClaimParams {
  poolId: number;
  amount: string;
  description: string;
  evidence: string;
}
