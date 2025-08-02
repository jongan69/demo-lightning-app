import React, { useState } from 'react';
import { 
  ScrollView, 
  View, 
  Text, 
  StyleSheet, 
  TouchableOpacity,
  TextInput,
  Alert,
  ActivityIndicator 
} from 'react-native';
import { StatusBar } from 'expo-status-bar';
import { SafeAreaView } from 'react-native-safe-area-context';
import { Settings, Wifi, Key, Trash2, RefreshCw } from 'lucide-react-native';
import { useLNC } from '../../hooks/useLNC';

export default function SettingsScreen() {
  const lnc = useLNC();
  const [pairingPhrase, setPairingPhrase] = useState('');
  const [password, setPassword] = useState('');
  const [showConnectForm, setShowConnectForm] = useState(!lnc.isConnected);

  const handleConnect = async () => {
    if (!pairingPhrase.trim() || !password.trim()) {
      Alert.alert('Error', 'Please enter both pairing phrase and password');
      return;
    }

    try {
      await lnc.connect(pairingPhrase.trim(), password);
      if (lnc.isConnected) {
        Alert.alert('Success', 'Connected to Lightning node!');
        setShowConnectForm(false);
        setPairingPhrase('');
        setPassword('');
      }
    } catch (error) {
      Alert.alert('Connection Failed', lnc.error || 'Unknown error occurred');
    }
  };

  const handleConnectWithStored = async () => {
    if (!password.trim()) {
      Alert.alert('Error', 'Please enter your password');
      return;
    }

    try {
      await lnc.connectWithStoredCredentials(password);
      if (lnc.isConnected) {
        Alert.alert('Success', 'Connected using stored credentials!');
        setShowConnectForm(false);
        setPassword('');
      }
    } catch (error) {
      Alert.alert('Connection Failed', lnc.error || 'Unknown error occurred');
    }
  };

  const handleDisconnect = async () => {
    Alert.alert(
      'Disconnect',
      'Are you sure you want to disconnect from your Lightning node?',
      [
        { text: 'Cancel', style: 'cancel' },
        { 
          text: 'Disconnect', 
          style: 'destructive',
          onPress: async () => {
            await lnc.disconnect();
            setShowConnectForm(true);
          }
        }
      ]
    );
  };

  const handleClearCredentials = async () => {
    Alert.alert(
      'Clear Stored Credentials',
      'This will remove all stored connection data. You will need to re-enter your pairing phrase.',
      [
        { text: 'Cancel', style: 'cancel' },
        { 
          text: 'Clear', 
          style: 'destructive',
          onPress: async () => {
            await lnc.clearCredentials();
            Alert.alert('Success', 'Stored credentials cleared');
          }
        }
      ]
    );
  };

  return (
    <SafeAreaView style={styles.container}>
      <StatusBar style="light" />
      <ScrollView style={styles.scrollView} showsVerticalScrollIndicator={false}>
        <View style={styles.header}>
          <Settings size={28} color="#F7931A" />
          <Text style={styles.title}>Settings</Text>
        </View>

        {/* Connection Status */}
        <View style={styles.card}>
          <Text style={styles.cardTitle}>Lightning Node Connection</Text>
          
          <View style={styles.statusRow}>
            <View style={styles.statusInfo}>
              <Text style={styles.statusLabel}>Status</Text>
              <Text style={[styles.statusValue, { 
                color: lnc.isConnected ? '#00FF88' : '#FF4444' 
              }]}>
                {lnc.isConnected ? 'Connected' : 'Disconnected'}
              </Text>
            </View>
            <Wifi 
              size={24} 
              color={lnc.isConnected ? '#00FF88' : '#FF4444'} 
            />
          </View>

          {lnc.nodeInfo && (
            <View style={styles.nodeInfo}>
              <Text style={styles.nodeInfoLabel}>Node Alias</Text>
              <Text style={styles.nodeInfoValue}>{lnc.nodeInfo.alias}</Text>
              
              <Text style={styles.nodeInfoLabel}>Public Key</Text>
              <Text style={styles.nodeInfoValue} numberOfLines={1} ellipsizeMode="middle">
                {lnc.nodeInfo.identityPubkey}
              </Text>
              
              <Text style={styles.nodeInfoLabel}>Block Height</Text>
              <Text style={styles.nodeInfoValue}>{lnc.nodeInfo.blockHeight}</Text>
            </View>
          )}

          {lnc.isConnected ? (
            <View style={styles.buttonContainer}>
              <TouchableOpacity 
                style={[styles.button, styles.refreshButton]} 
                onPress={lnc.refreshData}
                disabled={lnc.isConnecting}
              >
                {lnc.isConnecting ? (
                  <ActivityIndicator size="small" color="#0099FF" />
                ) : (
                  <RefreshCw size={20} color="#0099FF" />
                )}
                <Text style={[styles.buttonText, { color: '#0099FF' }]}>
                  Refresh Data
                </Text>
              </TouchableOpacity>

              <TouchableOpacity 
                style={[styles.button, styles.disconnectButton]} 
                onPress={handleDisconnect}
              >
                <Text style={[styles.buttonText, { color: '#FF4444' }]}>
                  Disconnect
                </Text>
              </TouchableOpacity>
            </View>
          ) : (
            <TouchableOpacity 
              style={[styles.button, styles.connectButton]} 
              onPress={() => setShowConnectForm(true)}
            >
              <Text style={styles.buttonText}>Connect to Node</Text>
            </TouchableOpacity>
          )}
        </View>

        {/* Connection Form */}
        {showConnectForm && (
          <View style={styles.card}>
            <Text style={styles.cardTitle}>Connect to Lightning Node</Text>
            <Text style={styles.cardSubtitle}>
              Enter your Lightning Node Connect pairing phrase and password
            </Text>

            <View style={styles.inputContainer}>
              <Text style={styles.inputLabel}>Pairing Phrase</Text>
              <TextInput
                style={styles.textInput}
                value={pairingPhrase}
                onChangeText={setPairingPhrase}
                placeholder="Enter pairing phrase..."
                placeholderTextColor="#666"
                multiline
                numberOfLines={2}
                textAlignVertical="top"
              />
            </View>

            <View style={styles.inputContainer}>
              <Text style={styles.inputLabel}>Password</Text>
              <TextInput
                style={styles.textInput}
                value={password}
                onChangeText={setPassword}
                placeholder="Enter password..."
                placeholderTextColor="#666"
                secureTextEntry
              />
            </View>

            <View style={styles.buttonContainer}>
              <TouchableOpacity 
                style={[styles.button, styles.connectButton]} 
                onPress={handleConnect}
                disabled={lnc.isConnecting}
              >
                {lnc.isConnecting ? (
                  <ActivityIndicator size="small" color="#fff" />
                ) : (
                  <Key size={20} color="#fff" />
                )}
                <Text style={styles.buttonText}>
                  {lnc.isConnecting ? 'Connecting...' : 'Connect'}
                </Text>
              </TouchableOpacity>

              <TouchableOpacity 
                style={[styles.button, styles.storedButton]} 
                onPress={handleConnectWithStored}
                disabled={lnc.isConnecting}
              >
                <Text style={[styles.buttonText, { color: '#0099FF' }]}>
                  Use Stored Credentials
                </Text>
              </TouchableOpacity>
            </View>

            {lnc.error && (
              <View style={styles.errorContainer}>
                <Text style={styles.errorText}>{lnc.error}</Text>
              </View>
            )}
          </View>
        )}

        {/* Advanced Settings */}
        <View style={styles.card}>
          <Text style={styles.cardTitle}>Advanced</Text>
          
          <TouchableOpacity 
            style={styles.advancedButton} 
            onPress={handleClearCredentials}
          >
            <Trash2 size={20} color="#FF4444" />
            <Text style={[styles.advancedButtonText, { color: '#FF4444' }]}>
              Clear Stored Credentials
            </Text>
          </TouchableOpacity>
        </View>

        {/* App Info */}
        <View style={styles.card}>
          <Text style={styles.cardTitle}>About</Text>
          <Text style={styles.aboutText}>
            Lightning Taproot Assets Wallet
          </Text>
          <Text style={styles.aboutText}>
            Version 1.0.0
          </Text>
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
    alignItems: 'center',
    marginBottom: 24,
  },
  title: {
    fontSize: 28,
    fontWeight: 'bold',
    color: '#fff',
    marginLeft: 12,
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
    marginBottom: 12,
  },
  cardSubtitle: {
    fontSize: 14,
    color: '#999',
    marginBottom: 16,
  },
  statusRow: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
    marginBottom: 16,
  },
  statusInfo: {
    flex: 1,
  },
  statusLabel: {
    fontSize: 14,
    color: '#999',
    marginBottom: 4,
  },
  statusValue: {
    fontSize: 16,
    fontWeight: '600',
  },
  nodeInfo: {
    marginBottom: 16,
  },
  nodeInfoLabel: {
    fontSize: 14,
    color: '#999',
    marginBottom: 4,
    marginTop: 8,
  },
  nodeInfoValue: {
    fontSize: 14,
    color: '#fff',
    fontFamily: 'monospace',
  },
  inputContainer: {
    marginBottom: 16,
  },
  inputLabel: {
    fontSize: 14,
    color: '#fff',
    marginBottom: 8,
    fontWeight: '600',
  },
  textInput: {
    backgroundColor: 'rgba(255, 255, 255, 0.1)',
    borderRadius: 8,
    padding: 12,
    color: '#fff',
    fontSize: 16,
    borderWidth: 1,
    borderColor: 'rgba(255, 255, 255, 0.2)',
  },
  buttonContainer: {
    flexDirection: 'row',
    gap: 12,
  },
  button: {
    flex: 1,
    flexDirection: 'row',
    alignItems: 'center',
    justifyContent: 'center',
    padding: 12,
    borderRadius: 8,
    gap: 8,
  },
  connectButton: {
    backgroundColor: '#F7931A',
  },
  refreshButton: {
    backgroundColor: 'rgba(0, 153, 255, 0.1)',
    borderWidth: 1,
    borderColor: '#0099FF',
  },
  disconnectButton: {
    backgroundColor: 'rgba(255, 68, 68, 0.1)',
    borderWidth: 1,
    borderColor: '#FF4444',
  },
  storedButton: {
    backgroundColor: 'rgba(0, 153, 255, 0.1)',
    borderWidth: 1,
    borderColor: '#0099FF',
  },
  buttonText: {
    color: '#fff',
    fontSize: 16,
    fontWeight: '600',
  },
  advancedButton: {
    flexDirection: 'row',
    alignItems: 'center',
    padding: 12,
    gap: 12,
  },
  advancedButtonText: {
    fontSize: 16,
    fontWeight: '600',
  },
  errorContainer: {
    backgroundColor: 'rgba(255, 68, 68, 0.1)',
    borderRadius: 8,
    padding: 12,
    marginTop: 12,
  },
  errorText: {
    color: '#FF4444',
    fontSize: 14,
  },
  aboutText: {
    color: '#999',
    fontSize: 14,
    marginBottom: 4,
  },
});