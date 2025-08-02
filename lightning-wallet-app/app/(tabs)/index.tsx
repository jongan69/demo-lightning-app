import React, { useEffect } from 'react';
import { ScrollView, View, Text, StyleSheet, TouchableOpacity, RefreshControl } from 'react-native';
import { StatusBar } from 'expo-status-bar';
import { SafeAreaView } from 'react-native-safe-area-context';
import { Activity, Wifi, WifiOff, Users, Zap, TrendingUp, CircleAlert as AlertCircle } from 'lucide-react-native';
import { useLNC } from '../../hooks/useLNC';
import { useAssets } from '../../hooks/useAssets';

export default function DashboardScreen() {
  const lnc = useLNC();
  const assets = useAssets();

  // Calculate total balance in BTC
  const totalBalance = () => {
    const onchainSats = lnc.walletBalance?.totalBalance || 0;
    const lightningSats = lnc.channelBalance?.localBalance?.sat || 0;
    return (onchainSats + lightningSats) / 100000000; // Convert sats to BTC
  };

  const onchainBalance = () => {
    return (lnc.walletBalance?.totalBalance || 0) / 100000000;
  };

  const lightningBalance = () => {
    return (lnc.channelBalance?.localBalance?.sat || 0) / 100000000;
  };

  const getChannelStats = () => {
    const activeChannels = lnc.channels.filter(ch => ch.active).length;
    const totalChannels = lnc.channels.length;
    const pendingChannels = totalChannels - activeChannels;
    
    return {
      active: activeChannels,
      pending: pendingChannels,
      total: totalChannels,
    };
  };

  const onRefresh = async () => {
    await Promise.all([
      lnc.refreshData(),
      assets.refreshData(),
    ]);
  };

  return (
    <SafeAreaView style={styles.container}>
      <StatusBar style="light" />
      <ScrollView 
        style={styles.scrollView} 
        showsVerticalScrollIndicator={false}
        refreshControl={
          <RefreshControl
            refreshing={lnc.isConnecting || assets.loading}
            onRefresh={onRefresh}
            tintColor="#F7931A"
          />
        }
      >
        <View style={styles.header}>
          <Text style={styles.title}>
            {lnc.nodeInfo?.alias || 'Lightning Node'}
          </Text>
          <View style={styles.statusContainer}>
            <View style={[styles.statusDot, { 
              backgroundColor: lnc.isConnected ? '#00FF88' : '#FF4444' 
            }]} />
            <Text style={styles.statusText}>
              {lnc.isConnected ? 'ONLINE' : 'OFFLINE'}
            </Text>
          </View>
        </View>

        {/* Connection Error */}
        {lnc.error && (
          <View style={[styles.card, { backgroundColor: 'rgba(255, 68, 68, 0.1)' }]}>
            <Text style={[styles.cardTitle, { color: '#FF4444' }]}>Connection Error</Text>
            <Text style={{ color: '#FF4444', fontSize: 14 }}>{lnc.error}</Text>
          </View>
        )}

        {/* Balance Card */}
        <View style={styles.card}>
          <Text style={styles.cardTitle}>Total Balance</Text>
          <Text style={styles.balanceAmount}>
            {totalBalance().toFixed(8)} BTC
          </Text>
          <Text style={styles.balanceUsd}>
            {lnc.isConnected ? '≈ $' + (totalBalance() * 67850).toFixed(2) + ' USD' : 'Not connected'}
          </Text>
          
          <View style={styles.balanceBreakdown}>
            <View style={styles.balanceItem}>
              <Text style={styles.balanceLabel}>On-chain</Text>
              <Text style={styles.balanceValue}>
                {onchainBalance().toFixed(8)}
              </Text>
            </View>
            <View style={styles.balanceItem}>
              <Text style={styles.balanceLabel}>Lightning</Text>
              <Text style={styles.balanceValue}>
                {lightningBalance().toFixed(8)}
              </Text>
            </View>
          </View>
        </View>

        {/* Quick Actions */}
        <View style={styles.actionsContainer}>
          <TouchableOpacity style={styles.actionButton}>
            <Zap size={24} color="#F7931A" />
            <Text style={styles.actionText}>Send</Text>
          </TouchableOpacity>
          <TouchableOpacity style={styles.actionButton}>
            <TrendingUp size={24} color="#F7931A" />
            <Text style={styles.actionText}>Receive</Text>
          </TouchableOpacity>
          <TouchableOpacity style={styles.actionButton}>
            <Users size={24} color="#F7931A" />
            <Text style={styles.actionText}>Channels</Text>
          </TouchableOpacity>
        </View>

        {/* Channels Overview */}
        <View style={styles.card}>
          <View style={styles.cardHeader}>
            <Text style={styles.cardTitle}>Channels</Text>
            <TouchableOpacity>
              <Text style={styles.viewAllText}>View All</Text>
            </TouchableOpacity>
          </View>
          
          <View style={styles.channelsGrid}>
            <View style={styles.channelStat}>
              <Text style={styles.channelNumber}>{getChannelStats().active}</Text>
              <Text style={styles.channelLabel}>Active</Text>
            </View>
            <View style={styles.channelStat}>
              <Text style={styles.channelNumber}>{getChannelStats().pending}</Text>
              <Text style={styles.channelLabel}>Pending</Text>
            </View>
            <View style={styles.channelStat}>
              <Text style={styles.channelNumber}>{getChannelStats().total}</Text>
              <Text style={styles.channelLabel}>Total</Text>
            </View>
          </View>
        </View>

        {/* Recent Activity */}
        <View style={styles.card}>
          <View style={styles.cardHeader}>
            <Text style={styles.cardTitle}>Recent Activity</Text>
            <TouchableOpacity>
              <Text style={styles.viewAllText}>View All</Text>
            </TouchableOpacity>
          </View>
          
          {assets.transactions.length > 0 ? (
            assets.transactions.slice(0, 3).map((transaction, index) => (
              <View key={transaction.id || index} style={styles.activityItem}>
                <View style={styles.activityIcon}>
                  <Activity size={16} color="#F7931A" />
                </View>
                <View style={styles.activityContent}>
                  <Text style={styles.activityTitle}>
                    {transaction.tx_type === 'Send' ? 'Asset Transfer' : 
                     transaction.tx_type === 'Receive' ? 'Asset Received' : 'Asset Issuance'}
                  </Text>
                  <Text style={styles.activitySubtitle}>
                    {new Date(transaction.created_at).toLocaleDateString()}
                  </Text>
                </View>
                <View style={styles.activityAmount}>
                  <Text style={styles.activityAmountText}>
                    {transaction.amount} SATS
                  </Text>
                  <Text style={[styles.activityStatus, 
                    transaction.status === 'Confirmed' ? styles.statusConfirmed : 
                    transaction.status === 'Pending' ? styles.statusPending : styles.statusIssued
                  ]}>
                    {transaction.status}
                  </Text>
                </View>
              </View>
            ))
          ) : (
            <View style={{ alignItems: 'center', paddingVertical: 20 }}>
              <Text style={{ color: '#999', fontSize: 14 }}>
                No recent activity
              </Text>
            </View>
          )}
        </View>

        {/* Node Health */}
        <View style={styles.card}>
          <Text style={styles.cardTitle}>Node Health</Text>
          <View style={styles.healthMetrics}>
            <View style={styles.healthItem}>
              {lnc.isConnected ? (
                <Wifi size={20} color="#00FF88" />
              ) : (
                <WifiOff size={20} color="#FF4444" />
              )}
              <Text style={styles.healthLabel}>Network</Text>
              <Text style={styles.healthValue}>
                {lnc.isConnected ? 'Connected' : 'Disconnected'}
              </Text>
            </View>
            <View style={styles.healthItem}>
              <Zap size={20} color={lightningBalance() > 0 ? "#0099FF" : "#FFA500"} />
              <Text style={styles.healthLabel}>Liquidity</Text>
              <Text style={styles.healthValue}>
                {lightningBalance() > 0 ? 'Good' : 'Low'}
              </Text>
            </View>
            <View style={styles.healthItem}>
              <AlertCircle size={20} color={lnc.nodeInfo?.syncedToChain ? "#00FF88" : "#FFA500"} />
              <Text style={styles.healthLabel}>Sync</Text>
              <Text style={styles.healthValue}>
                {lnc.nodeInfo?.syncedToChain ? 'Synced' : 'Syncing'}
              </Text>
            </View>
          </View>
        </View>
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
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
    marginBottom: 24,
  },
  title: {
    fontSize: 28,
    fontWeight: 'bold',
    color: '#fff',
  },
  statusContainer: {
    flexDirection: 'row',
    alignItems: 'center',
  },
  statusDot: {
    width: 8,
    height: 8,
    borderRadius: 4,
    marginRight: 8,
  },
  statusText: {
    color: '#fff',
    fontSize: 12,
    fontWeight: '600',
  },
  card: {
    backgroundColor: 'rgba(255, 255, 255, 0.05)',
    borderRadius: 16,
    padding: 20,
    marginBottom: 16,
    borderWidth: 1,
    borderColor: 'rgba(255, 255, 255, 0.1)',
  },
  cardTitle: {
    fontSize: 18,
    fontWeight: 'bold',
    color: '#fff',
    marginBottom: 16,
  },
  cardHeader: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
    marginBottom: 16,
  },
  viewAllText: {
    color: '#F7931A',
    fontSize: 14,
    fontWeight: '600',
  },
  balanceAmount: {
    fontSize: 32,
    fontWeight: 'bold',
    color: '#F7931A',
    marginBottom: 4,
  },
  balanceUsd: {
    fontSize: 16,
    color: '#999',
    marginBottom: 20,
  },
  balanceBreakdown: {
    flexDirection: 'row',
    justifyContent: 'space-between',
  },
  balanceItem: {
    flex: 1,
  },
  balanceLabel: {
    fontSize: 14,
    color: '#999',
    marginBottom: 4,
  },
  balanceValue: {
    fontSize: 16,
    fontWeight: '600',
    color: '#fff',
  },
  actionsContainer: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    marginBottom: 16,
  },
  actionButton: {
    flex: 1,
    backgroundColor: 'rgba(255, 255, 255, 0.05)',
    borderRadius: 12,
    padding: 16,
    alignItems: 'center',
    marginHorizontal: 4,
    borderWidth: 1,
    borderColor: 'rgba(255, 255, 255, 0.1)',
  },
  actionText: {
    color: '#fff',
    fontSize: 14,
    fontWeight: '600',
    marginTop: 8,
  },
  channelsGrid: {
    flexDirection: 'row',
    justifyContent: 'space-between',
  },
  channelStat: {
    alignItems: 'center',
  },
  channelNumber: {
    fontSize: 24,
    fontWeight: 'bold',
    color: '#0099FF',
    marginBottom: 4,
  },
  channelLabel: {
    fontSize: 14,
    color: '#999',
  },
  activityItem: {
    flexDirection: 'row',
    alignItems: 'center',
    paddingVertical: 12,
    borderBottomWidth: 1,
    borderBottomColor: 'rgba(255, 255, 255, 0.1)',
  },
  activityIcon: {
    width: 32,
    height: 32,
    backgroundColor: 'rgba(247, 147, 26, 0.2)',
    borderRadius: 16,
    alignItems: 'center',
    justifyContent: 'center',
    marginRight: 12,
  },
  activityContent: {
    flex: 1,
  },
  activityTitle: {
    fontSize: 16,
    fontWeight: '600',
    color: '#fff',
    marginBottom: 2,
  },
  activitySubtitle: {
    fontSize: 14,
    color: '#999',
  },
  activityAmount: {
    alignItems: 'flex-end',
  },
  activityAmountText: {
    fontSize: 14,
    fontWeight: '600',
    color: '#fff',
    marginBottom: 2,
  },
  activityStatus: {
    fontSize: 12,
    fontWeight: '600',
    paddingHorizontal: 8,
    paddingVertical: 2,
    borderRadius: 8,
  },
  statusConfirmed: {
    backgroundColor: 'rgba(0, 255, 136, 0.2)',
    color: '#00FF88',
  },
  statusPending: {
    backgroundColor: 'rgba(255, 165, 0, 0.2)',
    color: '#FFA500',
  },
  statusIssued: {
    backgroundColor: 'rgba(0, 153, 255, 0.2)',
    color: '#0099FF',
  },
  healthMetrics: {
    flexDirection: 'row',
    justifyContent: 'space-between',
  },
  healthItem: {
    alignItems: 'center',
    flex: 1,
  },
  healthLabel: {
    fontSize: 14,
    color: '#999',
    marginTop: 8,
    marginBottom: 4,
  },
  healthValue: {
    fontSize: 14,
    fontWeight: '600',
    color: '#fff',
  },
});