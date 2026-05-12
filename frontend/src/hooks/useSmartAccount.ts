'use client';

import { useState, useCallback } from 'react';
import type {
  RecurringPayment,
  RecurringInterval,
  SpendingLimit,
  MultisigProposal,
  ScheduledTransfer,
} from '@/types/smart-account.types';
import * as smartAccountService from '@/services/smart-account.service';

interface UseSmartAccountReturn {
  // State
  loading: boolean;
  error: string | null;

  // Recurring Payments
  createRecurring: (
    owner: string,
    recipient: string,
    token: string,
    amount: bigint,
    interval: RecurringInterval,
    maxExecutions: number
  ) => Promise<number | null>;
  executeRecurring: (caller: string, paymentId: number) => Promise<boolean>;
  cancelRecurring: (owner: string, paymentId: number) => Promise<boolean>;
  fetchRecurring: (paymentId: number) => Promise<RecurringPayment | null>;

  // Spending Limits
  setSpendingLimit: (
    owner: string,
    token: string,
    maxAmount: bigint,
    periodSeconds: number
  ) => Promise<boolean>;
  checkSpending: (
    owner: string,
    token: string,
    amount: bigint
  ) => Promise<boolean>;
  fetchSpendingLimit: (
    owner: string,
    token: string
  ) => Promise<SpendingLimit | null>;

  // Multisig
  setupMultisig: (
    owner: string,
    signers: string[],
    threshold: number
  ) => Promise<boolean>;
  proposeMultisigTx: (
    proposer: string,
    recipient: string,
    token: string,
    amount: bigint,
    ttlSecs: number
  ) => Promise<number | null>;
  approveMultisigTx: (
    signer: string,
    owner: string,
    proposalId: number
  ) => Promise<boolean>;
  fetchMultisigProposal: (
    proposalId: number
  ) => Promise<MultisigProposal | null>;

  // Scheduled Transfers
  scheduleTransfer: (
    owner: string,
    recipient: string,
    token: string,
    amount: bigint,
    executeAfter: number
  ) => Promise<number | null>;
  executeScheduled: (caller: string, transferId: number) => Promise<boolean>;
  cancelScheduled: (owner: string, transferId: number) => Promise<boolean>;
  fetchScheduled: (transferId: number) => Promise<ScheduledTransfer | null>;

  // Utility
  clearError: () => void;
}

export function useSmartAccount(): UseSmartAccountReturn {
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const clearError = useCallback(() => setError(null), []);

  // ── Helpers ─────────────────────────────────────────────────
  async function withLoading<T>(fn: () => Promise<T>): Promise<T | null> {
    setLoading(true);
    setError(null);
    try {
      const result = await fn();
      return result;
    } catch (err: unknown) {
      const message = err instanceof Error ? err.message : 'Unknown error';
      setError(message);
      return null;
    } finally {
      setLoading(false);
    }
  }

  // ── Recurring Payments ──────────────────────────────────────
  const createRecurring = useCallback(
    async (
      owner: string,
      recipient: string,
      token: string,
      amount: bigint,
      interval: RecurringInterval,
      maxExecutions: number
    ) =>
      withLoading(() =>
        smartAccountService.createRecurringPayment(
          owner,
          recipient,
          token,
          amount,
          interval,
          maxExecutions
        )
      ),
    []
  );

  const executeRecurring = useCallback(
    async (caller: string, paymentId: number) => {
      const result = await withLoading(() =>
        smartAccountService.executeRecurringPayment(caller, paymentId)
      );
      return result !== null;
    },
    []
  );

  const cancelRecurring = useCallback(
    async (owner: string, paymentId: number) => {
      const result = await withLoading(() =>
        smartAccountService.cancelRecurringPayment(owner, paymentId)
      );
      return result !== null;
    },
    []
  );

  const fetchRecurring = useCallback(
    async (paymentId: number) =>
      withLoading(() =>
        smartAccountService.getRecurringPayment(paymentId)
      ),
    []
  );

  // ── Spending Limits ─────────────────────────────────────────
  const setSpendingLimitFn = useCallback(
    async (
      owner: string,
      token: string,
      maxAmount: bigint,
      periodSeconds: number
    ) => {
      const result = await withLoading(() =>
        smartAccountService.setSpendingLimit(owner, token, maxAmount, periodSeconds)
      );
      return result !== null;
    },
    []
  );

  const checkSpendingFn = useCallback(
    async (owner: string, token: string, amount: bigint) => {
      const result = await withLoading(() =>
        smartAccountService.checkSpending(owner, token, amount)
      );
      return result ?? false;
    },
    []
  );

  const fetchSpendingLimit = useCallback(
    async (owner: string, token: string) =>
      withLoading(() =>
        smartAccountService.getSpendingLimit(owner, token)
      ),
    []
  );

  // ── Multisig ────────────────────────────────────────────────
  const setupMultisigFn = useCallback(
    async (owner: string, signers: string[], threshold: number) => {
      const result = await withLoading(() =>
        smartAccountService.setupMultisig(owner, signers, threshold)
      );
      return result !== null;
    },
    []
  );

  const proposeMultisigTxFn = useCallback(
    async (
      proposer: string,
      recipient: string,
      token: string,
      amount: bigint,
      ttlSecs: number
    ) =>
      withLoading(() =>
        smartAccountService.proposeMultisigTx(proposer, recipient, token, amount, ttlSecs)
      ),
    []
  );

  const approveMultisigTxFn = useCallback(
    async (signer: string, owner: string, proposalId: number) => {
      const result = await withLoading(() =>
        smartAccountService.approveMultisigTx(signer, owner, proposalId)
      );
      return result !== null;
    },
    []
  );

  const fetchMultisigProposal = useCallback(
    async (proposalId: number) =>
      withLoading(() =>
        smartAccountService.getMultisigProposal(proposalId)
      ),
    []
  );

  // ── Scheduled Transfers ─────────────────────────────────────
  const scheduleTransferFn = useCallback(
    async (
      owner: string,
      recipient: string,
      token: string,
      amount: bigint,
      executeAfter: number
    ) =>
      withLoading(() =>
        smartAccountService.scheduleTransfer(owner, recipient, token, amount, executeAfter)
      ),
    []
  );

  const executeScheduledFn = useCallback(
    async (caller: string, transferId: number) => {
      const result = await withLoading(() =>
        smartAccountService.executeScheduledTransfer(caller, transferId)
      );
      return result !== null;
    },
    []
  );

  const cancelScheduledFn = useCallback(
    async (owner: string, transferId: number) => {
      const result = await withLoading(() =>
        smartAccountService.cancelScheduledTransfer(owner, transferId)
      );
      return result !== null;
    },
    []
  );

  const fetchScheduled = useCallback(
    async (transferId: number) =>
      withLoading(() =>
        smartAccountService.getScheduledTransfer(transferId)
      ),
    []
  );

  return {
    loading,
    error,
    createRecurring,
    executeRecurring,
    cancelRecurring,
    fetchRecurring,
    setSpendingLimit: setSpendingLimitFn,
    checkSpending: checkSpendingFn,
    fetchSpendingLimit,
    setupMultisig: setupMultisigFn,
    proposeMultisigTx: proposeMultisigTxFn,
    approveMultisigTx: approveMultisigTxFn,
    fetchMultisigProposal,
    scheduleTransfer: scheduleTransferFn,
    executeScheduled: executeScheduledFn,
    cancelScheduled: cancelScheduledFn,
    fetchScheduled,
    clearError,
  };
}
