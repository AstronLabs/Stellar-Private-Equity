// ── Pool Types ──────────────────────────────────────────────────

export enum PoolStatus {
  Active = 'Active',
  Inactive = 'Inactive',
  Dissolved = 'Dissolved',
}

export interface Pool {
  id: number;
  name: string;
  description: string;
  creator: string;
  maxMembers: number;
  currentMembers: number;
  contributionAmount: bigint;
  maxPayout: bigint;
  votingQuorum: number; // percentage (0-100)
  status: PoolStatus;
  createdAt: number;
  totalContributed: bigint;
  totalPaidOut: bigint;
}

export interface PoolMember {
  pool_id: number;
  member: string;
  totalContributed: bigint;
  joinedAt: number;
}

export interface CreatePoolParams {
  name: string;
  description: string;
  maxMembers: number;
  contributionAmount: string;
  maxPayout: string;
  votingQuorum: number;
}
