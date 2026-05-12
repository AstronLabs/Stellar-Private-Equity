'use client';

import React, { useState } from 'react';
import styles from './SmartAccount.module.css';

interface MultisigSetupProps {
  currentSigners: string[];
  currentThreshold: number;
  onSetup: (data: { signers: string[]; threshold: number }) => void;
  loading?: boolean;
}

export default function MultisigSetup({
  currentSigners,
  currentThreshold,
  onSetup,
  loading,
}: MultisigSetupProps) {
  const [signers, setSigners] = useState<string[]>(
    currentSigners.length > 0 ? currentSigners : ['']
  );
  const [threshold, setThreshold] = useState(currentThreshold || 1);

  const addSigner = () => {
    if (signers.length < 10) {
      setSigners([...signers, '']);
    }
  };

  const removeSigner = (idx: number) => {
    const updated = signers.filter((_, i) => i !== idx);
    setSigners(updated);
    if (threshold > updated.length) {
      setThreshold(updated.length);
    }
  };

  const updateSigner = (idx: number, value: string) => {
    const updated = [...signers];
    updated[idx] = value;
    setSigners(updated);
  };

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    const validSigners = signers.filter((s) => s.trim().length > 0);
    onSetup({ signers: validSigners, threshold });
  };

  return (
    <div className={styles.panel} id="multisig-setup-panel">
      <h3 className={styles.panelTitle}>
        <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
          <path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z" />
        </svg>
        Multisig Protection
      </h3>

      {currentSigners.length > 0 && (
        <div className={styles.currentConfig}>
          <span className={styles.configLabel}>Current:</span>
          <span className={styles.configValue}>
            {currentThreshold}-of-{currentSigners.length} signers
          </span>
        </div>
      )}

      <form onSubmit={handleSubmit} className={styles.form}>
        <div className={styles.signersSection}>
          <label className={styles.formLabel}>Signers</label>
          {signers.map((signer, idx) => (
            <div key={idx} className={styles.signerRow}>
              <input
                className={styles.formInput}
                type="text"
                placeholder={`Signer ${idx + 1} address (G…)`}
                value={signer}
                onChange={(e) => updateSigner(idx, e.target.value)}
                required
                id={`signer-input-${idx}`}
              />
              {signers.length > 1 && (
                <button
                  type="button"
                  className={styles.btnIcon}
                  onClick={() => removeSigner(idx)}
                  aria-label="Remove signer"
                >
                  ×
                </button>
              )}
            </div>
          ))}
          {signers.length < 10 && (
            <button
              type="button"
              className={styles.btnGhost}
              onClick={addSigner}
              id="add-signer-btn"
            >
              + Add Signer
            </button>
          )}
        </div>

        <div className={styles.formGroup}>
          <label className={styles.formLabel} htmlFor="multisig-threshold">
            Required Approvals (Threshold)
          </label>
          <select
            id="multisig-threshold"
            className={styles.formSelect}
            value={threshold}
            onChange={(e) => setThreshold(parseInt(e.target.value))}
          >
            {signers.filter((s) => s.trim().length > 0).map((_, idx) => (
              <option key={idx + 1} value={idx + 1}>
                {idx + 1} of {signers.filter((s) => s.trim().length > 0).length}
              </option>
            ))}
          </select>
        </div>

        <button
          type="submit"
          className={styles.btnPrimary}
          disabled={loading}
          id="submit-multisig-setup"
        >
          {loading ? 'Configuring…' : 'Configure Multisig'}
        </button>
      </form>
    </div>
  );
}
