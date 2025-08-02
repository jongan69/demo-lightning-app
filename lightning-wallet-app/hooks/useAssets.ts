import { useState, useEffect, useCallback } from 'react';
import { TaprootService, TaprootAsset } from '../services/TaprootService';

export interface AssetsState {
  assets: TaprootAsset[];
  transactions: any[];
  loading: boolean;
  error: string | null;
}

export const useAssets = () => {
  const [taprootService] = useState(() => new TaprootService());
  const [state, setState] = useState<AssetsState>({
    assets: [],
    transactions: [],
    loading: false,
    error: null,
  });

  const loadAssets = useCallback(async () => {
    setState(prev => ({ ...prev, loading: true, error: null }));

    try {
      const assets = await taprootService.listAssets();
      setState(prev => ({ ...prev, assets, loading: false }));
    } catch (error) {
      setState(prev => ({
        ...prev,
        loading: false,
        error: error instanceof Error ? error.message : 'Failed to load assets',
      }));
    }
  }, [taprootService]);

  const loadTransactions = useCallback(async () => {
    try {
      const transactions = await taprootService.getTransactions();
      setState(prev => ({ ...prev, transactions }));
    } catch (error) {
      console.error('Failed to load transactions:', error);
    }
  }, [taprootService]);

  const sendAsset = useCallback(async (assetId: string, amount: number, destination: string) => {
    try {
      const transferId = await taprootService.sendAsset({
        assetId,
        amount,
        destination,
      });
      
      if (transferId) {
        // Refresh assets and transactions after successful send
        await Promise.all([loadAssets(), loadTransactions()]);
      }
      
      return transferId;
    } catch (error) {
      console.error('Failed to send asset:', error);
      return null;
    }
  }, [taprootService, loadAssets, loadTransactions]);

  const createAssetInvoice = useCallback(async (assetId: string, amount: number, description?: string) => {
    try {
      return await taprootService.createAssetInvoice({
        assetId,
        amount,
        description,
      });
    } catch (error) {
      console.error('Failed to create asset invoice:', error);
      return null;
    }
  }, [taprootService]);

  const refreshData = useCallback(async () => {
    await Promise.all([loadAssets(), loadTransactions()]);
  }, [loadAssets, loadTransactions]);

  // Load assets on mount
  useEffect(() => {
    loadAssets();
    loadTransactions();
  }, [loadAssets, loadTransactions]);

  return {
    ...state,
    loadAssets,
    loadTransactions,
    sendAsset,
    createAssetInvoice,
    refreshData,
  };
};