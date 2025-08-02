import { useState, useEffect, useCallback } from 'react';
import { LNCService, NodeInfo, WalletBalance, ChannelBalance } from '../services/LNCService';

export interface LNCState {
  isConnected: boolean;
  isConnecting: boolean;
  nodeInfo: NodeInfo | null;
  walletBalance: WalletBalance | null;
  channelBalance: ChannelBalance | null;
  channels: any[];
  error: string | null;
}

export const useLNC = () => {
  const [lncService] = useState(() => new LNCService());
  const [state, setState] = useState<LNCState>({
    isConnected: false,
    isConnecting: false,
    nodeInfo: null,
    walletBalance: null,
    channelBalance: null,
    channels: [],
    error: null,
  });

  const connect = useCallback(async (pairingPhrase: string, password: string) => {
    setState(prev => ({ ...prev, isConnecting: true, error: null }));

    try {
      const initialized = await lncService.initialize({ pairingPhrase, password });
      if (!initialized) {
        throw new Error('Failed to initialize LNC service');
      }

      const connected = await lncService.connect();
      if (!connected) {
        throw new Error('Failed to connect to Lightning node');
      }

      setState(prev => ({ ...prev, isConnected: true, isConnecting: false }));
      
      // Fetch initial data
      await refreshData();
    } catch (error) {
      setState(prev => ({
        ...prev,
        isConnecting: false,
        error: error instanceof Error ? error.message : 'Unknown error occurred',
      }));
    }
  }, [lncService]);

  const connectWithStoredCredentials = useCallback(async (password: string) => {
    setState(prev => ({ ...prev, isConnecting: true, error: null }));

    try {
      const initialized = await lncService.initialize({ password });
      if (!initialized) {
        throw new Error('Failed to initialize LNC service with stored credentials');
      }

      const connected = await lncService.connect();
      if (!connected) {
        throw new Error('Failed to connect to Lightning node');
      }

      setState(prev => ({ ...prev, isConnected: true, isConnecting: false }));
      
      // Fetch initial data
      await refreshData();
    } catch (error) {
      setState(prev => ({
        ...prev,
        isConnecting: false,
        error: error instanceof Error ? error.message : 'Unknown error occurred',
      }));
    }
  }, [lncService]);

  const disconnect = useCallback(async () => {
    try {
      await lncService.disconnect();
      setState(prev => ({
        ...prev,
        isConnected: false,
        nodeInfo: null,
        walletBalance: null,
        channelBalance: null,
        channels: [],
      }));
    } catch (error) {
      console.error('Error disconnecting:', error);
    }
  }, [lncService]);

  const refreshData = useCallback(async () => {
    if (!state.isConnected) return;

    try {
      const [nodeInfo, walletBalance, channelBalance, channels] = await Promise.all([
        lncService.getNodeInfo(),
        lncService.getWalletBalance(),
        lncService.getChannelBalance(),
        lncService.listChannels(),
      ]);

      setState(prev => ({
        ...prev,
        nodeInfo,
        walletBalance,
        channelBalance,
        channels,
      }));
    } catch (error) {
      console.error('Error refreshing data:', error);
      setState(prev => ({
        ...prev,
        error: error instanceof Error ? error.message : 'Failed to refresh data',
      }));
    }
  }, [lncService, state.isConnected]);

  const createInvoice = useCallback(async (amount: number, memo?: string) => {
    return await lncService.createInvoice(amount, memo);
  }, [lncService]);

  const payInvoice = useCallback(async (paymentRequest: string) => {
    return await lncService.payInvoice(paymentRequest);
  }, [lncService]);

  const clearCredentials = useCallback(async () => {
    await lncService.clearStoredCredentials();
  }, [lncService]);

  // Auto-refresh data every 30 seconds when connected
  useEffect(() => {
    if (!state.isConnected) return;

    const interval = setInterval(refreshData, 30000);
    return () => clearInterval(interval);
  }, [state.isConnected, refreshData]);

  return {
    ...state,
    connect,
    connectWithStoredCredentials,
    disconnect,
    refreshData,
    createInvoice,
    payInvoice,
    clearCredentials,
  };
};