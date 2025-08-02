import LNC from '@lightninglabs/lnc-rn';
import * as SecureStore from 'expo-secure-store';
import CryptoJS from 'crypto-js';

export interface LNCConfig {
  pairingPhrase?: string;
  password: string;
  serverHost?: string;
}

export interface StoredCredentials {
  pairingPhrase: string;
  password: string;
  serverHost: string;
  sessionKey?: string;
}

export interface NodeInfo {
  identityPubkey: string;
  alias: string;
  numActiveChannels: number;
  numInactiveChannels: number;
  blockHeight: number;
  syncedToChain: boolean;
}

export interface ChannelBalance {
  localBalance: { sat: number; msat: number };
  remoteBalance: { sat: number; msat: number };
}

export interface WalletBalance {
  totalBalance: number;
  confirmedBalance: number;
  unconfirmedBalance: number;
}

export class LNCService {
  private lnc: LNC | null = null;
  private isConnected: boolean = false;
  private credentials: StoredCredentials | null = null;

  constructor() {
    this.lnc = null;
  }

  async initialize(config: LNCConfig): Promise<boolean> {
    try {
      // Check for stored credentials first
      const storedCreds = await this.getStoredCredentials(config.password);
      
      if (storedCreds) {
        // Use stored credentials
        this.credentials = storedCreds;
      } else if (config.pairingPhrase) {
        // Use new pairing phrase
        this.credentials = {
          pairingPhrase: config.pairingPhrase,
          password: config.password,
          serverHost: config.serverHost || 'mailbox.terminal.lightning.today:443',
        };
        // Store credentials securely
        await this.storeCredentials(this.credentials, config.password);
      } else {
        throw new Error('No pairing phrase provided and no stored credentials found');
      }

      // Initialize LNC
      this.lnc = new LNC({
        pairingPhrase: this.credentials.pairingPhrase,
        serverHost: this.credentials.serverHost,
      });

      return true;
    } catch (error) {
      console.error('Failed to initialize LNC:', error);
      return false;
    }
  }

  async connect(): Promise<boolean> {
    if (!this.lnc) {
      throw new Error('LNC not initialized. Call initialize() first.');
    }

    try {
      await this.lnc.connect();
      this.isConnected = true;
      console.log('Connected to Lightning node via LNC');
      return true;
    } catch (error) {
      console.error('Failed to connect to Lightning node:', error);
      this.isConnected = false;
      return false;
    }
  }

  async disconnect(): Promise<void> {
    if (this.lnc && this.isConnected) {
      try {
        await this.lnc.disconnect();
        this.isConnected = false;
        console.log('Disconnected from Lightning node');
      } catch (error) {
        console.error('Error disconnecting:', error);
      }
    }
  }

  isNodeConnected(): boolean {
    return this.isConnected;
  }

  async getNodeInfo(): Promise<NodeInfo | null> {
    if (!this.lnc || !this.isConnected) {
      return null;
    }

    try {
      const info = await this.lnc.lnd.lightning.getInfo();
      return {
        identityPubkey: info.identityPubkey,
        alias: info.alias,
        numActiveChannels: info.numActiveChannels,
        numInactiveChannels: info.numInactiveChannels,
        blockHeight: info.blockHeight,
        syncedToChain: info.syncedToChain,
      };
    } catch (error) {
      console.error('Failed to get node info:', error);
      return null;
    }
  }

  async getWalletBalance(): Promise<WalletBalance | null> {
    if (!this.lnc || !this.isConnected) {
      return null;
    }

    try {
      const balance = await this.lnc.lnd.lightning.walletBalance();
      return {
        totalBalance: parseInt(balance.totalBalance),
        confirmedBalance: parseInt(balance.confirmedBalance),
        unconfirmedBalance: parseInt(balance.unconfirmedBalance),
      };
    } catch (error) {
      console.error('Failed to get wallet balance:', error);
      return null;
    }
  }

  async getChannelBalance(): Promise<ChannelBalance | null> {
    if (!this.lnc || !this.isConnected) {
      return null;
    }

    try {
      const balance = await this.lnc.lnd.lightning.channelBalance();
      return {
        localBalance: {
          sat: parseInt(balance.localBalance?.sat || '0'),
          msat: parseInt(balance.localBalance?.msat || '0'),
        },
        remoteBalance: {
          sat: parseInt(balance.remoteBalance?.sat || '0'),
          msat: parseInt(balance.remoteBalance?.msat || '0'),
        },
      };
    } catch (error) {
      console.error('Failed to get channel balance:', error);
      return null;
    }
  }

  async listChannels(): Promise<any[]> {
    if (!this.lnc || !this.isConnected) {
      return [];
    }

    try {
      const channels = await this.lnc.lnd.lightning.listChannels();
      return channels.channels || [];
    } catch (error) {
      console.error('Failed to list channels:', error);
      return [];
    }
  }

  async createInvoice(amount: number, memo?: string): Promise<string | null> {
    if (!this.lnc || !this.isConnected) {
      return null;
    }

    try {
      const invoice = await this.lnc.lnd.lightning.addInvoice({
        value: amount.toString(),
        memo: memo || '',
      });
      return invoice.paymentRequest;
    } catch (error) {
      console.error('Failed to create invoice:', error);
      return null;
    }
  }

  async payInvoice(paymentRequest: string): Promise<boolean> {
    if (!this.lnc || !this.isConnected) {
      return false;
    }

    try {
      await this.lnc.lnd.lightning.sendPaymentSync({
        paymentRequest,
      });
      return true;
    } catch (error) {
      console.error('Failed to pay invoice:', error);
      return false;
    }
  }

  private async storeCredentials(credentials: StoredCredentials, password: string): Promise<void> {
    try {
      const encrypted = CryptoJS.AES.encrypt(JSON.stringify(credentials), password).toString();
      await SecureStore.setItemAsync('lnc_credentials', encrypted);
    } catch (error) {
      console.error('Failed to store credentials:', error);
      throw error;
    }
  }

  private async getStoredCredentials(password: string): Promise<StoredCredentials | null> {
    try {
      const encrypted = await SecureStore.getItemAsync('lnc_credentials');
      if (!encrypted) {
        return null;
      }

      const decryptedBytes = CryptoJS.AES.decrypt(encrypted, password);
      const decryptedData = decryptedBytes.toString(CryptoJS.enc.Utf8);
      
      if (!decryptedData) {
        throw new Error('Invalid password');
      }

      return JSON.parse(decryptedData);
    } catch (error) {
      console.error('Failed to retrieve stored credentials:', error);
      return null;
    }
  }

  async clearStoredCredentials(): Promise<void> {
    try {
      await SecureStore.deleteItemAsync('lnc_credentials');
    } catch (error) {
      console.error('Failed to clear stored credentials:', error);
    }
  }
}