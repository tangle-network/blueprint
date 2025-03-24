# Tangle Cloud Pricing Engine

The Tangle Cloud Pricing Engine is a flexible system for pricing cloud services in a decentralized marketplace. It enables service operators to define pricing models, handle price calculations, and participate in Request for Quote (RFQ) negotiations with potential customers.

## Overview

The pricing engine supports both direct pricing calculations and a decentralized request-for-quote (RFQ) system. It is designed to integrate with Tangle's existing blockchain infrastructure where operators register for blueprint services and users can request services from specific operators.

### Key Features

-   **Flexible Pricing Models**: Support for fixed, usage-based, and tiered pricing strategies
-   **Resource-Based Calculations**: Price calculations based on specific resource requirements
-   **Request for Quote (RFQ)**: Decentralized protocol for price discovery
-   **Blockchain Integration**: On-chain settlement of price quotes and service activation
-   **Operator Management**: Tools for operators to manage their pricing strategies
-   **User Selection**: Mechanisms for users to select operators based on various criteria

## User Experience Flows

The pricing engine supports multiple user flows for service discovery, pricing, and activation:

### 1. Direct Request Flow

In this flow, users directly request services from specific operators on-chain:

1. Users identify specific operators they want to work with
2. Users submit on-chain requests specifying these operators
3. Operators receive the request and calculate prices using the pricing engine
4. Operators approve or reject based on their policies
5. When the approval threshold is met, the service instance starts

The pricing engine augments this flow by providing transparent pricing information to help users make informed decisions and operators to consistently price their services.

### 2. RFQ-Based Selection Flow

In this flow, users broadcast requests to discover and select operators based on price quotes:

1. Users send an RFQ to the network (using `pricing_requestForQuote` RPC)
2. Multiple operators respond with signed price quotes
3. Users evaluate quotes based on their criteria (price, SLA, reputation, etc.)
4. Users select operators and submit the signed quotes on-chain
5. Service can start immediately since quotes already represent operator approvals

This flow enables a more competitive marketplace where users can select the best operators for their needs based on real-time price quotes.

## Integration with Existing On-Chain Mechanisms

The pricing engine enhances Tangle's existing on-chain service request and approval mechanisms:

### Current On-Chain Flow

In the current system:

1. Operators register for blueprints, indicating their ability to provide specific services
2. Users submit on-chain requests for services, specifying which operators they want to use
3. Operators approve or reject these requests based on their internal policies
4. When the required threshold of operator approvals is met, the service instance is created and activated
5. Payments are processed according to the blockchain's payment mechanisms

### How Pricing Engine Enhances This System

The pricing engine acts as a layer that:

1. **Adds Pricing Transparency**: Allows operators to define and publish their pricing strategies
2. **Enables Price Discovery**: Lets users discover and compare prices from different operators
3. **Streamlines Approvals**: Provides mechanisms for pre-approved quotes that can be submitted on-chain
4. **Ensures Consistency**: Guarantees that operators use consistent pricing calculations

### On-Chain Settlement with RFQ

The RFQ system enhances on-chain settlement through:

1. **Signed Quotes**: Operators sign their price quotes, which can be verified on-chain
2. **Immediate Execution**: Users can submit signed quotes with their service requests, enabling immediate execution without waiting for operator approvals
3. **Price Guarantees**: Quotes include expiration timestamps, ensuring price stability for a defined period

## Architecture

The pricing engine consists of the following core components:

1. **Pricing Models**: Definition of pricing strategies and resource pricing
2. **Calculation Engine**: Core algorithms for calculating prices based on models
3. **RFQ System**: Protocol for decentralized price discovery
4. **Service Integration**: Management of service lifecycle and blockchain integration
5. **RPC Interface**: API for external communication with users and other systems

### System Architecture Diagram

```
┌─────────────────┐     ┌─────────────────┐     ┌─────────────────┐
│                 │     │                 │     │                 │
│  User Interface │────▶│  Pricing Engine │────▶│  Blockchain     │
│                 │     │                 │     │                 │
└─────────────────┘     └─────────────────┘     └─────────────────┘
         │                       │                       │
         │                       │                       │
         ▼                       ▼                       ▼
┌─────────────────┐     ┌─────────────────┐     ┌─────────────────┐
│                 │     │                 │     │                 │
│  RFQ System     │◀───▶│  Networking     │◀───▶│  Smart Contracts│
│                 │     │                 │     │                 │
└─────────────────┘     └─────────────────┘     └─────────────────┘
```

### Pricing Models

The system supports multiple pricing model types:

-   **Fixed Pricing**: Simple fixed prices for services
-   **Usage-Based Pricing**: Dynamic pricing based on resource consumption
-   **Tiered Pricing**: Different rates at different usage levels

Each model can be associated with resource-specific pricing details including:

-   Resource units (CPU, memory, storage, network, etc.)
-   Base pricing and scaling factors
-   Time periods for recurring charges
-   Minimum and maximum quantities

### RFQ Protocol

The RFQ protocol enables:

1. Broadcasting quote requests to multiple operators
2. Signed responses that can be verified on-chain
3. Selection mechanisms for users to choose operators
4. Integration with existing blockchain settlement

### RFQ Flow Diagram

```
┌─────────┐     ┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│         │     │             │     │             │     │             │
│  User   │────▶│  RFQ Query  │────▶│  Network    │────▶│  Operators  │
│         │     │             │     │             │     │             │
└─────────┘     └─────────────┘     └─────────────┘     └─────────────┘
                                                               │
┌─────────┐     ┌─────────────┐     ┌─────────────┐           │
│         │     │             │     │             │           │
│  User   │◀────│  Selection  │◀────│  Quotes     │◀──────────┘
│         │     │             │     │             │
└─────────┘     └─────────────┘     └─────────────┘
     │
     │
     ▼
┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│             │     │             │     │             │
│  On-Chain   │────▶│  Service    │────▶│  Payment    │
│  Settlement │     │  Activation │     │  Processing │
│             │     │             │     │             │
└─────────────┘     └─────────────┘     └─────────────┘
```

## API Reference

The pricing engine exposes several RPC methods:

### For Operators (Service Providers)

-   `pricing_getOperatorInfo`: Returns information about the operator
-   `pricing_getPricingModels`: Returns available pricing models
-   `pricing_calculatePrice`: Calculates price for a specific service request

### For Users (Service Requesters)

-   `pricing_requestForQuote`: Broadcasts an RFQ to available operators
-   `pricing_getRfqResults`: Retrieves quotes from operators

## User Selection Criteria

The RFQ system supports various selection criteria for users:

1. **Price-Based Selection**: Choose operators offering the lowest prices
2. **Quality-Based Selection**: Select based on reputation, SLAs, or other quality metrics
3. **Geographic Selection**: Filter by operator region
4. **Custom Criteria**: Combine multiple factors for personalized selection

## Requirements Tracking

This section will track key product requirements to ensure they are addressed during development:

| ID    | Requirement                               | Status      | Notes                          |
| ----- | ----------------------------------------- | ----------- | ------------------------------ |
| PR-01 | Support for multiple pricing models       | Implemented | Fixed, usage-based, and tiered |
| PR-02 | RFQ protocol implementation               | In Progress | Basic framework done           |
| PR-03 | Signature verification for quotes         | Planned     | Security enhancement           |
| PR-04 | Direct blockchain integration             | Planned     | For on-chain settlement        |
| PR-05 | Support for complex resource requirements | Planned     | For advanced services          |
| PR-06 | Geographic filtering for operators        | Planned     | For region-specific selection  |
| PR-07 | User-defined selection criteria           | Planned     | For custom operator selection  |
| PR-08 | Quote expiration and validity checks      | Implemented | For price guarantees           |
