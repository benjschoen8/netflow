# netflow — Flutter Frontend

## Setup

1. **Install Flutter** (if not already): https://docs.flutter.dev/get-started/install

2. **Create the Flutter project scaffold** (run once):
   ```bash
   cd netflow-main/frontend
   flutter create . --project-name netflow --org com.netflow
   ```
   This generates platform-specific files (android/, ios/, etc.).  
   It will NOT overwrite the `lib/` folder you already have.

3. **Install dependencies:**
   ```bash
   flutter pub get
   ```

4. **Configure the API URL** in `lib/core/config.dart`:
   ```dart
   // For Android emulator:
   static const String baseUrl = 'http://10.0.2.2:3000';

   // For iOS simulator or physical device on same LAN:
   static const String baseUrl = 'http://192.168.x.x:3000';
   ```

5. **Start the backend** (from `netflow-main/backend/`):
   ```bash
   cargo run
   ```

6. **Initialise the ledger** (one-time):
   ```bash
   curl -X POST http://localhost:3000/init
   ```

7. **Run the Flutter app:**
   ```bash
   flutter run
   ```

## Project Structure

```
lib/
├── main.dart                          # App entry point
├── core/
│   ├── config.dart                    # API base URL
│   ├── api/ledger_api.dart            # HTTP client
│   └── models/
│       ├── account_summary.dart       # Account list item model
│       └── net_worth_result.dart      # Net worth model
├── features/
│   └── dashboard/
│       ├── dashboard_screen.dart      # Main dashboard
│       ├── dashboard_provider.dart    # Riverpod providers
│       └── widgets/
│           ├── net_worth_card.dart    # Net worth hero card
│           └── account_tile.dart     # Account list row
└── shared/
    └── theme/app_theme.dart           # Colors, typography, theme
```

## Design

- **Dark theme** — deep navy `#080C18` background
- **DM Mono** for all numbers (precise, financial feel)
- **Plus Jakarta Sans** for UI labels
- **Teal `#00D4AA`** for assets / positive values
- **Red `#FF4757`** for debt / negative values  
- **Amber `#FFB347`** for overdue accounts
- Pull-to-refresh on the dashboard
