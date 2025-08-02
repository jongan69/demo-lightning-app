export interface TaprootAsset {
  assetId: string;
  name: string;
  balance: number;
  decimals: number;
  type: 'NORMAL' | 'COLLECTIBLE';
  metaData?: {
    description?: string;
    imageUrl?: string;
    issuer?: string;
  };
}

export interface AssetTransfer {
  assetId: string;
  amount: number;
  destination: string;
}

export interface AssetInvoice {
  assetId: string;
  amount: number;
  description?: string;
  expiry?: number;
}

export interface ApiResponse<T> {
  success: boolean;
  data?: T;
  error?: string;
  message?: string;
}

export class TaprootService {
  private baseUrl: string;

  constructor(baseUrl: string = 'http://localhost:3000/api') {
    this.baseUrl = baseUrl;
  }

  async listAssets(): Promise<TaprootAsset[]> {
    try {
      const response = await fetch(`${this.baseUrl}/assets`);
      const result: ApiResponse<TaprootAsset[]> = await response.json();
      
      if (result.success && result.data) {
        return result.data;
      } else {
        throw new Error(result.error || 'Failed to fetch assets');
      }
    } catch (error) {
      console.error('Failed to list assets:', error);
      return [];
    }
  }

  async getAssetBalance(assetId: string): Promise<number> {
    try {
      const response = await fetch(`${this.baseUrl}/assets/${assetId}/balance`);
      const result: ApiResponse<number> = await response.json();
      
      if (result.success && result.data !== undefined) {
        return result.data;
      } else {
        throw new Error(result.error || 'Failed to fetch asset balance');
      }
    } catch (error) {
      console.error('Failed to get asset balance:', error);
      return 0;
    }
  }

  async sendAsset(transfer: AssetTransfer): Promise<string | null> {
    try {
      const response = await fetch(`${this.baseUrl}/assets/send`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(transfer),
      });

      const result: ApiResponse<string> = await response.json();
      
      if (result.success && result.data) {
        return result.data;
      } else {
        throw new Error(result.error || 'Failed to send asset');
      }
    } catch (error) {
      console.error('Failed to send asset:', error);
      return null;
    }
  }

  async createAssetInvoice(invoice: AssetInvoice): Promise<string | null> {
    try {
      const response = await fetch(`${this.baseUrl}/assets/invoice`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(invoice),
      });

      const result: ApiResponse<string> = await response.json();
      
      if (result.success && result.data) {
        return result.data;
      } else {
        throw new Error(result.error || 'Failed to create asset invoice');
      }
    } catch (error) {
      console.error('Failed to create asset invoice:', error);
      return null;
    }
  }

  async getTransactions(): Promise<any[]> {
    try {
      const response = await fetch(`${this.baseUrl}/transactions`);
      const result: ApiResponse<any[]> = await response.json();
      
      if (result.success && result.data) {
        return result.data;
      } else {
        throw new Error(result.error || 'Failed to fetch transactions');
      }
    } catch (error) {
      console.error('Failed to get transactions:', error);
      return [];
    }
  }
}