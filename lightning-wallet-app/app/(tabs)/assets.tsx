import React, { useState } from 'react';
import { ScrollView, View, Text, StyleSheet, TouchableOpacity, TextInput } from 'react-native';
import { StatusBar } from 'expo-status-bar';
import { SafeAreaView } from 'react-native-safe-area-context';
import { Plus, TrendingUp, Send, Download, Coins, Sparkles } from 'lucide-react-native';

const mockAssets = [
  {
    id: 'asset_001',
    name: 'Lightning Gold',
    symbol: 'LGOLD',
    balance: 1500.0,
    value: 0.00000123,
    change24h: 5.2,
    description: 'Digital gold backed by Bitcoin reserves'
  },
  {
    id: 'asset_002',
    name: 'Taproot Shares',
    symbol: 'TSHARE',
    balance: 250.0,
    value: 0.00000089,
    change24h: -2.1,
    description: 'Equity tokens for decentralized projects'
  },
  {
    id: 'asset_003',
    name: 'Lightning Points',
    symbol: 'LPTS',
    balance: 5000.0,
    value: 0.00000012,
    change24h: 12.8,
    description: 'Reward points for Lightning Network activity'
  }
];

export default function AssetsScreen() {
  const [activeTab, setActiveTab] = useState('portfolio');
  const [assetName, setAssetName] = useState('');
  const [assetSymbol, setAssetSymbol] = useState('');
  const [totalSupply, setTotalSupply] = useState('');

  const totalPortfolioValue = mockAssets.reduce((sum, asset) => sum + (asset.balance * asset.value), 0);

  return (
    <SafeAreaView style={styles.container}>
      <StatusBar style="light" />
      <ScrollView style={styles.scrollView} showsVerticalScrollIndicator={false}>
        <View style={styles.header}>
          <Text style={styles.title}>Taproot Assets</Text>
          <TouchableOpacity style={styles.addButton}>
            <Plus size={20} color="#F7931A" />
          </TouchableOpacity>
        </View>

        {/* Portfolio Value */}
        <View style={styles.portfolioCard}>
          <Text style={styles.portfolioLabel}>Portfolio Value</Text>
          <Text style={styles.portfolioAmount}>{totalPortfolioValue.toFixed(8)} BTC</Text>
          <Text style={styles.portfolioUsd}>≈ $1,234.56 USD</Text>
          <View style={styles.portfolioChange}>
            <TrendingUp size={16} color="#00FF88" />
            <Text style={styles.portfolioChangeText}>+8.4% (24h)</Text>
          </View>
        </View>

        {/* Tab Navigation */}
        <View style={styles.tabContainer}>
          <TouchableOpacity 
            style={[styles.tab, activeTab === 'portfolio' && styles.activeTab]}
            onPress={() => setActiveTab('portfolio')}
          >
            <Text style={[styles.tabText, activeTab === 'portfolio' && styles.activeTabText]}>Portfolio</Text>
          </TouchableOpacity>
          <TouchableOpacity 
            style={[styles.tab, activeTab === 'issue' && styles.activeTab]}
            onPress={() => setActiveTab('issue')}
          >
            <Text style={[styles.tabText, activeTab === 'issue' && styles.activeTabText]}>Issue Asset</Text>
          </TouchableOpacity>
          <TouchableOpacity 
            style={[styles.tab, activeTab === 'marketplace' && styles.activeTab]}
            onPress={() => setActiveTab('marketplace')}
          >
            <Text style={[styles.tabText, activeTab === 'marketplace' && styles.activeTabText]}>Marketplace</Text>
          </TouchableOpacity>
        </View>

        {/* Portfolio Tab */}
        {activeTab === 'portfolio' && (
          <View>
            {mockAssets.map((asset, index) => (
              <View key={asset.id} style={styles.assetCard}>
                <View style={styles.assetHeader}>
                  <View style={styles.assetIcon}>
                    <Coins size={24} color="#F7931A" />
                  </View>
                  <View style={styles.assetInfo}>
                    <Text style={styles.assetName}>{asset.name}</Text>
                    <Text style={styles.assetSymbol}>{asset.symbol}</Text>
                  </View>
                  <View style={styles.assetValue}>
                    <Text style={styles.assetBalance}>{asset.balance.toLocaleString()}</Text>
                    <Text style={[styles.assetChange, asset.change24h > 0 ? styles.positive : styles.negative]}>
                      {asset.change24h > 0 ? '+' : ''}{asset.change24h}%
                    </Text>
                  </View>
                </View>
                
                <Text style={styles.assetDescription}>{asset.description}</Text>
                
                <View style={styles.assetActions}>
                  <TouchableOpacity style={styles.assetActionButton}>
                    <Send size={16} color="#F7931A" />
                    <Text style={styles.assetActionText}>Send</Text>
                  </TouchableOpacity>
                  <TouchableOpacity style={styles.assetActionButton}>
                    <Download size={16} color="#F7931A" />
                    <Text style={styles.assetActionText}>Receive</Text>
                  </TouchableOpacity>
                </View>
              </View>
            ))}
          </View>
        )}

        {/* Issue Asset Tab */}
        {activeTab === 'issue' && (
          <View style={styles.card}>
            <Text style={styles.cardTitle}>Issue New Asset</Text>
            <Text style={styles.cardSubtitle}>Create a new Taproot asset on the Lightning Network</Text>
            
            <View style={styles.inputGroup}>
              <Text style={styles.inputLabel}>Asset Name</Text>
              <TextInput
                style={styles.input}
                value={assetName}
                onChangeText={setAssetName}
                placeholder="e.g., My Token"
                placeholderTextColor="#666"
              />
            </View>

            <View style={styles.inputGroup}>
              <Text style={styles.inputLabel}>Symbol</Text>
              <TextInput
                style={styles.input}
                value={assetSymbol}
                onChangeText={setAssetSymbol}
                placeholder="e.g., MTK"
                placeholderTextColor="#666"
              />
            </View>

            <View style={styles.inputGroup}>
              <Text style={styles.inputLabel}>Total Supply</Text>
              <TextInput
                style={styles.input}
                value={totalSupply}
                onChangeText={setTotalSupply}
                placeholder="1000000"
                placeholderTextColor="#666"
                keyboardType="numeric"
              />
            </View>

            <View style={styles.feeInfo}>
              <Text style={styles.feeLabel}>Issuance Fee</Text>
              <Text style={styles.feeAmount}>0.00001 BTC</Text>
            </View>

            <TouchableOpacity style={styles.issueButton}>
              <Sparkles size={20} color="#fff" />
              <Text style={styles.issueButtonText}>Issue Asset</Text>
            </TouchableOpacity>
          </View>
        )}

        {/* Marketplace Tab */}
        {activeTab === 'marketplace' && (
          <View>
            <View style={styles.marketplaceHeader}>
              <Text style={styles.cardTitle}>Asset Marketplace</Text>
              <Text style={styles.cardSubtitle}>Discover and trade Taproot assets</Text>
            </View>
            
            {mockAssets.map((asset, index) => (
              <View key={`market_${asset.id}`} style={styles.marketAssetCard}>
                <View style={styles.marketAssetHeader}>
                  <View style={styles.assetIcon}>
                    <Coins size={20} color="#F7931A" />
                  </View>
                  <View style={styles.assetInfo}>
                    <Text style={styles.assetName}>{asset.name}</Text>
                    <Text style={styles.assetSymbol}>{asset.symbol}</Text>
                  </View>
                  <View style={styles.assetValue}>
                    <Text style={styles.assetPrice}>{asset.value.toFixed(8)} BTC</Text>
                    <Text style={[styles.assetChange, asset.change24h > 0 ? styles.positive : styles.negative]}>
                      {asset.change24h > 0 ? '+' : ''}{asset.change24h}%
                    </Text>
                  </View>
                </View>
                
                <Text style={styles.assetDescription}>{asset.description}</Text>
                
                <View style={styles.marketActions}>
                  <TouchableOpacity style={styles.buyButton}>
                    <Text style={styles.buyButtonText}>Buy</Text>
                  </TouchableOpacity>
                  <TouchableOpacity style={styles.sellButton}>
                    <Text style={styles.sellButtonText}>Sell</Text>
                  </TouchableOpacity>
                </View>
              </View>
            ))}
          </View>
        )}
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
  addButton: {
    backgroundColor: 'rgba(247, 147, 26, 0.2)',
    borderRadius: 8,
    padding: 8,
  },
  portfolioCard: {
    backgroundColor: 'rgba(255, 255, 255, 0.05)',
    borderRadius: 16,
    padding: 20,
    marginBottom: 20,
    borderWidth: 1,
    borderColor: 'rgba(255, 255, 255, 0.1)',
    alignItems: 'center',
  },
  portfolioLabel: {
    fontSize: 14,
    color: '#999',
    marginBottom: 8,
  },
  portfolioAmount: {
    fontSize: 28,
    fontWeight: 'bold',
    color: '#F7931A',
    marginBottom: 4,
  },
  portfolioUsd: {
    fontSize: 16,
    color: '#999',
    marginBottom: 12,
  },
  portfolioChange: {
    flexDirection: 'row',
    alignItems: 'center',
  },
  portfolioChangeText: {
    fontSize: 14,
    fontWeight: '600',
    color: '#00FF88',
    marginLeft: 4,
  },
  tabContainer: {
    flexDirection: 'row',
    backgroundColor: 'rgba(255, 255, 255, 0.05)',
    borderRadius: 12,
    padding: 4,
    marginBottom: 20,
  },
  tab: {
    flex: 1,
    alignItems: 'center',
    paddingVertical: 12,
    borderRadius: 8,
  },
  activeTab: {
    backgroundColor: 'rgba(247, 147, 26, 0.2)',
  },
  tabText: {
    fontSize: 14,
    fontWeight: '600',
    color: '#666',
  },
  activeTabText: {
    color: '#F7931A',
  },
  assetCard: {
    backgroundColor: 'rgba(255, 255, 255, 0.05)',
    borderRadius: 16,
    padding: 20,
    marginBottom: 16,
    borderWidth: 1,
    borderColor: 'rgba(255, 255, 255, 0.1)',
  },
  assetHeader: {
    flexDirection: 'row',
    alignItems: 'center',
    marginBottom: 12,
  },
  assetIcon: {
    width: 40,
    height: 40,
    backgroundColor: 'rgba(247, 147, 26, 0.2)',
    borderRadius: 20,
    alignItems: 'center',
    justifyContent: 'center',
    marginRight: 12,
  },
  assetInfo: {
    flex: 1,
  },
  assetName: {
    fontSize: 16,
    fontWeight: 'bold',
    color: '#fff',
    marginBottom: 2,
  },
  assetSymbol: {
    fontSize: 14,
    color: '#999',
  },
  assetValue: {
    alignItems: 'flex-end',
  },
  assetBalance: {
    fontSize: 16,
    fontWeight: 'bold',
    color: '#fff',
    marginBottom: 2,
  },
  assetChange: {
    fontSize: 12,
    fontWeight: '600',
  },
  positive: {
    color: '#00FF88',
  },
  negative: {
    color: '#FF4444',
  },
  assetDescription: {
    fontSize: 14,
    color: '#999',
    marginBottom: 16,
    lineHeight: 20,
  },
  assetActions: {
    flexDirection: 'row',
    justifyContent: 'space-between',
  },
  assetActionButton: {
    flex: 1,
    flexDirection: 'row',
    alignItems: 'center',
    justifyContent: 'center',
    backgroundColor: 'rgba(255, 255, 255, 0.1)',
    borderRadius: 8,
    paddingVertical: 12,
    marginHorizontal: 4,
  },
  assetActionText: {
    fontSize: 14,
    fontWeight: '600',
    color: '#F7931A',
    marginLeft: 4,
  },
  card: {
    backgroundColor: 'rgba(255, 255, 255, 0.05)',
    borderRadius: 16,
    padding: 20,
    borderWidth: 1,
    borderColor: 'rgba(255, 255, 255, 0.1)',
  },
  cardTitle: {
    fontSize: 20,
    fontWeight: 'bold',
    color: '#fff',
    marginBottom: 8,
  },
  cardSubtitle: {
    fontSize: 14,
    color: '#999',
    marginBottom: 20,
  },
  inputGroup: {
    marginBottom: 16,
  },
  inputLabel: {
    fontSize: 14,
    fontWeight: '600',
    color: '#999',
    marginBottom: 8,
  },
  input: {
    backgroundColor: 'rgba(255, 255, 255, 0.1)',
    borderRadius: 12,
    padding: 16,
    fontSize: 16,
    color: '#fff',
    borderWidth: 1,
    borderColor: 'rgba(255, 255, 255, 0.2)',
  },
  feeInfo: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
    marginBottom: 24,
    padding: 16,
    backgroundColor: 'rgba(255, 255, 255, 0.05)',
    borderRadius: 12,
  },
  feeLabel: {
    fontSize: 14,
    color: '#999',
  },
  feeAmount: {
    fontSize: 14,
    fontWeight: '600',
    color: '#F7931A',
  },
  issueButton: {
    backgroundColor: '#F7931A',
    borderRadius: 12,
    paddingVertical: 16,
    flexDirection: 'row',
    alignItems: 'center',
    justifyContent: 'center',
  },
  issueButtonText: {
    fontSize: 16,
    fontWeight: 'bold',
    color: '#fff',
    marginLeft: 8,
  },
  marketplaceHeader: {
    marginBottom: 20,
  },
  marketAssetCard: {
    backgroundColor: 'rgba(255, 255, 255, 0.05)',
    borderRadius: 16,
    padding: 20,
    marginBottom: 16,
    borderWidth: 1,
    borderColor: 'rgba(255, 255, 255, 0.1)',
  },
  marketAssetHeader: {
    flexDirection: 'row',
    alignItems: 'center',
    marginBottom: 12,
  },
  assetPrice: {
    fontSize: 14,
    fontWeight: 'bold',
    color: '#fff',
    marginBottom: 2,
  },
  marketActions: {
    flexDirection: 'row',
    justifyContent: 'space-between',
  },
  buyButton: {
    flex: 1,
    backgroundColor: '#00FF88',
    borderRadius: 8,
    paddingVertical: 12,
    alignItems: 'center',
    marginRight: 8,
  },
  buyButtonText: {
    fontSize: 14,
    fontWeight: 'bold',
    color: '#000',
  },
  sellButton: {
    flex: 1,
    backgroundColor: 'rgba(255, 68, 68, 0.2)',
    borderRadius: 8,
    paddingVertical: 12,
    alignItems: 'center',
    marginLeft: 8,
    borderWidth: 1,
    borderColor: '#FF4444',
  },
  sellButtonText: {
    fontSize: 14,
    fontWeight: 'bold',
    color: '#FF4444',
  },
});