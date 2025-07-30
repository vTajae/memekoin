# Trading Integration with Barter-rs

This document describes the trading functionality integrated into the Axum Worker using the barter-rs framework.

## Overview

The trading system provides a comprehensive API for cryptocurrency trading operations, including:

- Market data retrieval (quotes, order books)
- Order placement and management
- Portfolio balance tracking
- Multi-exchange support
- Real-time trading capabilities

## Architecture

### Core Components

1. **Trading Client** (`src/clients/trading.rs`)
   - Handles direct communication with exchange APIs
   - Supports multiple exchanges (Binance, Coinbase, Kraken, OKX, Bybit)
   - Provides unified interface for trading operations

2. **Trading Service** (`src/service/trading.rs`)
   - Business logic layer for trading operations
   - Orchestrates trading client calls
   - Handles data transformation and validation

3. **Trading Handlers** (`src/handler/trading.rs`)
   - HTTP request handlers for trading endpoints
   - Input validation and response formatting
   - Error handling and logging

4. **Trading Entities** (`src/entity/trading.rs`)
   - Data models for trading operations
   - Portfolio, orders, instruments, and market data structures

5. **Trading DTOs** (`src/dto/trading.rs`)
   - Request/response data transfer objects
   - API contract definitions

## Supported Exchanges

- **Binance** - Spot and futures trading
- **Coinbase Pro** - Spot trading
- **Kraken** - Spot and futures trading
- **OKX** - Multi-asset trading
- **Bybit** - Derivatives trading

## API Endpoints

### Market Data

#### Get Quote
```
POST /api/trading/quote
```
Request:
```json
{
  "exchange": "binance",
  "symbol": "BTCUSDT"
}
```

Response:
```json
{
  "symbol": "BTCUSDT",
  "exchange": "binance",
  "bid_price": "45000.50",
  "ask_price": "45001.25",
  "spread": "0.75",
  "timestamp": "2024-01-01T12:00:00Z"
}
```

#### Get Order Book
```
POST /api/trading/orderbook
```
Request:
```json
{
  "exchange": "binance",
  "symbol": "BTCUSDT",
  "depth": 20
}
```

### Trading Operations

#### Place Order
```
POST /api/trading/order
```
Request:
```json
{
  "exchange": "binance",
  "symbol": "BTCUSDT",
  "side": "BUY",
  "order_type": "LIMIT",
  "quantity": "0.001",
  "price": "45000.00"
}
```

#### Get Balances
```
POST /api/trading/balances
```
Request:
```json
{
  "exchange": "binance"
}
```

### Configuration

#### Get Trading Status
```
POST /api/trading/status
```

#### Get Trading Configuration
```
GET /api/trading/config
```

#### Health Check
```
GET /api/trading/health
```

## Configuration

### Environment Variables

Add the following to your `wrangler.toml`:

```toml
[vars]
# Binance API Keys
BINANCE_API_KEY = "your-binance-api-key"
BINANCE_API_SECRET = "your-binance-api-secret"

# Coinbase API Keys
COINBASE_API_KEY = "your-coinbase-api-key"
COINBASE_API_SECRET = "your-coinbase-api-secret"
COINBASE_PASSPHRASE = "your-coinbase-passphrase"

# Kraken API Keys
KRAKEN_API_KEY = "your-kraken-api-key"
KRAKEN_API_SECRET = "your-kraken-api-secret"
```

### Security Notes

- API keys are stored as environment variables
- All trading operations require proper authentication
- Sandbox mode is enabled by default for safety
- Rate limiting is implemented per exchange requirements

## Features

### Market Data
- Real-time price quotes
- Order book depth data
- Trading instrument information
- Market status monitoring

### Trading Operations
- Market and limit orders
- Stop-loss and take-profit orders
- Order status tracking
- Trade execution history

### Portfolio Management
- Multi-asset balance tracking
- Portfolio value calculation
- Position monitoring
- P&L calculation

### Risk Management
- Order validation
- Balance checks
- Rate limiting
- Error handling and recovery

## Development

### Testing

The system includes comprehensive testing capabilities:

1. **Mock Responses** - Simulated exchange responses for development
2. **Sandbox Mode** - Safe testing environment
3. **Error Simulation** - Test error handling scenarios

### Extending Support

To add a new exchange:

1. Add exchange to the `Exchange` enum
2. Implement exchange-specific URL building
3. Add authentication logic
4. Update response parsing
5. Add to supported exchanges list

## Monitoring

### Logging

All trading operations are logged with:
- Request/response details
- Error conditions
- Performance metrics
- Security events

### Health Checks

- Trading service health endpoint
- Exchange connectivity status
- API key validation
- Rate limit monitoring

## Error Handling

The system provides comprehensive error handling:

- Network connectivity issues
- API rate limiting
- Invalid credentials
- Malformed requests
- Exchange-specific errors

## Performance

- Asynchronous operations
- Connection pooling
- Response caching
- Optimized data structures

## Security

- Secure API key storage
- Request signing
- Input validation
- Rate limiting
- Audit logging

## Future Enhancements

- WebSocket streaming data
- Advanced order types
- Algorithmic trading strategies
- Portfolio optimization
- Risk analytics
- Multi-exchange arbitrage

## Support

For issues or questions regarding the trading integration:

1. Check the logs for detailed error information
2. Verify API key configuration
3. Test with sandbox mode first
4. Review exchange-specific documentation
