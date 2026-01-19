# Blueprint Portal Specification

## Overview
The Blueprint Portal is a local operator dashboard for managing Blueprint instances, designed to run alongside the Blueprint Manager to provide real-time monitoring and control of blueprint deployments.

## Core Requirements

### 1. Architecture
- **Type**: Local web application spawned by Blueprint Manager
- **Purpose**: Operator-focused dashboard for blueprint management
- **Deployment**: Runs on operator's machine, not cloud-based
- **Technology Stack**: 
  - React with TypeScript
  - Tangle Network UI components (@tangle-network/ui-components)
  - Webb Provider for wallet integration
  - Real-time WebSocket connections for live updates

### 2. Key Features

#### Dashboard
- System health overview with real-time metrics
- Active blueprint instances count and status
- Resource utilization (CPU, memory, disk, network)
- Quick access to common operations

#### Blueprint Management
- List all registered blueprints
- Deploy new blueprint instances
- Monitor blueprint health and performance
- View logs and debug information
- Start/stop/restart blueprint services

#### Container Integration
- Docker container monitoring
- Kubernetes cluster support
- Remote instance management
- Container resource allocation
- Log aggregation and viewing

#### System Monitoring
- Real-time CPU, memory, disk usage
- Network traffic monitoring
- Process management
- Alert configuration
- Performance history graphs

#### Wallet Integration
- Use @tangle-network/api-provider-environment
- Transaction management for blueprint operations
- Wallet connection status
- Balance and fee monitoring

### 3. Technical Implementation

#### Frontend
```typescript
// Core providers setup
import { WebbProvider } from '@tangle-network/api-provider-environment';
import { UIProvider } from '@tangle-network/ui-components';

// Wrap application with providers
<WebbProvider>
  <UIProvider>
    <BlueprintPortal />
  </UIProvider>
</WebbProvider>
```

#### Backend Services
- Docker API integration via Unix socket
- Kubernetes API client for cluster management
- System metrics collection (OS-level monitoring)
- WebSocket server for real-time updates
- RESTful API for blueprint operations

#### Data Flow
1. Blueprint Manager spawns portal server
2. Portal connects to local Docker daemon
3. WebSocket connection established for live updates
4. Metrics collected every 2-5 seconds
5. UI updates in real-time without polling

### 4. UI/UX Requirements

#### Design System
- Follow Tangle Network design patterns
- Dark theme by default for operator comfort
- Responsive layout for various screen sizes
- Accessibility compliant (WCAG 2.1 AA)

#### Key Pages
1. **Dashboard** - Overview and quick stats
2. **Instances** - Container and blueprint management
3. **Monitoring** - Detailed system metrics
4. **Blueprints** - Registry and deployment
5. **Settings** - Configuration and preferences
6. **Logs** - Centralized log viewer

### 5. Integration Points

#### Blueprint Manager
- Auto-start when manager launches
- Shared configuration files
- Direct API communication
- Process lifecycle management

#### Container Runtimes
- Docker Engine API
- Kubernetes API
- Podman support (future)
- Container runtime abstraction layer

#### Monitoring Stack
- Prometheus metrics export
- Grafana dashboard integration
- Custom alerting rules
- Log aggregation with Loki/Elasticsearch

### 6. Security Considerations

- Local-only by default (localhost binding)
- Optional authentication for remote access
- TLS/SSL for all communications
- Secure storage of credentials
- Role-based access control (future)

### 7. Development Approach

#### Phase 1: MVP
- Basic dashboard with Docker integration
- Blueprint listing and status
- Simple system metrics
- Local-only access

#### Phase 2: Enhanced Monitoring
- Kubernetes support
- Advanced metrics and graphs
- Log aggregation
- Alert configuration

#### Phase 3: Full Integration
- Wallet integration
- Transaction management
- Remote instance support
- Multi-operator collaboration

### 8. Testing Strategy

- Unit tests for all components
- Integration tests with mock Docker API
- E2E tests with real containers
- Performance testing with load simulation
- Security audit before production

### 9. Future Enhancements

- Mobile responsive design
- Progressive Web App (PWA) support
- Plugin system for custom extensions
- AI-powered anomaly detection
- Automated optimization suggestions
- Multi-language support

## Implementation Notes

1. **Start Simple**: Begin with Docker monitoring and basic metrics
2. **Use Existing Libraries**: Leverage Tangle UI components fully
3. **Real Data Only**: No mock data in production
4. **Performance First**: Optimize for low resource usage
5. **Developer Experience**: Clear documentation and examples

## References

- Tangle Network UI: https://github.com/webb-tools/tangle
- Docker Engine API: https://docs.docker.com/engine/api/
- Kubernetes API: https://kubernetes.io/docs/reference/
- Blueprint SDK: https://github.com/tangle-network/blueprint