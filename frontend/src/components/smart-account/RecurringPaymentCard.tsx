'use client';

import React from 'react';
import type { RecurringPayment } from '@/types/smart-account.types';
import styles from './SmartAccount.module.css';

interface RecurringPaymentCardProps {
  payment: RecurringPayment;
  onExecute?: (id: number) => void;
  onCancel?: (id: number) => void;
}

export default function RecurringPaymentCard({
  payment,
  onExecute,
  onCancel,
}: RecurringPaymentCardProps) {
  const nextDate = new Date(payment.nextExecution * 1000);
  const isReady = Date.now() >= payment.nextExecution * 1000;
  const executionLabel =
    payment.maxExecutions === 0
      ? `${payment.totalExecuted} / ∞`
      : `${payment.totalExecuted} / ${payment.maxExecutions}`;

  return (
    <div
      className={`${styles.card} ${!payment.isActive ? styles.cardInactive : ''}`}
      id={`recurring-payment-${payment.id}`}
    >
      <div className={styles.cardHeader}>
        <div className={styles.cardIcon}>
          <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
            <path d="M17 1l4 4-4 4" />
            <path d="M3 11V9a4 4 0 0 1 4-4h14" />
            <path d="M7 23l-4-4 4-4" />
            <path d="M21 13v2a4 4 0 0 1-4 4H3" />
          </svg>
        </div>
        <span className={styles.badge}>
          {payment.interval}
        </span>
        <span className={`${styles.statusDot} ${payment.isActive ? styles.statusActive : styles.statusInactive}`} />
      </div>

      <div className={styles.cardBody}>
        <p className={styles.amountLarge}>
          {(Number(payment.amount) / 1e7).toLocaleString()} <span className={styles.unit}>XLM</span>
        </p>
        <p className={styles.label}>
          To: <span className={styles.mono}>{payment.recipient.slice(0, 8)}…{payment.recipient.slice(-4)}</span>
        </p>
        <div className={styles.metaRow}>
          <span className={styles.metaItem}>
            <span className={styles.metaLabel}>Executions</span>
            <span className={styles.metaValue}>{executionLabel}</span>
          </span>
          <span className={styles.metaItem}>
            <span className={styles.metaLabel}>Next</span>
            <span className={`${styles.metaValue} ${isReady ? styles.textReady : ''}`}>
              {isReady ? 'Ready now' : nextDate.toLocaleDateString()}
            </span>
          </span>
        </div>
      </div>

      {payment.isActive && (
        <div className={styles.cardActions}>
          {isReady && onExecute && (
            <button
              className={styles.btnPrimary}
              onClick={() => onExecute(payment.id)}
              id={`execute-recurring-${payment.id}`}
            >
              Execute Now
            </button>
          )}
          {onCancel && (
            <button
              className={styles.btnDanger}
              onClick={() => onCancel(payment.id)}
              id={`cancel-recurring-${payment.id}`}
            >
              Cancel
            </button>
          )}
        </div>
      )}
    </div>
  );
}
