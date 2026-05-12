'use client';

import React, { useState } from 'react';
import styles from './SmartAccount.module.css';

interface ScheduledTransferFormProps {
  onSubmit: (data: {
    recipient: string;
    amount: string;
    executeAfter: number;
  }) => void;
  loading?: boolean;
}

export default function ScheduledTransferForm({
  onSubmit,
  loading,
}: ScheduledTransferFormProps) {
  const [recipient, setRecipient] = useState('');
  const [amount, setAmount] = useState('');
  const [date, setDate] = useState('');
  const [time, setTime] = useState('');

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    const datetime = new Date(`${date}T${time}`);
    const executeAfter = Math.floor(datetime.getTime() / 1000);
    onSubmit({ recipient, amount, executeAfter });
  };

  // Minimum date is tomorrow
  const minDate = new Date(Date.now() + 86_400_000).toISOString().split('T')[0];

  return (
    <form onSubmit={handleSubmit} className={styles.form} id="scheduled-transfer-form">
      <h3 className={styles.formTitle}>Schedule Transfer</h3>

      <div className={styles.formGroup}>
        <label className={styles.formLabel} htmlFor="schedule-recipient">
          Recipient Address
        </label>
        <input
          id="schedule-recipient"
          className={styles.formInput}
          type="text"
          placeholder="G..."
          value={recipient}
          onChange={(e) => setRecipient(e.target.value)}
          required
        />
      </div>

      <div className={styles.formGroup}>
        <label className={styles.formLabel} htmlFor="schedule-amount">
          Amount (XLM)
        </label>
        <input
          id="schedule-amount"
          className={styles.formInput}
          type="number"
          step="0.0000001"
          min="0"
          placeholder="500.00"
          value={amount}
          onChange={(e) => setAmount(e.target.value)}
          required
        />
      </div>

      <div className={styles.formRow}>
        <div className={styles.formGroup}>
          <label className={styles.formLabel} htmlFor="schedule-date">
            Date
          </label>
          <input
            id="schedule-date"
            className={styles.formInput}
            type="date"
            min={minDate}
            value={date}
            onChange={(e) => setDate(e.target.value)}
            required
          />
        </div>
        <div className={styles.formGroup}>
          <label className={styles.formLabel} htmlFor="schedule-time">
            Time
          </label>
          <input
            id="schedule-time"
            className={styles.formInput}
            type="time"
            value={time}
            onChange={(e) => setTime(e.target.value)}
            required
          />
        </div>
      </div>

      <button
        type="submit"
        className={styles.btnPrimary}
        disabled={loading}
        id="submit-scheduled"
      >
        {loading ? 'Scheduling…' : 'Schedule Transfer'}
      </button>
    </form>
  );
}
