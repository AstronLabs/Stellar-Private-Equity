'use client';

import React from 'react';
import type { ScheduledTransfer } from '@/types/smart-account.types';
import styles from './SmartAccount.module.css';

interface ScheduledTransferCardProps {
  transfer: ScheduledTransfer;
  onExecute?: (id: number) => void;
  onCancel?: (id: number) => void;
}

export default function ScheduledTransferCard({
  transfer,
  onExecute,
  onCancel,
}: ScheduledTransferCardProps) {
  const executeDate = new Date(transfer.executeAfter * 1000);
  const now = Date.now();
  const isReady = now >= transfer.executeAfter * 1000;
  const timeRemaining = transfer.executeAfter * 1000 - now;

  const formatCountdown = (ms: number) => {
    if (ms <= 0) return 'Ready';
    const days = Math.floor(ms / 86_400_000);
    const hours = Math.floor((ms % 86_400_000) / 3_600_000);
    const mins = Math.floor((ms % 3_600_000) / 60_000);
    if (days > 0) return `${days}d ${hours}h`;
    if (hours > 0) return `${hours}h ${mins}m`;
    return `${mins}m`;
  };

  return (
    <div
      className={`${styles.card} ${transfer.executed ? styles.cardCompleted : ''}`}
      id={`scheduled-transfer-${transfer.id}`}
    >
      <div className={styles.cardHeader}>
        <div className={styles.cardIcon}>
          <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
            <circle cx="12" cy="12" r="10" />
            <polyline points="12,6 12,12 16,14" />
          </svg>
        </div>
        <span className={styles.badge}>
          {transfer.executed ? 'Completed' : isReady ? 'Ready' : formatCountdown(timeRemaining)}
        </span>
      </div>

      <div className={styles.cardBody}>
        <p className={styles.amountLarge}>
          {(Number(transfer.amount) / 1e7).toLocaleString()}{' '}
          <span className={styles.unit}>XLM</span>
        </p>
        <p className={styles.label}>
          To:{' '}
          <span className={styles.mono}>
            {transfer.recipient.slice(0, 8)}…{transfer.recipient.slice(-4)}
          </span>
        </p>
        <p className={styles.label}>
          Scheduled: {executeDate.toLocaleDateString()} at{' '}
          {executeDate.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })}
        </p>
      </div>

      {!transfer.executed && (
        <div className={styles.cardActions}>
          {isReady && onExecute && (
            <button
              className={styles.btnPrimary}
              onClick={() => onExecute(transfer.id)}
              id={`execute-scheduled-${transfer.id}`}
            >
              Execute Now
            </button>
          )}
          {onCancel && (
            <button
              className={styles.btnDanger}
              onClick={() => onCancel(transfer.id)}
              id={`cancel-scheduled-${transfer.id}`}
            >
              Cancel
            </button>
          )}
        </div>
      )}
    </div>
  );
}
