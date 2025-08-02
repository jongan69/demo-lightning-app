import React, { useState } from 'react';
import { ScrollView, View, Text, StyleSheet, TouchableOpacity } from 'react-native';
import { StatusBar } from 'expo-status-bar';
import { SafeAreaView } from 'react-native-safe-area-context';
import { Send, Download, Zap, Bitcoin, ArrowUpRight, ArrowDownLeft, Coins, Clock } from 'lucide-react-native';

const mockTransactions = [
  {
    id: 'tx_001',
    type: 'lightning_payment',
    direction: 'sent',
    amount: 0.00025000,
    fee: 0.00000001,
    status: 'confirmed',
    timestamp: '2024-01-15T10:30:00Z',
    description: 'Coffee payment',
    recipient: 'Lightning Cafe'
  },
  {
    id: 'tx_002',
    type: 'asset_transfer',
    direction: 'received',
    amount: 100.0,
    asset: 'LGOLD',
    status: 'confirmed',
    timestamp: '2024-01-15T09:15:00Z',
    description: 'Asset transfer',
    sender: 'Exchange Wallet'
  },
  {
    id: 'tx_003',
    type: 'channel_open',
    direction: 'neutral',
    amount: 0.01000000,
    status: 'confirmed',
    timestamp: '2024-01-14T16:45:00Z',
    description: 'Channel opened',
    peer: 'Lightning Router'
  },
  {
    id: 'tx_004',
    type: 'onchain_payment',
    direction: 'sent',
    amount: 0.00500000,
    fee: 0.00000150,
    status: 'confirmed',
    timestamp: '2024-01-14T14:20:00Z',
    description: 'On-chain transfer',
    recipient: 'Cold Storage'
  },
  {
    id: 'tx_005',
    type: 'asset_issuance',
    direction: 'neutral',
    amount: 1000.0,
    asset: 'TSHARE',
    status: 'confirmed',
    timestamp: '2024-01-13T11:00:00Z',
    description: 'Asset issuance',
    details: 'Created new asset'
  }
];

export default function HistoryScreen() {
  const [filter, setFilter] = useState('all');

  const formatTime = (timestamp: string) => {
    const date = new Date(timestamp);
    const now = new Date();
    const diffMs = now.getTime() - date.getTime();
    const diffHours = Math.floor(diffMs / (1000 * 60 * 60));
    const diffDays = Math.floor(diffHours / 24);
    
    if (diffHours < 1) return 'Just now';
    if (diffHours < 24) return `${diffHours}h ago`;
    if (diffDays < 7) return `${diffDays}d ago`;
    return date.toLocaleDateString();
  };

  const getTransactionIcon = (type: string, direction: string) => {
    if (type === 'lightning_payment') {
      return direction === 'sent' ? 
        <ArrowUpRight size={16} color="#0099FF" /> : 
        <ArrowDownLeft size={16} color="#00FF88" />;
    }
    if (type === 'onchain_payment') {
      return direction === 'sent' ? 
        <ArrowUpRight size={16} color="#F7931A" /> : 
        <ArrowDownLeft size={16} color="#F7931A" />;
    }
    if (type === 'asset_transfer' || type === 'asset_issuance') {
      return <Coins size={16} color="#9333EA" />;
    }
    return <Zap size={16} color="#666" />;
  };

  const filteredTransactions = filter === 'all' ? mockTransactions : 
    mockTransactions.filter(tx => {
      if (filter === 'lightning') return tx.type.includes('lightning');
      if (filter === 'onchain') return tx.type.includes('onchain');
      if (filter === 'assets') return tx.type.includes('asset');
      if (filter === 'channels') return tx.type.includes('channel');
      return true;
    });

  return (
    <SafeAreaView style={styles.container}>
      <StatusBar style="light" />
      <ScrollView style={styles.scrollView} showsVerticalScrollIndicator={false}>
        <View style={styles.header}>
          <Text style={styles.title}>Transaction History</Text>
        </View>

        {/* Filter Tabs */}
        <ScrollView 
          horizontal 
          showsHorizontalScrollIndicator={false} 
          style={styles.filterContainer}
          contentContainerStyle={styles.filterContent}
        >
          {['all', 'lightning', 'onchain', 'assets', 'channels'].map((filterType) => (
            <TouchableOpacity
              key={filterType}
              style={[styles.filterTab, filter === filterType && styles.activeFilterTab]}
              onPress={() => setFilter(filterType)}
            >
              <Text style={[styles.filterText, filter === filterType && styles.activeFilterText]}>
                {filterType.charAt(0).toUpperCase() + filterType.slice(1)}
              </Text>
            </TouchableOpacity>
          ))}
        </ScrollView>

        {/* Transaction List */}
        <View style={styles.transactionsList}>
          {filteredTransactions.map((transaction, index) => (
            <TouchableOpacity key={transaction.id} style={styles.transactionCard}>
              <View style={styles.transactionHeader}>
                <View style={styles.transactionIcon}>
                  {getTransactionIcon(transaction.type, transaction.direction)}
                </View>
                <View style={styles.transactionInfo}>
                  <Text style={styles.transactionTitle}>
                    {transaction.type === 'lightning_payment' ? 'Lightning Payment' :
                     transaction.type === 'onchain_payment' ? 'On-chain Payment' :
                     transaction.type === 'asset_transfer' ? 'Asset Transfer' :
                     transaction.type === 'asset_issuance' ? 'Asset Issuance' :
                     'Channel Operation'}
                  </Text>
                  <Text style={styles.transactionSubtitle}>
                    {transaction.description}
                  </Text>
                  <Text style={styles.transactionTime}>
                    {formatTime(transaction.timestamp)}
                  </Text>
                </View>
                <View style={styles.transactionAmount}>
                  <Text style={[
                    styles.amountText,
                    transaction.direction === 'sent' ? styles.sentAmount :
                    transaction.direction === 'received' ? styles.receivedAmount :
                    styles.neutralAmount
                  ]}>
                    {transaction.direction === 'sent' ? '-' : 
                     transaction.direction === 'received' ? '+' : ''}
                    {transaction.amount.toFixed(transaction.asset ? 2 : 8)}
                    {transaction.asset ? ` ${transaction.asset}` : ' BTC'}
                  </Text>
                  {transaction.fee && (
                    <Text style={styles.feeText}>
                      Fee: {transaction.fee.toFixed(8)} BTC
                    </Text>
                  )}
                  <View style={[
                    styles.statusBadge,
                    transaction.status === 'confirmed' ? styles.confirmedBadge :
                    transaction.status === 'pending' ? styles.pendingBadge :
                    styles.failedBadge
                  ]}>
                    <Text style={[
                      styles.statusText,
                      transaction.status === 'confirmed' ? styles.confirmedText :
                      transaction.status === 'pending' ? styles.pendingText :
                      styles.failedText
                    ]}>
                      {transaction.status}
                    </Text>
                  </View>
                </View>
              </View>
              
              {(transaction.recipient || transaction.sender || transaction.peer) && (
                <View style={styles.transactionDetails}>
                  <Text style={styles.detailLabel}>
                    {transaction.recipient ? 'To: ' : transaction.sender ? 'From: ' : 'Peer: '}
                  </Text>
                  <Text style={styles.detailValue}>
                    {transaction.recipient || transaction.sender || transaction.peer}
                  </Text>
                </View>
              )}
            </TouchableOpacity>
          ))}
        </View>

        {/* Load More */}
        <TouchableOpacity style={styles.loadMoreButton}>
          <Clock size={16} color="#F7931A" />
          <Text style={styles.loadMoreText}>Load More</Text>
        </TouchableOpacity>
      </ScrollView>
    </SafeAreaView>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: '#0a0a0a',
  },
  scrollView: {
    flex: 1,
    padding: 20,
  },
  header: {
    marginBottom: 24,
  },
  title: {
    fontSize: 28,
    fontWeight: 'bold',
    color: '#fff',
  },
  filterContainer: {
    marginBottom: 20,
  },
  filterContent: {
    paddingHorizontal: 0,
  },
  filterTab: {
    backgroundColor: 'rgba(255, 255, 255, 0.05)',
    borderRadius: 20,
    paddingHorizontal: 16,
    paddingVertical: 8,
    marginRight: 12,
    borderWidth: 1,
    borderColor: 'rgba(255, 255, 255, 0.1)',
  },
  activeFilterTab: {
    backgroundColor: 'rgba(247, 147, 26, 0.2)',
    borderColor: '#F7931A',
  },
  filterText: {
    fontSize: 14,
    fontWeight: '600',
    color: '#666',
  },
  activeFilterText: {
    color: '#F7931A',
  },
  transactionsList: {
    marginBottom: 20,
  },
  transactionCard: {
    backgroundColor: 'rgba(255, 255, 255, 0.05)',
    borderRadius: 16,
    padding: 16,
    marginBottom: 12,
    borderWidth: 1,
    borderColor: 'rgba(255, 255, 255, 0.1)',
  },
  transactionHeader: {
    flexDirection: 'row',
    alignItems: 'flex-start',
  },
  transactionIcon: {
    width: 32,
    height: 32,
    backgroundColor: 'rgba(255, 255, 255, 0.1)',
    borderRadius: 16,
    alignItems: 'center',
    justifyContent: 'center',
    marginRight: 12,
  },
  transactionInfo: {
    flex: 1,
  },
  transactionTitle: {
    fontSize: 16,
    fontWeight: '600',
    color: '#fff',
    marginBottom: 2,
  },
  transactionSubtitle: {
    fontSize: 14,
    color: '#999',
    marginBottom: 4,
  },
  transactionTime: {
    fontSize: 12,
    color: '#666',
  },
  transactionAmount: {
    alignItems: 'flex-end',
  },
  amountText: {
    fontSize: 16,
    fontWeight: 'bold',
    marginBottom: 2,
  },
  sentAmount: {
    color: '#FF4444',
  },
  receivedAmount: {
    color: '#00FF88',
  },
  neutralAmount: {
    color: '#F7931A',
  },
  feeText: {
    fontSize: 12,
    color: '#666',
    marginBottom: 4,
  },
  statusBadge: {
    paddingHorizontal: 8,
    paddingVertical: 2,
    borderRadius: 8,
  },
  confirmedBadge: {
    backgroundColor: 'rgba(0, 255, 136, 0.2)',
  },
  pendingBadge: {
    backgroundColor: 'rgba(255, 165, 0, 0.2)',
  },
  failedBadge: {
    backgroundColor: 'rgba(255, 68, 68, 0.2)',
  },
  statusText: {
    fontSize: 10,
    fontWeight: '600',
    textTransform: 'uppercase',
  },
  confirmedText: {
    color: '#00FF88',
  },
  pendingText: {
    color: '#FFA500',
  },
  failedText: {
    color: '#FF4444',
  },
  transactionDetails: {
    flexDirection: 'row',
    marginTop: 8,
    paddingTop: 8,
    borderTopWidth: 1,
    borderTopColor: 'rgba(255, 255, 255, 0.1)',
  },
  detailLabel: {
    fontSize: 12,
    color: '#666',
    marginRight: 4,
  },
  detailValue: {
    fontSize: 12,
    color: '#999',
    flex: 1,
  },
  loadMoreButton: {
    backgroundColor: 'rgba(247, 147, 26, 0.2)',
    borderRadius: 12,
    paddingVertical: 16,
    flexDirection: 'row',
    alignItems: 'center',
    justifyContent: 'center',
    borderWidth: 1,
    borderColor: 'rgba(247, 147, 26, 0.3)',
  },
  loadMoreText: {
    fontSize: 14,
    fontWeight: '600',
    color: '#F7931A',
    marginLeft: 8,
  },
});