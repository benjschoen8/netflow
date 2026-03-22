import '../../models/account_summary.dart';
import '../../models/net_worth_result.dart';
import '../../models/ledger_entry.dart';
import '../../models/credit_card_info.dart';
import '../../models/account_detail.dart';
import '../../models/statement.dart';
import 'api_client.dart';

class LedgerRepository {
  LedgerRepository(this._api);
  final ApiClient _api;

  // ── Lifecycle ──────────────────────────────────────────────────────────────

  Future<void> init() => _api.post('/init');

  // ── Queries ────────────────────────────────────────────────────────────────

  Future<List<AccountSummary>> listAccounts() async {
    final data = await _api.get('/accounts') as List<dynamic>;
    return data.cast<Map<String, dynamic>>().map(AccountSummary.fromJson).toList();
  }

  Future<List<NetWorthResult>> getNetWorth({String? currency}) async {
    final data = await _api.get(
      '/net-worth',
      query: currency != null ? {'currency': currency} : null,
    ) as List<dynamic>;
    return data.cast<Map<String, dynamic>>().map(NetWorthResult.fromJson).toList();
  }

  // ── Account management ─────────────────────────────────────────────────────

  Future<void> openCashAccount({
    required String name,
    required String accountNumber,
    required String bank,
    required String currency,
    required String initialBalance,
  }) =>
      _api.post('/accounts/cash', {
        'name': name,
        'account_number': accountNumber,
        'bank': bank,
        'currency': currency,
        'initial_balance': initialBalance,
      });

  Future<void> addPhysicalWallet({
    required String name,
    required String currency,
    required String initialBalance,
  }) =>
      _api.post('/accounts/wallet', {
        'name': name,
        'currency': currency,
        'initial_balance': initialBalance,
      });

  Future<void> addDigitalWallet({
    required String name,
    required String provider,
    required String providerAccountId,
    required String currency,
    required String initialBalance,
  }) =>
      _api.post('/accounts/digital-wallet', {
        'name': name,
        'provider': provider,
        'provider_account_id': providerAccountId,
        'currency': currency,
        'initial_balance': initialBalance,
      });

  Future<void> openInvestmentAccount({
    required String name,
    required String accountNumber,
    required String bank,
    required String currency,
    required String cashBalance,
  }) =>
      _api.post('/accounts/investment', {
        'name': name,
        'account_number': accountNumber,
        'bank': bank,
        'currency': currency,
        'cash_balance': cashBalance,
      });

  Future<void> addCreditCard({
    required String name,
    required String lastFour,
    required String network,
    required int expiryMonth,
    required int expiryYear,
    required String creditLimit,
    required String currency,
    required String outstanding,
    required int statementDay,
    required int dueDay,
    String? interestRate,
  }) =>
      _api.post('/accounts/credit-card', {
        'name': name,
        'last_four': lastFour,
        'network': network,
        'expiry_month': expiryMonth,
        'expiry_year': expiryYear,
        'credit_limit': creditLimit,
        'currency': currency,
        'outstanding': outstanding,
        'statement_day': statementDay,
        'due_day': dueDay,
        if (interestRate != null) 'interest_rate': interestRate,
      });

  Future<void> openLoan({
    required String name,
    required String bank,
    required String creditor,
    required String currency,
    required String principal,
    String? accountNumber,
    String? interestRate,
    int? dueDay,
    String? maturityDate,
    String? minimumPayment,
  }) =>
      _api.post('/accounts/loan', {
        'name': name,
        'bank': bank,
        'creditor': creditor,
        'currency': currency,
        'principal': principal,
        if (accountNumber != null) 'account_number': accountNumber,
        if (interestRate != null) 'interest_rate': interestRate,
        if (dueDay != null) 'due_day': dueDay,
        if (maturityDate != null) 'maturity_date': maturityDate,
        if (minimumPayment != null) 'minimum_payment': minimumPayment,
      });

  Future<void> removeAccount(String accountId) =>
      _api.delete('/accounts/$accountId');

  Future<void> updateAccountInfo(
    String accountId, {
    String? name,
    String? bank,
    String? accountNumber,
  }) =>
      _api.patch('/accounts/$accountId', {
        if (name != null) 'name': name,
        if (bank != null) 'bank': bank,
        if (accountNumber != null) 'account_number': accountNumber,
      });

  Future<void> transferFunds({
    required String fromAccountId,
    required String toAccountId,
    required String amount,
    required String currency,
    String? label,
    String? description,
  }) =>
      _api.post('/accounts/$fromAccountId/transfer', {
        'to_account_id': toAccountId,
        'amount':        amount,
        'currency':      currency,
        if (label       != null) 'label':       label,
        if (description != null) 'description': description,
      });

  // ── Transactions ───────────────────────────────────────────────────────────

  Future<void> deposit(String accountId, String amount, String currency) =>
      _api.post('/accounts/$accountId/deposit', {
        'amount': amount,
        'currency': currency,
      });

  Future<void> withdraw(String accountId, String amount, String currency) =>
      _api.post('/accounts/$accountId/withdraw', {
        'amount': amount,
        'currency': currency,
      });

  Future<void> charge(String accountId, String amount, String currency) =>
      _api.post('/accounts/$accountId/charge', {
        'amount': amount,
        'currency': currency,
      });

  Future<void> pay({
    required String debtAccountId,
    required String fromAccountId,
    required String amount,
    required String currency,
  }) =>
      _api.post('/accounts/$debtAccountId/pay', {
        'from_account_id': fromAccountId,
        'amount': amount,
        'currency': currency,
      });

  Future<void> closeStatement({
    required String accountId,
    String? minimumPayment,
    required String currency,
  }) =>
      _api.post('/accounts/$accountId/statement', {
        'currency': currency,
        if (minimumPayment != null && minimumPayment.isNotEmpty)
          'minimum_payment': minimumPayment,
      });

  Future<void> grantTemporaryLimit({
    required String accountId,
    required String newLimit,
    required String currency,
    required String expiresOn,
  }) =>
      _api.post('/accounts/$accountId/grant-limit', {
        'new_limit': newLimit,
        'currency': currency,
        'expires_on': expiresOn,
      });

  Future<void> revokeTemporaryLimit(String accountId) =>
      _api.post('/accounts/$accountId/revoke-limit');

  Future<void> accrueInterest(String accountId) =>
      _api.post('/accounts/$accountId/interest');

  // ── Holdings ───────────────────────────────────────────────────────────────

  Future<void> addHolding({
    required String accountId,
    required String ticker,
    required String investmentType,
    required String quantity,
    required String unitPrice,
    required String currency,
  }) =>
      _api.post('/accounts/$accountId/holdings', {
        'ticker': ticker,
        'investment_type': investmentType,
        'quantity': quantity,
        'unit_price': unitPrice,
        'currency': currency,
      });

  Future<void> removeHolding(String accountId, String ticker) =>
      _api.delete('/accounts/$accountId/holdings/$ticker');

  Future<void> updateHoldingPrice({
    required String accountId,
    required String ticker,
    required String newPrice,
    required String currency,
  }) =>
      _api.patch('/accounts/$accountId/holdings/$ticker/price', {
        'new_price': newPrice,
        'currency': currency,
      });

  // ── Annotated transaction helpers ─────────────────────────────────────────

  Future<void> depositWithAnnotation(
    String accountId, String amount, String currency, {
    String? label, String? description,
  }) => _api.post('/accounts/$accountId/deposit', {
        'amount': amount, 'currency': currency,
        if (label != null) 'label': label,
        if (description != null) 'description': description,
      });

  Future<void> withdrawWithAnnotation(
    String accountId, String amount, String currency, {
    String? label, String? description,
  }) => _api.post('/accounts/$accountId/withdraw', {
        'amount': amount, 'currency': currency,
        if (label != null) 'label': label,
        if (description != null) 'description': description,
      });

  Future<void> chargeWithAnnotation(
    String accountId, String amount, String currency, {
    String? label, String? description,
  }) => _api.post('/accounts/$accountId/charge', {
        'amount': amount, 'currency': currency,
        if (label != null) 'label': label,
        if (description != null) 'description': description,
      });

  Future<void> payWithAnnotation({
    required String debtAccountId,
    required String fromAccountId,
    required String amount,
    required String currency,
    String? label,
    String? description,
  }) => _api.post('/accounts/$debtAccountId/pay', {
        'from_account_id': fromAccountId,
        'amount': amount, 'currency': currency,
        if (label != null) 'label': label,
        if (description != null) 'description': description,
      });

  // ── Rich per-type detail ───────────────────────────────────────────────────

  Future<CashAccountDetail> getCashDetail(String id) async =>
      CashAccountDetail.fromJson(
          await _api.get('/accounts/$id/detail/cash') as Map<String, dynamic>);

  Future<PhysicalWalletDetail> getWalletDetail(String id) async =>
      PhysicalWalletDetail.fromJson(
          await _api.get('/accounts/$id/detail/wallet') as Map<String, dynamic>);

  Future<DigitalWalletDetail> getDigitalWalletDetail(String id) async =>
      DigitalWalletDetail.fromJson(
          await _api.get('/accounts/$id/detail/digital-wallet') as Map<String, dynamic>);

  Future<InvestmentAccountDetail> getInvestmentDetail(String id) async =>
      InvestmentAccountDetail.fromJson(
          await _api.get('/accounts/$id/detail/investment') as Map<String, dynamic>);

  Future<LoanAccountDetail> getLoanDetail(String id) async =>
      LoanAccountDetail.fromJson(
          await _api.get('/accounts/$id/detail/loan') as Map<String, dynamic>);

  // ── Statements ─────────────────────────────────────────────────────────────

  Future<List<Statement>> listStatements(String accountId) async {
    final data = await _api.get('/accounts/$accountId/statements') as List<dynamic>;
    return data.cast<Map<String, dynamic>>().map(Statement.fromJson).toList();
  }

  Future<List<LedgerEntry>> statementEntries(
      String accountId, String statementId) async {
    final data = await _api.get(
        '/accounts/$accountId/statements/$statementId/entries') as List<dynamic>;
    return data
        .cast<Map<String, dynamic>>()
        .map(LedgerEntry.fromJson)
        .toList();
  }

  // ── Credit card detail ───────────────────────────────────────────────────

  Future<CreditCardInfo> getCreditCardInfo(String accountId) async {
    final data = await _api.get('/accounts/$accountId/detail/credit-card');
    return CreditCardInfo.fromJson(data as Map<String, dynamic>);
  }

  // ── Ledger entries ─────────────────────────────────────────────────────────

  Future<List<LedgerEntry>> listEntries(String accountId) async {
    final data = await _api.get('/accounts/$accountId/entries') as List<dynamic>;
    return data.cast<Map<String, dynamic>>().map(LedgerEntry.fromJson).toList();
  }

  Future<void> updateAnnotation(
    String entryId, {
    String? label,
    String? description,
  }) =>
      _api.patch('/entries/$entryId', {
        'label': label,
        'description': description,
      });
}
