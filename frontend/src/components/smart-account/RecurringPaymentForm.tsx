'use client';

import React, { useState } from 'react';
import { RecurringInterval } from '@/types/smart-account.types';
import styles from './SmartAccount.module.css';

interface RecurringPaymentFormProps {
  onSubmit: (data: {
    recipient: string;
    amount: string;
    interval: RecurringInterval;
    maxExecutions: number;
  }) => void;
  loading?: boolean;
}

export default function RecurringPaymentForm({
  onSubmit,
  loading,
}: RecurringPaymentFormProps) {
  const [recipient, setRecipient] = useState('');
  const [amount, setAmount] = useState('');
  const [interval, setInterval] = useState<RecurringInterval>(RecurringInterval.Weekly);
  const [maxExecutions, setMaxExecutions] = useState(0);

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    onSubmit({ recipient, amount, interval, maxExecutions });
  };

  return (
    <form onSubmit={handleSubmit} className={styles.form} id="recurring-payment-form">
      <h3 className={styles.formTitle}>New Recurring Payment</h3>

      <div className={styles.formGroup}>
        <label className={styles.formLabel} htmlFor="recurring-recipient">
          Recipient Address
        </label>
        <input
          id="recurring-recipient"
          className={styles.formInput}
          type="text"
          placeholder="G..."
          value={recipient}
          onChange={(e) => setRecipient(e.target.value)}
          required
        />
      </div>

      <div className={styles.formRow}>
        <div className={styles.formGroup}>
          <label className={styles.formLabel} htmlFor="recurring-amount">
            Amount (XLM)
          </label>
          <input
            id="recurring-amount"
            className={styles.formInput}
            type="number"
            step="0.0000001"
            min="0"
            placeholder="100.00"
            value={amount}
            onChange={(e) => setAmount(e.target.value)}
            required
          />
        </div>

        <div className={styles.formGroup}>
          <label className={styles.formLabel} htmlFor="recurring-interval">
            Interval
          </label>
          <select
            id="recurring-interval"
            className={styles.formSelect}
            value={interval}
            onChange={(e) => setInterval(e.target.value as RecurringInterval)}
          >
            <option value={RecurringInterval.Weekly}>Weekly</option>
            <option value={RecurringInterval.Monthly}>Monthly</option>
          </select>
        </div>
      </div>

      <div className={styles.formGroup}>
        <label className={styles.formLabel} htmlFor="recurring-max">
          Max Executions <span className={styles.hint}>(0 = unlimited)</span>
        </label>
        <input
          id="recurring-max"
          className={styles.formInput}
          type="number"
          min="0"
          placeholder="52"
          value={maxExecutions}
          onChange={(e) => setMaxExecutions(parseInt(e.target.value) || 0)}
        />
      </div>

      <button
        type="submit"
        className={styles.btnPrimary}
        disabled={loading}
        id="submit-recurring"
      >
        {loading ? 'Creating…' : 'Create Recurring Payment'}
      </button>
    </form>
  );
}
