'use client';

import React from 'react';
import type { MultisigProposal } from '@/types/smart-account.types';
import styles from './SmartAccount.module.css';

interface MultisigProposalCardProps {
  proposal: MultisigProposal;
  threshold: number;
  currentUser?: string;
  onApprove?: (id: number) => void;
}

export default function MultisigProposalCard({
  proposal,
  threshold,
  currentUser,
  onApprove,
}: MultisigProposalCardProps) {
  const expiresDate = new Date(proposal.expiresAt * 1000);
  const isExpired = Date.now() > proposal.expiresAt * 1000;
  const hasApproved = currentUser
    ? proposal.approvals.includes(currentUser)
    : false;
  const approvalsNeeded = Math.max(0, threshold - proposal.approvals.length);

  return (
    <div
      className={`${styles.card} ${proposal.executed ? styles.cardCompleted : isExpired ? styles.cardInactive : ''}`}
      id={`multisig-proposal-${proposal.id}`}
    >
      <div className={styles.cardHeader}>
        <div className={styles.cardIcon}>
          <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
            <path d="M16 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2" />
            <circle cx="8.5" cy="7" r="4" />
            <line x1="20" y1="8" x2="20" y2="14" />
            <line x1="23" y1="11" x2="17" y2="11" />
          </svg>
        </div>
        <span className={styles.badge}>
          {proposal.executed ? 'Executed' : isExpired ? 'Expired' : `${proposal.approvals.length}/${threshold} signed`}
        </span>
      </div>

      <div className={styles.cardBody}>
        <p className={styles.amountLarge}>
          {(Number(proposal.amount) / 1e7).toLocaleString()}{' '}
          <span className={styles.unit}>XLM</span>
        </p>
        <p className={styles.label}>
          To:{' '}
          <span className={styles.mono}>
            {proposal.recipient.slice(0, 8)}…{proposal.recipient.slice(-4)}
          </span>
        </p>

        {/* Approval progress */}
        <div className={styles.approvalDots}>
          {Array.from({ length: threshold }).map((_, i) => (
            <span
              key={i}
              className={`${styles.approvalDot} ${i < proposal.approvals.length ? styles.approvalDotFilled : ''}`}
            />
          ))}
        </div>

        <div className={styles.metaRow}>
          <span className={styles.metaItem}>
            <span className={styles.metaLabel}>Proposer</span>
            <span className={`${styles.metaValue} ${styles.mono}`}>
              {proposal.proposer.slice(0, 6)}…
            </span>
          </span>
          <span className={styles.metaItem}>
            <span className={styles.metaLabel}>Expires</span>
            <span className={styles.metaValue}>
              {isExpired ? 'Expired' : expiresDate.toLocaleDateString()}
            </span>
          </span>
        </div>
      </div>

      {!proposal.executed && !isExpired && !hasApproved && onApprove && (
        <div className={styles.cardActions}>
          <button
            className={styles.btnPrimary}
            onClick={() => onApprove(proposal.id)}
            id={`approve-multisig-${proposal.id}`}
          >
            Approve ({approvalsNeeded} more needed)
          </button>
        </div>
      )}

      {hasApproved && !proposal.executed && (
        <div className={styles.cardFooter}>
          <span className={styles.textSuccess}>✓ You approved this</span>
        </div>
      )}
    </div>
  );
}
