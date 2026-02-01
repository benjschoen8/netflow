# netflow
# Master Roadmap: Multi-Asset Tracking System (Rust Edition)

**Priority Protocol:** Backend First. Frontend Second.
**Architecture:** Rust Backend (Axum + SeaORM) + React Frontend (SPA).

---

## **Phase 1: The Core Foundation (Identity & Spending)**
**Goal:** Authenticate the user and build the "Headless" Ledger API.

### **1.1 Identity & Access Management (IAM)**
**Backend Engineering (Rust) - Priority 1**
* [ ] **Crate Setup:** Initialize cargo workspace (`backend`, `backend/core`, `backend/entity`).
* [ ] **Database (Migration):** Create `users` table (`username`, `password_hash` as `String`, `email`).
* [ ] **Database (Migration):** Create `user_ui_preferences` table.
    * *Columns:* `user_id` (UUID), `dashboard_layout` (JSONB).
* [ ] **API (Axum):** Implement `POST /auth/register`.
    * *Logic:* Hash password using `argon2`. Transactional insert into `users` & `user_ui_preferences`.
* [ ] **API (Axum):** Implement `POST /auth/token`.
    * *Logic:* Verify hash, issue `jsonwebtoken` (JWT).
* [ ] **API:** Implement `PATCH /users/me/preferences` using `serde_json`.

**Frontend Engineering (React) - Priority 2**
* [ ] **State (Zustand):** `useAuthStore` (Holds JWT & `isAuthenticated`).
* [ ] **UI:** Build **Login Screen** and **Settings Page** (Currency Selector, Theme Toggle).

### **1.2 The Classification Engine**
**Backend Engineering (Rust) - Priority 1**
* [ ] **Database:** Create `expense_categories` & `labels` tables.
* [ ] **Entity:** Generate SeaORM entities for `Category` and `Label`.
* [ ] **API:** Implement `GET /categories`.
    * *Logic:* Recursive CTE or application-side tree building for category hierarchy.
* [ ] **API:** Implement `POST /categories` and `GET /labels`.

**Frontend Engineering (React) - Priority 2**
* [ ] **State:** `useCategories()` hook with caching.
* [ ] **UI:** Build **Category Manager** and **Label Manager**.

### **1.3 The Ledger V1 (Spending Log)**
**Backend Engineering (Rust) - Priority 1**
* [ ] **Database:** Create `transactions` table (`amount` as `Decimal`, `currency`, `timestamp`).
    * *Note:* Use `rust_decimal` crate for financial precision. Never use floats.
* [ ] **Database:** Create `transaction_labels` link table.
* [ ] **API:** Implement `POST /transactions`.
    * *Logic:* Validation struct with `validator` crate. Ensure User owns Category.
* [ ] **API:** Implement `GET /reports/spending`.
    * *Logic:* Aggregation query using SeaORM `group_by`.

**Frontend Engineering (React) - Priority 2**
* [ ] **Form (Hook Form):** Build **"Quick Add" Widget** (Amount, Currency, Category, Labels).
* [ ] **UI:** Build **Transaction List** and **Spending Pie Chart**.

---

## **Phase 2: The Liability Ledger (Debt Engine)**
**Goal:** Manage Credit Cards, Mortgages, and "Ghost Debt".

### **2.1 The Containers (Liabilities)**
**Backend Engineering (Rust) - Priority 1**
* [ ] **Database:** Create `liability_accounts` table.
    * *Enum:* `AccountType` (CreditCard, Loan).
* [ ] **API:** Implement `GET /accounts/liabilities`.
* [ ] **Service (Rust):** Implement `UtilizationCalculator`.
    * *Logic:* Struct impl that computes `(locked / limit)` safely.

**Frontend Engineering (React) - Priority 2**
* [ ] **State:** `useLiabilities()` hook.
* [ ] **UI:** Build **Credit Card Component** (Utilization progress bar) and **Loan Detail View**.

### **2.2 The Logic (Debt Intelligence)**
**Backend Engineering (Rust) - Priority 1**
* [ ] **Logic:** Implement **Cash Flow Projector** (Module).
    * *Function:* `fn calculate_cash_exit(tx_date: NaiveDate, closing_day: u8) -> NaiveDate`
* [ ] **Logic:** Implement **Regional Installment Strategy** (Traits).
    * *Trait:* `InstallmentStrategy` with impls for `TaiwanStrategy` and `USStrategy`.
* [ ] **API:** Implement `GET /reports/upcoming-bills`.

**Frontend Engineering (React) - Priority 2**
* [ ] **UI:** Build **"Cash Flow Forecast" Widget** and **Installment Breakdown**.

### **2.3 The Ledger V2 (Debt Spending)**
**Backend Engineering (Rust) - Priority 1**
* [ ] **Update:** Modify `transactions` entity to `Option<Uuid>` for `source_liability_id`.
* [ ] **Logic:** Update `POST` handler to match `source_liability_id` logic.

**Frontend Engineering (React) - Priority 2**
* [ ] **Update:** Specific **"Source" Dropdown** in Quick Add Widget (Credit Cards).

---

## **Phase 3: The Account Ledger (Liquidity & Settlement)**
**Goal:** Manage Bank Accounts and Pay off Phase 2 Debts.

### **3.1 The Containers (Liquid Assets)**
**Backend Engineering (Rust) - Priority 1**
* [ ] **Database:** Create `liquid_accounts` table.
* [ ] **API:** Implement `GET /accounts/liquid`.
* [ ] **Logic:** `fn calculate_balance(account_id: Uuid) -> Decimal`.

**Frontend Engineering (React) - Priority 2**
* [ ] **UI:** Build **Bank Dashboard** and **Net Liquid Assets** widget.

### **3.2 The Ledger V3 (Settlement)**
**Backend Engineering (Rust) - Priority 1**
* [ ] **Update:** Update `TransactionType` enum (`Income`, `Transfer`, `LiabilityPayment`).
* [ ] **API:** Implement `POST /transactions/transfer` (Atomic transaction using `txn.begin()`).
* [ ] **API:** Implement `POST /transactions/pay-bill`.

**Frontend Engineering (React) - Priority 2**
* [ ] **UI:** Build **Transfer Wizard** and **"Pay Bill" Button**.

---

## **Phase 4: The Investment Ledger (Wealth Engine)**
**Goal:** Track Stocks, Real Estate, and Net Worth Growth.

### **4.1 The Containers (Assets)**
**Backend Engineering (Rust) - Priority 1**
* [ ] **Database:** Create `investment_assets` & `security_positions`.
* [ ] **API:** Implement `GET /accounts/investments`.
* [ ] **Service:** `PriceFetcher` trait (Async HTTP client via `reqwest`).

**Frontend Engineering (React) - Priority 2**
* [ ] **UI:** Build **Portfolio Table** and **Real Estate Card**.

### **4.2 The Ledger V4 (Trading)**
**Backend Engineering (Rust) - Priority 1**
* [ ] **API:** Implement `POST /transactions/trade`.
    * *Input:* `TradeRequest { symbol, qty, price, side }`.
* [ ] **Logic:** Atomic database transaction updating Cash Core and Position.

**Frontend Engineering (React) - Priority 2**
* [ ] **Form:** Build **Trade Entry Form**.

---

## **Phase 5: The Grand Aggregation (Reporting)**
**Goal:** The Master Dashboard that pulls it all together.

### **5.1 The Portfolio Service**
**Backend Engineering (Rust) - Priority 1**
* [ ] **API:** Implement `GET /reports/net-worth`.
    * *Logic:* Parallel fetching of assets/liabilities using `tokio::join!`.
    * *Logic:* Currency conversion using `rust_decimal` arithmetic.

**Frontend Engineering (React) - Priority 2**
* [ ] **State:** `useNetWorth()` hook.
* [ ] **UI:** Build **Net Worth Hero Component** and **Currency Toggle**.

### **5.2 The User-Configurable Dashboard**
**Backend Engineering (Rust) - Priority 1**
* [ ] **API:** Connect `GET /user/preferences` (Returns `Json<UserPreferences>`).
* [ ] **API:** Implement `PATCH /user/preferences`.

**Frontend Engineering (React) - Priority 2**
* [ ] **Logic:** **Dynamic Layout Engine** (Map JSON to Components).
* [ ] **UI:** **Edit Mode** (Drag-and-Drop).
