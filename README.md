# Lightning Taproot Assets Mobile App

A complete Lightning Network Taproot Assets mobile application built with React Native and Rust, featuring a unified backend that combines Lightning wallet functionality with Taproot Assets gateway capabilities.

## 🏗️ System Architecture

```mermaid
graph TB
    subgraph "Mobile App (React Native)"
        A[Dashboard] --> B[LNC Service]
        A --> C[Taproot Service]
        D[Settings] --> B
        E[Asset Management] --> C
        F[Transaction History] --> C
        B --> G[Lightning Node Connect]
        C --> H[REST API Client]
    end
    
    subgraph "Unified Rust Backend"
        H --> I[Axum Router]
        I --> J[/api/* - Lightning Wallet API]
        I --> K[/v1/taproot-assets/* - Gateway API]
        I --> L[/health - Health Checks]
        
        J --> M[API Handlers]
        K --> N[Gateway Handlers]
        
        M --> O[TapdClient]
        N --> O
        O --> P[Taproot Assets Daemon]
        
        M --> Q[Database Layer]
        N --> Q
        Q --> R[(PostgreSQL)]
    end
    
    subgraph "Lightning Infrastructure"
        G -.-> S[Lightning Node]
        P --> S
        S --> T[Bitcoin Network]
    end
    
    style A fill:#e1f5fe
    style I fill:#fff3e0
    style S fill:#f3e5f5
    style T fill:#e8f5e8
```

## 📋 Backend Architecture Deep Dive

### 🔧 Module Structure

```mermaid
graph LR
    subgraph "src/"
        A[main.rs] --> B[Axum Router Setup]
        
        subgraph "API Layer"
            C[api/] --> D[handlers.rs]
            C --> E[routes.rs]
            D --> F[Lightning Wallet Endpoints]
        end
        
        subgraph "Gateway Layer"
            G[gateway/] --> H[health.rs]
            G --> I[assets.rs]
            G --> J[addresses.rs]
            G --> K[wallet.rs]
            G --> L[info.rs]
            G --> M[routes.rs]
        end
        
        subgraph "Core Services"
            N[taproot/] --> O[client.rs - TapdClient]
            P[storage/] --> Q[database.rs]
            R[types.rs] --> S[Shared Types]
        end
        
        subgraph "Support Modules"
            T[config.rs] --> U[Configuration]
            V[crypto.rs] --> W[Cryptographic Utils]
            X[error.rs] --> Y[Error Handling]
        end
        
        B --> C
        B --> G
        O --> Z[Taproot Assets Daemon]
        Q --> AA[(PostgreSQL)]
    end
```

### 🚀 API Endpoints

#### Lightning Wallet API (`/api/*`)
```bash
GET  /api/assets                 # List all Taproot assets
GET  /api/assets/balance         # Get asset balances  
POST /api/assets/send            # Send assets
POST /api/assets/address         # Create asset address
POST /api/assets/mint            # Mint new assets
GET  /api/transactions           # Transaction history
```

#### Taproot Assets Gateway API (`/v1/taproot-assets/*`)
```bash
# Asset Management
GET  /v1/taproot-assets/assets/list    # List assets from daemon
POST /v1/taproot-assets/assets/mint    # Mint assets via daemon

# Address Management  
GET  /v1/taproot-assets/addresses/list # List addresses
POST /v1/taproot-assets/addresses/new  # Create new address

# Information & Health
GET  /v1/taproot-assets/info           # Daemon info
GET  /v1/taproot-assets/wallet/balance # Wallet balance
GET  /health                           # Health check
GET  /readiness                        # Readiness check
```

### 🔄 Data Flow Architecture

```mermaid
sequenceDiagram
    participant MA as Mobile App
    participant BE as Backend Router
    participant AH as API Handler
    participant GH as Gateway Handler
    participant TC as TapdClient
    participant TAD as Taproot Assets Daemon
    participant DB as PostgreSQL
    
    Note over MA,TAD: Asset Transfer Flow
    
    MA->>BE: POST /api/assets/send
    BE->>AH: Route to API Handler
    AH->>TC: Call TapdClient.send_asset()
    TC->>TAD: gRPC: SendAsset Request
    TAD-->>TC: Asset Transfer Response
    TC-->>AH: Transfer Details
    AH->>DB: Store Transaction
    AH-->>MA: Success Response
    
    Note over MA,TAD: Gateway API Flow
    
    MA->>BE: GET /v1/taproot-assets/assets/list
    BE->>GH: Route to Gateway Handler
    GH->>TC: Call TapdClient.list_assets()
    TC->>TAD: HTTP: List Assets Request
    TAD-->>TC: Assets Data
    TC-->>GH: Parsed Assets
    GH-->>MA: JSON Response
```

## 📱 Frontend Architecture

### 🧩 Component Hierarchy

```mermaid
graph TD
    A[App.tsx] --> B[Navigation]
    B --> C[Dashboard]
    B --> D[Settings]
    B --> E[Asset Management]
    
    C --> F[WalletBalance]
    C --> G[ChannelStatus]
    C --> H[NodeInfo]
    
    D --> I[LNCConnection]
    D --> J[NodeConfiguration]
    
    E --> K[AssetList]
    E --> L[AssetTransfer]
    E --> M[TransactionHistory]
    
    subgraph "Services"
        N[LNCService]
        O[TaprootService] 
        P[SecureStorage]
    end
    
    F --> N
    G --> N
    H --> N
    K --> O
    L --> O
    M --> O
    I --> P
```

### 🔐 Security Architecture

```mermaid
graph LR
    subgraph "Mobile App Security"
        A[User Input] --> B[SecureStore Encryption]
        B --> C[Encrypted Storage]
        A --> D[LNC Pairing]
        D --> E[End-to-End Encryption]
    end
    
    subgraph "Backend Security"
        F[CORS Protection] --> G[Axum Router]
        G --> H[Request Validation]
        H --> I[Authenticated Requests]
    end
    
    subgraph "Network Security"
        E --> J[WSS/HTTPS Only]
        I --> J
        J --> K[Lightning Network]
        J --> L[Taproot Assets Daemon]
    end
    
    style B fill:#ffebee
    style E fill:#ffebee
    style F fill:#e8f5e8
    style J fill:#fff3e0
```

## 🛠 Development Setup

### Prerequisites
- **Node.js 18+** and npm
- **Rust 1.70+** and Cargo
- **Expo CLI** (`npm install -g @expo/cli`)
- **PostgreSQL** (optional for development)

### Quick Start

1. **Clone and setup:**
   ```bash
   git clone <repository>
   cd lightning-app-mvp
   ```

2. **Start the unified backend:**
   ```bash
   cd backend
   cargo run
   ```
   🚀 Backend runs on http://localhost:3000

3. **Start the mobile app:**
   ```bash
   cd lightning-wallet-app
   npm install
   npm run dev
   ```
   📱 Access via Expo Go app or simulator

### Lightning Node Setup

```mermaid
graph LR
    A[Lightning Terminal] --> B[Generate Pairing Phrase]
    B --> C[litcli sessions add]
    C --> D[Enter in Mobile App]
    D --> E[Secure Connection]
    E --> F[Real Lightning Data]
```

To connect to a real Lightning node:

1. **Set up Lightning Terminal (litd)** with LNC enabled
2. **Generate a pairing phrase:**
   ```bash
   litcli sessions add --label="Mobile App" --type admin
   ```
3. **Enter credentials** in the app Settings → Connection

## 🔧 Configuration

### Backend Environment Variables
```bash
# Database
DATABASE_URL=postgresql://postgres:password@localhost:5432/taproot_assets

# Taproot Assets Integration
TAPROOT_GATEWAY_URL=http://127.0.0.1:8080

# Server Configuration  
SERVER_HOST=0.0.0.0
SERVER_PORT=3000

# Logging
RUST_LOG=info
```

### Frontend Configuration
```typescript
// App configuration in lightning-wallet-app/
const config = {
  BACKEND_URL: 'http://localhost:3000',
  LNC_WEBSOCKET: 'wss://your-node.com:8080',
  DEFAULT_TIMEOUT: 30000
};
```

## 🏗️ Database Schema

```mermaid
erDiagram
    TRANSACTIONS {
        uuid id PK
        string tx_type
        string asset_id
        bigint amount
        string status
        timestamp created_at
        timestamp updated_at
    }
    
    ASSET_BALANCES {
        string asset_id PK
        bigint balance
        timestamp updated_at
    }
    
    TAPROOT_ASSETS {
        string asset_id PK
        string name
        bigint balance
        int decimals
        string asset_type
        json meta_data
    }
    
    TRANSACTIONS ||--o{ TAPROOT_ASSETS : references
    ASSET_BALANCES ||--|| TAPROOT_ASSETS : tracks
```

## 📋 Current Status

### ✅ Completed Features

#### Backend Infrastructure
- [x] **Unified Rust Backend** - Combined Lightning wallet + Taproot gateway APIs
- [x] **Axum Web Framework** - High-performance async web server
- [x] **PostgreSQL Integration** - Database layer with SQLx
- [x] **CORS Configuration** - Mobile app communication
- [x] **Health Check Endpoints** - Service monitoring
- [x] **Error Handling** - Comprehensive error types and responses

#### Mobile Application
- [x] **React Native Foundation** - Cross-platform mobile app
- [x] **Lightning Node Connect** - Secure node connection via @lightninglabs/lnc-rn
- [x] **Real-time Dashboard** - Live Lightning node data display
- [x] **Secure Storage** - Encrypted credential management with Expo SecureStore  
- [x] **Professional UI** - Dark theme and responsive design
- [x] **Settings Management** - LNC pairing and configuration

#### Integration Layer
- [x] **TapdClient** - Taproot Assets daemon communication
- [x] **REST API Structure** - Complete endpoint definitions
- [x] **Type Definitions** - Shared Rust types for consistency
- [x] **Configuration Management** - Environment-based config

### 🚧 Integration Ready

#### Taproot Assets Features
- [ ] **Asset Minting** - Create new Taproot assets
- [ ] **Asset Transfers** - Send/receive via Lightning Network
- [ ] **Address Generation** - Create Taproot asset addresses
- [ ] **Transaction History** - Complete audit trail
- [ ] **Balance Tracking** - Real-time asset balances

#### Enhanced Features
- [ ] **Database Migrations** - Schema version management
- [ ] **gRPC Integration** - Direct Taproot Assets daemon communication
- [ ] **WebSocket Support** - Real-time updates
- [ ] **Advanced Error Handling** - User-friendly error messages
- [ ] **Testing Suite** - Comprehensive test coverage

## 🧪 Testing

### Backend API Testing
```bash
# Health checks
curl http://localhost:3000/health
curl http://localhost:3000/readiness

# Lightning Wallet API
curl http://localhost:3000/api/assets
curl http://localhost:3000/api/transactions

# Taproot Gateway API  
curl http://localhost:3000/v1/taproot-assets/info
curl http://localhost:3000/v1/taproot-assets/assets/list
```

### Mobile App Testing
```bash
cd lightning-wallet-app
npm run dev    # Start development server
npm test       # Run test suite (when implemented)
```

## 🔐 Security Features

### Multi-Layer Security

```mermaid
graph TD
    A[User Authentication] --> B[Secure Credential Storage]
    B --> C[End-to-End Encryption]
    C --> D[Network Security]
    D --> E[Backend Validation]
    
    A1[Password Protection] --> A
    A2[Biometric Auth] --> A
    
    B1[Expo SecureStore] --> B
    B2[Device Keychain] --> B
    
    C1[Lightning Node Connect] --> C
    C2[TLS/SSL] --> C
    
    D1[HTTPS/WSS Only] --> D
    D2[Certificate Pinning] --> D
    
    E1[Input Validation] --> E
    E2[Request Authentication] --> E
    
    style A fill:#ffebee
    style C fill:#e8f5e8
    style E fill:#fff3e0
```

- **🔐 End-to-end encryption** via Lightning Node Connect
- **🗝️ Encrypted credential storage** with user password protection
- **🚫 No plaintext sensitive data** storage on device
- **🔒 HTTPS/WSS communications** for all network requests
- **🧹 Secure pairing phrase handling** with automatic cleanup
- **✅ Input validation** and sanitization on all endpoints
- **🛡️ CORS protection** and request rate limiting

## 🚀 Deployment

### Backend Deployment
```bash
# Production build
cargo build --release

# Docker deployment (create Dockerfile)
docker build -t lightning-backend .
docker run -p 3000:3000 lightning-backend
```

### Mobile App Deployment
```bash
# Build for app stores
cd lightning-wallet-app
expo build:android
expo build:ios
```

## 📚 API Documentation

### Response Formats

All API responses follow this structure:

```typescript
interface ApiResponse<T> {
  success: boolean;
  data?: T;
  error?: string;
  message?: string;
}
```

### Error Codes

| Code | Description |
|------|-------------|
| 200  | Success |
| 400  | Bad Request - Invalid input |
| 401  | Unauthorized - Authentication required |
| 404  | Not Found - Resource doesn't exist |
| 500  | Internal Server Error |
| 503  | Service Unavailable - Taproot daemon unreachable |

---

**Built with ❤️ for the Lightning Network community**

*This project demonstrates the power of combining Lightning Network infrastructure with Taproot Assets for next-generation Bitcoin applications.*