import React, { useState } from 'react';
import { ScrollView, View, Text, StyleSheet, TouchableOpacity, TextInput } from 'react-native';
import { StatusBar } from 'expo-status-bar';
import { SafeAreaView } from 'react-native-safe-area-context';
import { Send, Download, QrCode, Copy, Zap, Bitcoin } from 'lucide-react-native';

const mockAddresses = {
  bitcoin: 'bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh',
  lightning: 'lnbc1ps9zpqhp5qy5rstkwd6x9c6rsxj4w8t...'
};

export default function WalletScreen() {
  const [activeTab, setActiveTab] = useState('send');
  const [amount, setAmount] = useState('');
  const [address, setAddress] = useState('');

  return (
    <SafeAreaView style={styles.container}>
      <StatusBar style="light" />
      <ScrollView style={styles.scrollView} showsVerticalScrollIndicator={false}>
        <View style={styles.header}>
          <Text style={styles.title}>Wallet</Text>
        </View>

        {/* Balance Overview */}
        <View style={styles.balanceCard}>
          <Text style={styles.balanceLabel}>Total Balance</Text>
          <Text style={styles.balanceAmount}>0.00666347 BTC</Text>
          <Text style={styles.balanceUsd}>≈ $4,521.32 USD</Text>
          
          <View style={styles.balanceBreakdown}>
            <View style={styles.balanceItem}>
              <Bitcoin size={20} color="#F7931A" />
              <Text style={styles.balanceItemLabel}>On-chain</Text>
              <Text style={styles.balanceItemValue}>0.00542891</Text>
            </View>
            <View style={styles.balanceItem}>
              <Zap size={20} color="#0099FF" />
              <Text style={styles.balanceItemLabel}>Lightning</Text>
              <Text style={styles.balanceItemValue}>0.00123456</Text>
            </View>
          </View>
        </View>

        {/* Action Tabs */}
        <View style={styles.tabContainer}>
          <TouchableOpacity 
            style={[styles.tab, activeTab === 'send' && styles.activeTab]}
            onPress={() => setActiveTab('send')}
          >
            <Send size={20} color={activeTab === 'send' ? '#F7931A' : '#666'} />
            <Text style={[styles.tabText, activeTab === 'send' && styles.activeTabText]}>Send</Text>
          </TouchableOpacity>
          <TouchableOpacity 
            style={[styles.tab, activeTab === 'receive' && styles.activeTab]}
            onPress={() => setActiveTab('receive')}
          >
            <Download size={20} color={activeTab === 'receive' ? '#F7931A' : '#666'} />
            <Text style={[styles.tabText, activeTab === 'receive' && styles.activeTabText]}>Receive</Text>
          </TouchableOpacity>
        </View>

        {/* Send Tab */}
        {activeTab === 'send' && (
          <View style={styles.card}>
            <Text style={styles.cardTitle}>Send Payment</Text>
            
            <View style={styles.inputGroup}>
              <Text style={styles.inputLabel}>Amount (BTC)</Text>
              <TextInput
                style={styles.input}
                value={amount}
                onChangeText={setAmount}
                placeholder="0.00000000"
                placeholderTextColor="#666"
                keyboardType="numeric"
              />
            </View>

            <View style={styles.inputGroup}>
              <Text style={styles.inputLabel}>Recipient Address</Text>
              <View style={styles.addressInputContainer}>
                <TextInput
                  style={[styles.input, styles.addressInput]}
                  value={address}
                  onChangeText={setAddress}
                  placeholder="Bitcoin address or Lightning invoice"
                  placeholderTextColor="#666"
                  multiline
                />
                <TouchableOpacity style={styles.qrButton}>
                  <QrCode size={20} color="#F7931A" />
                </TouchableOpacity>
              </View>
            </View>

            <View style={styles.paymentTypeSelector}>
              <TouchableOpacity style={styles.paymentType}>
                <Bitcoin size={20} color="#F7931A" />
                <Text style={styles.paymentTypeText}>On-chain</Text>
              </TouchableOpacity>
              <TouchableOpacity style={[styles.paymentType, styles.activePaymentType]}>
                <Zap size={20} color="#0099FF" />
                <Text style={[styles.paymentTypeText, styles.activePaymentTypeText]}>Lightning</Text>
              </TouchableOpacity>
            </View>

            <TouchableOpacity style={styles.sendButton}>
              <Text style={styles.sendButtonText}>Send Payment</Text>
            </TouchableOpacity>
          </View>
        )}

        {/* Receive Tab */}
        {activeTab === 'receive' && (
          <View style={styles.card}>
            <Text style={styles.cardTitle}>Receive Payment</Text>
            
            <View style={styles.addressCard}>
              <View style={styles.addressHeader}>
                <Bitcoin size={20} color="#F7931A" />
                <Text style={styles.addressType}>Bitcoin Address</Text>
              </View>
              <Text style={styles.addressText}>{mockAddresses.bitcoin}</Text>
              <TouchableOpacity style={styles.copyButton}>
                <Copy size={16} color="#F7931A" />
                <Text style={styles.copyButtonText}>Copy</Text>
              </TouchableOpacity>
            </View>

            <View style={styles.addressCard}>
              <View style={styles.addressHeader}>
                <Zap size={20} color="#0099FF" />
                <Text style={styles.addressType}>Lightning Invoice</Text>
              </View>
              <Text style={styles.addressText}>{mockAddresses.lightning}</Text>
              <TouchableOpacity style={styles.copyButton}>
                <Copy size={16} color="#F7931A" />
                <Text style={styles.copyButtonText}>Copy</Text>
              </TouchableOpacity>
            </View>

            <TouchableOpacity style={styles.generateButton}>
              <QrCode size={20} color="#fff" />
              <Text style={styles.generateButtonText}>Generate QR Code</Text>
            </TouchableOpacity>
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
    marginBottom: 24,
  },
  title: {
    fontSize: 28,
    fontWeight: 'bold',
    color: '#fff',
  },
  balanceCard: {
    backgroundColor: 'rgba(255, 255, 255, 0.05)',
    borderRadius: 16,
    padding: 20,
    marginBottom: 20,
    borderWidth: 1,
    borderColor: 'rgba(255, 255, 255, 0.1)',
    alignItems: 'center',
  },
  balanceLabel: {
    fontSize: 14,
    color: '#999',
    marginBottom: 8,
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
    width: '100%',
    justifyContent: 'space-between',
  },
  balanceItem: {
    alignItems: 'center',
    flex: 1,
  },
  balanceItemLabel: {
    fontSize: 14,
    color: '#999',
    marginTop: 8,
    marginBottom: 4,
  },
  balanceItemValue: {
    fontSize: 16,
    fontWeight: '600',
    color: '#fff',
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
    flexDirection: 'row',
    alignItems: 'center',
    justifyContent: 'center',
    paddingVertical: 12,
    borderRadius: 8,
  },
  activeTab: {
    backgroundColor: 'rgba(247, 147, 26, 0.2)',
  },
  tabText: {
    fontSize: 16,
    fontWeight: '600',
    color: '#666',
    marginLeft: 8,
  },
  activeTabText: {
    color: '#F7931A',
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
    marginBottom: 20,
  },
  inputGroup: {
    marginBottom: 20,
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
  addressInputContainer: {
    flexDirection: 'row',
    alignItems: 'flex-end',
  },
  addressInput: {
    flex: 1,
    marginRight: 12,
    height: 80,
    textAlignVertical: 'top',
  },
  qrButton: {
    backgroundColor: 'rgba(247, 147, 26, 0.2)',
    borderRadius: 12,
    padding: 16,
    alignItems: 'center',
    justifyContent: 'center',
  },
  paymentTypeSelector: {
    flexDirection: 'row',
    marginBottom: 24,
  },
  paymentType: {
    flex: 1,
    flexDirection: 'row',
    alignItems: 'center',
    justifyContent: 'center',
    paddingVertical: 12,
    marginHorizontal: 4,
    borderRadius: 12,
    backgroundColor: 'rgba(255, 255, 255, 0.05)',
  },
  activePaymentType: {
    backgroundColor: 'rgba(0, 153, 255, 0.2)',
  },
  paymentTypeText: {
    fontSize: 14,
    fontWeight: '600',
    color: '#666',
    marginLeft: 8,
  },
  activePaymentTypeText: {
    color: '#0099FF',
  },
  sendButton: {
    backgroundColor: '#F7931A',
    borderRadius: 12,
    paddingVertical: 16,
    alignItems: 'center',
  },
  sendButtonText: {
    fontSize: 16,
    fontWeight: 'bold',
    color: '#fff',
  },
  addressCard: {
    backgroundColor: 'rgba(255, 255, 255, 0.05)',
    borderRadius: 12,
    padding: 16,
    marginBottom: 16,
    borderWidth: 1,
    borderColor: 'rgba(255, 255, 255, 0.1)',
  },
  addressHeader: {
    flexDirection: 'row',
    alignItems: 'center',
    marginBottom: 12,
  },
  addressType: {
    fontSize: 16,
    fontWeight: '600',
    color: '#fff',
    marginLeft: 8,
  },
  addressText: {
    fontSize: 14,
    color: '#999',
    marginBottom: 12,
    lineHeight: 20,
  },
  copyButton: {
    flexDirection: 'row',
    alignItems: 'center',
    alignSelf: 'flex-start',
  },
  copyButtonText: {
    fontSize: 14,
    fontWeight: '600',
    color: '#F7931A',
    marginLeft: 4,
  },
  generateButton: {
    backgroundColor: '#0099FF',
    borderRadius: 12,
    paddingVertical: 16,
    flexDirection: 'row',
    alignItems: 'center',
    justifyContent: 'center',
  },
  generateButtonText: {
    fontSize: 16,
    fontWeight: 'bold',
    color: '#fff',
    marginLeft: 8,
  },
});