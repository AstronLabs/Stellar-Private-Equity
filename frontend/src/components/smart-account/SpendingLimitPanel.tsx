'use client';

import React, { useState } from 'react';
import type { SpendingLimit } from '@/types/smart-account.types';
import styles from './SmartAccount.module.css';

interface SpendingLimitPanelProps {
  limits: SpendingLimit[];
  onSetLimit: (data: { token: string; maxAmount: string; periodSeconds: number }) => void;
  loading?: boolean;
}

const PERIOD_OPTIONS = [
  { label: 'Daily', value: 86_400 },
  { label: 'Weekly', value: 604_800 },
  { label: 'Monthly', value: 2_592_000 },
];

export default function SpendingLimitPanel({
  limits,
  onSetLimit,
  loading,
}: SpendingLimitPanelProps) {
  const [token, setToken] = useState('');
  const [maxAmount, setMaxAmount] = useState('');
  const [period, setPeriod] = useState(86_400);

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    onSetLimit({ token, maxAmount, periodSeconds: period });
  };

  return (
    <div className={styles.panel} id="spending-limit-panel">
      <h3 className={styles.panelTitle}>
        <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
          <rect x="1" y="4" width="22" height="16" rx="2" />
          <line x1="1" y1="10" x2="23" y2="10" />
        </svg>
        Spending Limits
      </h3>

      {/* Existing limits */}
      {limits.length > 0 && (
        <div className={styles.limitsList}>
          {limits.map((limit, idx) => {
            const used = Number(limit.currentSpent);
            const max = Number(limit.maxAmount);
            const pct = max > 0 ? Math.min((used / max) * 100, 100) : 0;
            const periodLabel = PERIOD_OPTIONS.find(
              (p) => p.value === limit.periodSeconds
            )?.label ?? `${limit.periodSeconds}s`;

            return (
              <div key={idx} className={styles.limitItem} id={`spending-limit-${idx}`}>
                <div className={styles.limitHeader}>
                  <span className={styles.mono}>
                    {limit.token.slice(0, 8)}…{limit.token.slice(-4)}
                  </span>
                  <span className={styles.badge}>{periodLabel}</span>
                </div>
                <div className={styles.progressBar}>
                  <div
                    className={`${styles.progressFill} ${pct >= 90 ? styles.progressDanger : pct >= 70 ? styles.progressWarn : ''}`}
                    style={{ width: `${pct}%` }}
                  />
                </div>
                <div className={styles.limitFooter}>
                  <span>{(used / 1e7).toLocaleString()} XLM spent</span>
                  <span>{(max / 1e7).toLocaleString()} XLM limit</span>
                </div>
              </div>
            );
          })}
        </div>
      )}

      {limits.length === 0 && (
        <p className={styles.emptyState}>No spending limits configured yet.</p>
      )}

      {/* Set new limit */}
      <form onSubmit={handleSubmit} className={styles.inlineForm}>
        <div className={styles.formRow}>
          <input
            className={styles.formInput}
            type="text"
            placeholder="Token address (G…)"
            value={token}
            onChange={(e) => setToken(e.target.value)}
            required
            id="limit-token-input"
          />
          <input
            className={styles.formInput}
            type="number"
            step="0.0000001"
            min="0"
            placeholder="Max amount"
            value={maxAmount}
            onChange={(e) => setMaxAmount(e.target.value)}
            required
            id="limit-amount-input"
          />
          <select
            className={styles.formSelect}
            value={period}
            onChange={(e) => setPeriod(parseInt(e.target.value))}
            id="limit-period-select"
          >
            {PERIOD_OPTIONS.map((opt) => (
              <option key={opt.value} value={opt.value}>
                {opt.label}
              </option>
            ))}
          </select>
        </div>
        <button
          type="submit"
          className={styles.btnSecondary}
          disabled={loading}
          id="submit-spending-limit"
        >
          {loading ? 'Setting…' : 'Set Limit'}
        </button>
      </form>
    </div>
  );
}
