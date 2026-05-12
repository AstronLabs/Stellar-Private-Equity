// ── Vote Types ──────────────────────────────────────────────────

export enum VoteChoice {
  Approve = 'Approve',
  Reject = 'Reject',
  Abstain = 'Abstain',
}

export interface Vote {
  claimId: number;
  voter: string;
  choice: VoteChoice;
  timestamp: number;
}

export interface VotingResult {
  claimId: number;
  totalVotes: number;
  approvals: number;
  rejections: number;
  abstentions: number;
  quorumReached: boolean;
  approved: boolean;
}

export interface CastVoteParams {
  claimId: number;
  choice: VoteChoice;
}
