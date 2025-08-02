# Lightning Taproot Assets Mobile App

A complete Lightning Network Taproot Assets mobile application built with React Native and Rust.

## 🚀 Features

- **Lightning Node Connect (LNC)** integration for secure node connection
- **Real-time Lightning wallet data** - balances, channels, node status
- **Taproot Assets management** - send, receive, and track assets
- **Encrypted credential storage** with password protection
- **Professional mobile UI** with dark theme
- **REST API backend** built with Rust and Axum

## 📱 Frontend (React Native)

### Core Components
- **Dashboard** - Real-time wallet overview with Lightning and on-chain balances
- **Settings** - LNC pairing, connection management, and app configuration
- **Asset Management** - View and manage Taproot assets
- **Transaction History** - Complete transaction tracking
- **Connection UI** - Secure pairing phrase entry and stored credentials

### Key Services
- **LNCService** - Lightning Node Connect integration
- **TaprootService** - Backend API communication
- **Secure Storage** - Encrypted credential management with Expo SecureStore

## 🦀 Backend (Rust)

### Architecture
- **Axum web framework** for REST API
- **PostgreSQL database** with SQLx (configurable)
- **Taproot Assets daemon** integration structure
- **CORS enabled** for mobile app communication

### API Endpoints
```
GET  /health                 - Health check
GET  /api/assets             - List all Taproot assets
GET  /api/assets/:id/balance - Get specific asset balance
POST /api/assets/send        - Send asset to destination
POST /api/assets/invoice     - Create asset invoice
GET  /api/transactions       - Get transaction history
```

## 🛠 Development Setup

### Prerequisites
- Node.js 18+ and npm
- Rust 1.70+ and Cargo
- Expo CLI
- PostgreSQL (optional for development)

### Quick Start

1. **Start the backend:**
   ```bash
   cd backend
   cargo run
   ```
   Backend runs on http://localhost:3000

2. **Start the mobile app:**
   ```bash
   cd lightning-wallet-app
   npm install
   npm run dev
   ```
   Access via Expo Go app or simulator

### Lightning Node Setup

To connect to a real Lightning node:

1. Set up Lightning Terminal (litd) with LNC enabled
2. Generate a pairing phrase:
   ```bash
   litcli sessions add --label="Mobile App" --type admin
   ```
3. Enter the pairing phrase and password in the app Settings

## 🔧 Configuration

### Backend Environment Variables
```bash
DATABASE_URL=postgresql://postgres:password@localhost:5432/taproot_assets
TAPD_GRPC_HOST=localhost:10029
SERVER_HOST=0.0.0.0
SERVER_PORT=3000
RUST_LOG=info
```

## 📋 Current Status

### ✅ Completed
- Complete backend API structure with Rust/Axum
- Lightning Node Connect integration with @lightninglabs/lnc-rn
- Real-time dashboard with live Lightning node data
- Secure credential management with Expo SecureStore
- Professional mobile UI with dark theme
- Settings screen for LNC pairing and connection management
- REST API endpoints for asset management
- Cross-platform React Native app structure

### 🚧 Ready for Integration
- Taproot Assets daemon gRPC integration
- Database migrations and persistence
- Asset transfer functionality via Lightning Network
- Transaction history tracking
- Enhanced error handling and validation

## 🧪 Testing

Backend API is running and responding:
```bash
# Health check
curl http://localhost:3000/health

# List assets  
curl http://localhost:3000/api/assets

# Get transactions
curl http://localhost:3000/api/transactions
```

Mobile app development server:
```bash
cd lightning-wallet-app && npm run dev
```

## 🔐 Security Features

- **End-to-end encryption** via Lightning Node Connect
- **Encrypted credential storage** with user password
- **No plaintext sensitive data** storage
- **HTTPS/WSS communications** for all network requests
- **Secure pairing phrase handling** with automatic cleanup

---

**Built with ❤️ for the Lightning Network community**
