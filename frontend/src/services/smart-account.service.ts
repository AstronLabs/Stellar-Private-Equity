/**
 * Smart Account Service
 *
 * Wraps all Soroban contract calls for the smart_account contract.
 * In production, these would build and submit real Soroban transactions.
 * Currently provides the interface + mock data for frontend development.
 */

import type {
  RecurringPayment,
  RecurringInterval,
  SpendingLimit,
  MultisigProposal,
  MultisigConfig,
  ScheduledTransfer,
} from '@/types/smart-account.types';

// Contract address — set via env in production
const SMART_ACCOUNT_CONTRACT_ID =
  process.env.NEXT_PUBLIC_SMART_ACCOUNT_CONTRACT_ID ?? '';

// ── Recurring Payments ──────────────────────────────────────────

export async function createRecurringPayment(
  owner: string,
  recipient: string,
  token: string,
  amount: bigint,
  interval: RecurringInterval,
  maxExecutions: number
): Promise<number> {
  // TODO: Build Soroban transaction calling smart_account.create_recurring
  console.log('[SmartAccount] createRecurringPayment', {
    owner,
    recipient,
    token,
    amount: amount.toString(),
    interval,
    maxExecutions,
  });
  return 0;
}

export async function executeRecurringPayment(
  caller: string,
  paymentId: number
): Promise<void> {
  // TODO: Build Soroban transaction calling smart_account.execute_recurring
  console.log('[SmartAccount] executeRecurringPayment', { caller, paymentId });
}

export async function cancelRecurringPayment(
  owner: string,
  paymentId: number
): Promise<void> {
  // TODO: Build Soroban transaction calling smart_account.cancel_recurring
  console.log('[SmartAccount] cancelRecurringPayment', { owner, paymentId });
}

export async function getRecurringPayment(
  paymentId: number
): Promise<RecurringPayment | null> {
  // TODO: Soroban query smart_account.get_recurring
  console.log('[SmartAccount] getRecurringPayment', { paymentId });
  return null;
}

export async function getRecurringCount(): Promise<number> {
  // TODO: Soroban query smart_account.recurring_count
  return 0;
}

// ── Spending Limits ─────────────────────────────────────────────

export async function setSpendingLimit(
  owner: string,
  token: string,
  maxAmount: bigint,
  periodSeconds: number
): Promise<void> {
  // TODO: Build Soroban transaction calling smart_account.set_spending_limit
  console.log('[SmartAccount] setSpendingLimit', {
    owner,
    token,
    maxAmount: maxAmount.toString(),
    periodSeconds,
  });
}

export async function checkSpending(
  owner: string,
  token: string,
  amount: bigint
): Promise<boolean> {
  // TODO: Soroban query smart_account.check_spending
  console.log('[SmartAccount] checkSpending', {
    owner,
    token,
    amount: amount.toString(),
  });
  return true;
}

export async function recordSpend(
  caller: string,
  owner: string,
  token: string,
  amount: bigint
): Promise<void> {
  // TODO: Build Soroban transaction calling smart_account.record_spend
  console.log('[SmartAccount] recordSpend', {
    caller,
    owner,
    token,
    amount: amount.toString(),
  });
}

export async function getSpendingLimit(
  owner: string,
  token: string
): Promise<SpendingLimit | null> {
  // TODO: Soroban query smart_account.get_spending_limit
  console.log('[SmartAccount] getSpendingLimit', { owner, token });
  return null;
}

// ── Multisig ────────────────────────────────────────────────────

export async function setupMultisig(
  owner: string,
  signers: string[],
  threshold: number
): Promise<void> {
  // TODO: Build Soroban transaction calling smart_account.setup_multisig
  console.log('[SmartAccount] setupMultisig', { owner, signers, threshold });
}

export async function proposeMultisigTx(
  proposer: string,
  recipient: string,
  token: string,
  amount: bigint,
  ttlSecs: number
): Promise<number> {
  // TODO: Build Soroban transaction calling smart_account.propose_multisig_tx
  console.log('[SmartAccount] proposeMultisigTx', {
    proposer,
    recipient,
    token,
    amount: amount.toString(),
    ttlSecs,
  });
  return 0;
}

export async function approveMultisigTx(
  signer: string,
  owner: string,
  proposalId: number
): Promise<void> {
  // TODO: Build Soroban transaction calling smart_account.approve_multisig_tx
  console.log('[SmartAccount] approveMultisigTx', {
    signer,
    owner,
    proposalId,
  });
}

export async function getMultisigProposal(
  proposalId: number
): Promise<MultisigProposal | null> {
  // TODO: Soroban query smart_account.get_multisig_proposal
  console.log('[SmartAccount] getMultisigProposal', { proposalId });
  return null;
}

export async function getMultisigApprovals(
  proposalId: number
): Promise<string[]> {
  // TODO: Soroban query smart_account.get_multisig_approvals
  console.log('[SmartAccount] getMultisigApprovals', { proposalId });
  return [];
}

// ── Scheduled Transfers ─────────────────────────────────────────

export async function scheduleTransfer(
  owner: string,
  recipient: string,
  token: string,
  amount: bigint,
  executeAfter: number
): Promise<number> {
  // TODO: Build Soroban transaction calling smart_account.schedule_transfer
  console.log('[SmartAccount] scheduleTransfer', {
    owner,
    recipient,
    token,
    amount: amount.toString(),
    executeAfter,
  });
  return 0;
}

export async function executeScheduledTransfer(
  caller: string,
  transferId: number
): Promise<void> {
  // TODO: Build Soroban transaction calling smart_account.execute_scheduled
  console.log('[SmartAccount] executeScheduledTransfer', {
    caller,
    transferId,
  });
}

export async function cancelScheduledTransfer(
  owner: string,
  transferId: number
): Promise<void> {
  // TODO: Build Soroban transaction calling smart_account.cancel_scheduled
  console.log('[SmartAccount] cancelScheduledTransfer', { owner, transferId });
}

export async function getScheduledTransfer(
  transferId: number
): Promise<ScheduledTransfer | null> {
  // TODO: Soroban query smart_account.get_scheduled
  console.log('[SmartAccount] getScheduledTransfer', { transferId });
  return null;
}

export async function getScheduledCount(): Promise<number> {
  // TODO: Soroban query smart_account.scheduled_count
  return 0;
}
