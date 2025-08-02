# Taproot Assets Mobile App - Complete Project Context

## Project Overview

We are building a mobile application that allows users to interact with the Lightning Network, specifically focused on managing Taproot Assets (formerly known as Taro). The app enables users to send, receive, and manage Bitcoin-based assets using the Lightning Network infrastructure.

## Technology Stack

### Frontend
- **React Native** with TypeScript
- **Lightning Node Connect (LNC)** via `@lightninglabs/lnc-rn` npm package
- **react-native-encrypted-storage** for secure credential storage
- **crypto-js** for additional encryption utilities

### Backend
- **Rust** with axum web framework
- **Taproot Assets daemon (tapd)** integration via gRPC
- **SQLx** for PostgreSQL database operations
- **tonic** for gRPC communication

### Development Environment
- **Polar** for Bitcoin and Lightning node management
- **Lightning Terminal (litd)** for LNC pairing phrase generation
- Local development setup with testnet integration

## User Requirements & Target Audience

### Target Users
- Bitcoin asset-interested users
- People wanting to manage Taproot Assets on mobile
- Lightning Network enthusiasts
- Users comfortable with technical Bitcoin concepts

### Core Use Cases
- Managing Taproot Assets portfolio
- Sending and receiving assets via Lightning Network
- Viewing asset metadata and transaction history
- Secure connection to personal Lightning nodes

### User Experience Level
- Project owner is "very very new" to Lightning Network development
- Need for educational progression and learning resources
- Emphasis on clear documentation and setup instructions

## Lightning Node Connect (LNC) Integration

### What is LNC?
Lightning Node Connect is a protocol that allows safe and easy connection between applications and Lightning nodes without requiring port exposure. Key features:

- **No port opening required** on the node
- **End-to-end encrypted** connections
- **Proxy server (mailbox)** handles routing
- **Pairing phrase authentication** system

### LNC Workflow
1. **User generates pairing phrase** in their Lightning node (via litd)
2. **User enters pairing phrase** into mobile app with password
3. **Persistent encrypted connection** established through mailbox
4. **Subsequent connections** require only password

### LNC Configuration
```typescript
const lnc = new LNC({
  pairingPhrase: 'artefact morning piano photo consider light',
  serverHost: 'mailbox.terminal.lightning.today:443' // default
});
```

### Security Features
- Credentials encrypted at rest with user password
- Long-term Diffie-Hellman keys for session management
- Secure proxy communication (mailbox unable to see content)
- Biometric authentication support planned

## Architecture Design

### High-Level System Architecture
```
Mobile App (React Native)
    ↓ (HTTPS/WSS)
Lightning Node Connect Proxy
    ↓ (Encrypted)
Lightning Node (LND + litd)
    ↓ (gRPC)
Taproot Assets Daemon (tapd)
```

### Frontend Architecture
```
src/
├── components/           # Reusable UI components
│   ├── AssetCard.tsx    # Individual asset display
│   ├── AssetList.tsx    # Asset portfolio view
│   ├── ConnectionStatus.tsx # LNC status indicator
│   └── SendReceiveModal.tsx # Transaction modals
├── screens/             # Main application screens
│   ├── ConnectScreen.tsx    # LNC pairing interface
│   ├── WalletScreen.tsx     # Main portfolio view
│   ├── AssetsScreen.tsx     # Asset management
│   └── SettingsScreen.tsx   # App configuration
├── services/            # Core business logic
│   ├── LNCService.ts        # Lightning Node Connect integration
│   ├── TaprootService.ts    # Taproot Assets API calls
│   └── StorageService.ts    # Secure credential storage
├── hooks/               # React hooks for state management
│   ├── useLNC.ts           # LNC connection management
│   ├── useAssets.ts        # Asset state management
│   └── useWallet.ts        # Wallet state management
└── types/               # TypeScript definitions
    ├── assets.ts           # Taproot asset types
    ├── lightning.ts        # LN data structures
    └── api.ts             # API response types
```

### Backend Architecture
```
src/
├── main.rs              # Server entry point and routing
├── taproot/             # Taproot Assets integration
│   ├── mod.rs
│   ├── assets.rs        # Asset management logic
│   └── daemon.rs        # tapd gRPC client
├── lightning/           # Lightning Network utilities
│   ├── mod.rs
│   └── client.rs        # LND integration helpers
├── api/                 # REST API implementation
│   ├── mod.rs
│   ├── routes.rs        # Endpoint definitions
│   └── handlers.rs      # Request handlers
└── storage/             # Database operations
    ├── mod.rs
    └── database.rs      # PostgreSQL operations
```

## Core Features & Functionality

### 1. Wallet Management
- **Asset Portfolio View**: Display all Taproot assets with current balances
- **Lightning Balance**: Show BTC balance and channel states
- **Asset Metadata**: Names, descriptions, icons for each asset type  
- **Transaction History**: Unified view of asset and Lightning transactions
- **Real-time Updates**: Live balance updates via LNC subscriptions

### 2. Lightning Network Operations
- **Node Connection**: Secure connection via Lightning Node Connect
- **Channel Management**: View and manage Lightning channels
- **Invoice Management**: Create and track Lightning invoices
- **Payment Routing**: Efficient routing for asset transfers
- **Network Status**: Connection status and node information display

### 3. Taproot Asset Operations
- **Send Assets**: Transfer Taproot assets via Lightning Network
- **Receive Assets**: Generate invoices for asset payments  
- **Asset Discovery**: Browse and add new asset types
- **Multi-Asset Support**: Handle multiple asset types simultaneously
- **Asset Issuance**: Future capability for creating new assets

### 4. Security & Storage
- **Encrypted Credentials**: Secure storage of LNC pairing phrases
- **Biometric Authentication**: Face/Touch ID for app access (planned)
- **Seed Backup**: Secure backup and recovery options (planned)
- **PIN Protection**: Additional security layer (planned)

## Technical Implementation Details

### LNC Service Implementation
```typescript
export class LNCService {
  private lnc: LNC | null = null;
  private eventEmitter: NativeEventEmitter;

  async initialize(config: LNCConfig): Promise<boolean> {
    // Check for stored credentials or use new pairing phrase
    // Initialize LNC instance with configuration
    // Store credentials securely
  }

  async connect(): Promise<boolean> {
    // Establish connection to Lightning node
    // Handle connection errors and retries
  }

  // Lightning operations: getWalletBalance, listChannels, createInvoice, etc.
  // Subscription management for real-time updates
}
```

### Taproot Assets Service
```typescript
export class TaprootService {
  private baseUrl: string;

  async listAssets(): Promise<TaprootAsset[]> {
    // Fetch assets from Rust backend
  }

  async sendAsset(transfer: AssetTransfer): Promise<string> {
    // Send asset via Lightning Network
  }

  async createAssetInvoice(assetId: string, amount: number): Promise<string> {
    // Generate Lightning invoice for asset payment
  }
}
```

### Secure Storage Implementation
```typescript
export class StorageService {
  static async storeCredentials(credentials: StoredCredentials, password: string) {
    // Encrypt credentials with user password
    // Store in react-native-encrypted-storage
  }

  static async getCredentials(password: string): Promise<StoredCredentials | null> {
    // Retrieve and decrypt stored credentials
  }
}
```

### Rust Backend API Endpoints
```rust
// REST API endpoints
GET  /api/assets                    // List all assets
GET  /api/assets/{id}/balance       // Get asset balance
POST /api/assets/send               // Send asset
POST /api/assets/invoice            // Create asset invoice
GET  /api/transactions              // Transaction history
GET  /api/health                    // Health check
```

## Development Phases

### Phase 1: Foundation (Weeks 1-2)
**Objectives**: Basic project setup and LNC integration
- Initialize React Native project with TypeScript
- Install and configure @lightninglabs/lnc-rn
- Set up Rust backend with axum framework
- Configure Polar environment with Taproot Assets
- Implement basic LNC connection service
- Create secure credential storage system

**Deliverables**:
- Working React Native app with LNC connection
- Basic Rust backend structure
- Encrypted credential storage
- Simple wallet balance display

### Phase 2: Core Functionality (Weeks 3-5)
**Objectives**: Asset management and transaction capabilities
- Integrate with Taproot Assets daemon
- Implement asset listing and metadata display
- Build send/receive asset functionality
- Create transaction history tracking
- Develop error handling and validation

**Deliverables**:
- Complete asset management system
- Working send/receive functionality
- Transaction history display
- Robust error handling

### Phase 3: UI/UX Polish (Weeks 6-7)
**Objectives**: Professional interface and user experience
- Design asset management interface
- Create intuitive send/receive flows
- Build channel management screens
- Implement settings and configuration
- Add loading states and offline capability
- Performance optimization

**Deliverables**:
- Polished user interface
- Smooth user experience
- Professional design system
- Performance optimizations

### Phase 4: Advanced Features (Weeks 8-10)
**Objectives**: Production readiness and advanced capabilities
- Asset issuance capabilities (if applicable)
- Multi-signature support
- Asset trading/swapping features
- Advanced routing options
- Comprehensive testing suite
- Security audit preparation

**Deliverables**:
- Advanced feature set
- Production-ready application
- Comprehensive test coverage
- Security documentation

## Development Environment Setup

### Prerequisites
1. **Polar**: Bitcoin and Lightning node management tool
2. **Taproot Assets**: Install `tapd` daemon for asset management
3. **React Native**: Mobile development environment (iOS/Android)
4. **Rust**: Backend development toolchain
5. **Lightning Terminal**: For LNC pairing phrase generation

### Local Development Workflow
1. **Start Polar** with Bitcoin and Lightning nodes
2. **Run Taproot Assets daemon** connected to Polar nodes
3. **Start Rust backend server** with tapd integration
4. **Run React Native app** in simulator/device
5. **Connect app** to local Lightning node via LNC

### Environment Configuration
```bash
# React Native setup
npx react-native init TaprootAssetsApp --template react-native-template-typescript
npm install @lightninglabs/lnc-rn react-native-encrypted-storage crypto-js

# LNC library setup
cd node_modules/@lightninglabs/lnc-rn
yarn run fetch-libraries

# Rust backend setup
cargo init taproot-backend
# Configure Cargo.toml with required dependencies
```

### LNC Pairing Setup
```bash
# Generate pairing phrase in Lightning Terminal
litcli --lndtlscertpath ~/.lit/tls.cert sessions add --label="Mobile App" --type admin
```

## Security Considerations

### Credential Management
- **Encrypted Storage**: All sensitive data encrypted with user password
- **No Plaintext Storage**: Pairing phrases never stored in plaintext
- **Secure Transmission**: End-to-end encryption via LNC protocol
- **Key Rotation**: Support for credential refresh and rotation

### Network Security
- **Certificate Pinning**: Implement for production deployment
- **TLS Encryption**: All API communications over HTTPS
- **Proxy Security**: Mailbox server unable to decrypt communications
- **Input Validation**: Comprehensive validation of all user inputs

### Asset Security
- **Transaction Verification**: Multi-step confirmation for asset transfers
- **Asset Validation**: Verify asset authenticity and metadata
- **Balance Verification**: Cross-check balances with multiple sources
- **Audit Trail**: Complete transaction logging and history

## API Design & Data Structures

### Taproot Asset Data Structure
```typescript
interface TaprootAsset {
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
```

### Lightning Network Data Structures
```typescript
interface NodeInfo {
  identityPubkey: string;
  alias: string;
  numActiveChannels: number;
  numInactiveChannels: number;
  blockHeight: number;
  syncedToChain: boolean;
}

interface ChannelBalance {
  localBalance: { sat: number; msat: number };
  remoteBalance: { sat: number; msat: number };
}
```

### API Response Formats
```typescript
interface ApiResponse<T> {
  success: boolean;
  data?: T;
  error?: string;
  message?: string;
}
```

## Testing Strategy

### Unit Testing
- **Service Layer**: Test all business logic functions
- **Utility Functions**: Test encryption, formatting, validation
- **React Hooks**: Test state management and side effects
- **API Handlers**: Test Rust backend endpoints

### Integration Testing  
- **LNC Integration**: Test connection and communication
- **Taproot Assets**: Test daemon integration
- **Database Operations**: Test data persistence
- **End-to-End Workflows**: Test complete user journeys

### Security Testing
- **Credential Handling**: Test encryption/decryption
- **Network Communication**: Test TLS and certificate validation
- **Input Validation**: Test against malicious inputs
- **Authentication**: Test pairing and session management

## Documentation Requirements

### User Documentation
- **Setup Guide**: Step-by-step installation and configuration
- **User Manual**: How to use all app features
- **Troubleshooting**: Common issues and solutions
- **Security Best Practices**: User security recommendations

### Developer Documentation
- **API Reference**: Complete backend API documentation
- **Architecture Guide**: System design and component interactions
- **Deployment Guide**: Production deployment instructions
- **Contributing Guide**: Development workflow and standards

## Future Enhancements

### Short-term (3-6 months)
- **Advanced Asset Features**: Asset creation and complex transfers
- **Multi-signature Support**: Enhanced security for high-value operations
- **Asset Trading**: Peer-to-peer asset exchange
- **Mobile Optimizations**: Performance and battery usage improvements

### Long-term (6-12 months)
- **DeFi Integration**: Decentralized finance protocol support
- **Multi-chain Support**: Support for other blockchain networks
- **Advanced Analytics**: Portfolio tracking and analytics
- **Social Features**: Asset sharing and community features

## Success Metrics

### Technical Metrics
- **Connection Reliability**: >99% LNC connection success rate
- **Transaction Success**: >95% asset transaction success rate
- **Performance**: <3 second app load times
- **Security**: Zero security incidents in production

### User Experience Metrics
- **User Adoption**: Target user acquisition and retention rates
- **Feature Usage**: Track usage of core features
- **User Satisfaction**: Regular user feedback and ratings
- **Support Requests**: Monitor and minimize support issues

## Risk Assessment & Mitigation

### Technical Risks
- **LNC Dependency**: Reliance on Lightning Labs infrastructure
  - *Mitigation*: Implement fallback connection methods
- **Taproot Assets Maturity**: Early-stage protocol
  - *Mitigation*: Thorough testing and gradual rollout
- **Mobile Platform Changes**: iOS/Android updates breaking compatibility
  - *Mitigation*: Regular testing and update schedule

### Security Risks
- **Key Management**: User credential compromise
  - *Mitigation*: Strong encryption and user education
- **Network Attacks**: Man-in-the-middle attacks
  - *Mitigation*: Certificate pinning and secure defaults
- **Asset Loss**: User error in transactions
  - *Mitigation*: Clear UX and confirmation flows

### Business Risks
- **Regulatory Changes**: Changing cryptocurrency regulations
  - *Mitigation*: Legal consultation and compliance planning
- **Market Adoption**: Slow Taproot Assets adoption
  - *Mitigation*: Focus on core Lightning functionality initially
- **Competition**: Similar apps entering market
  - *Mitigation*: Focus on superior UX and unique features

## Project Status & Next Steps

### Current Status
- **Architecture Design**: Complete ✅
- **Core Code Structure**: Complete ✅
- **Development Plan**: Complete ✅
- **Documentation**: Complete ✅

### Immediate Next Steps
1. **Environment Setup**: Set up development environment with Polar and tapd
2. **Basic Implementation**: Implement core LNC connection functionality
3. **Testing**: Test basic app functionality with local Lightning node
4. **Backend Development**: Begin Rust backend implementation
5. **Integration**: Connect frontend to backend services

### Success Criteria for Next Phase
- Successfully connect mobile app to Lightning node via LNC
- Display basic wallet balance and node information
- Implement secure credential storage and retrieval
- Establish communication between React Native app and Rust backend
- Complete basic asset listing functionality

This comprehensive context document captures the complete scope, technical details, and implementation plan for the Taproot Assets mobile application project.