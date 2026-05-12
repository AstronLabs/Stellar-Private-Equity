import type { Metadata } from 'next';
import SmartAccountDashboard from '@/components/smart-account/SmartAccountDashboard';
import styles from './page.module.css';

export const metadata: Metadata = {
  title: 'Smart Accounts | PoolSafe',
  description:
    'Automate your insurance contributions with Stellar Smart Accounts — recurring payments, spending limits, multisig protection, and scheduled transfers.',
};

export default function SmartAccountPage() {
  return (
    <main className={styles.main}>
      <SmartAccountDashboard />
    </main>
  );
}
