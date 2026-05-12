// ── User Types ──────────────────────────────────────────────────

export interface User {
  publicKey: string;
  displayName?: string;
  joinedPools: number[];
  totalContributed: bigint;
  totalClaimed: bigint;
}

export interface WalletState {
  isConnected: boolean;
  publicKey: string | null;
  network: 'testnet' | 'mainnet' | 'futurenet';
}
