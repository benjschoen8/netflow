import '../../models/account_summary.dart';
import '../../models/net_worth_result.dart';
import 'api_client.dart';

/// All calls to the Rust API live here.
/// Screens and providers never touch [ApiClient] directly.
class LedgerRepository {
  LedgerRepository(this._api);
  final ApiClient _api;

  // ── Lifecycle ─────────────────────────────────────────────────────────────

  Future<void> init() => _api.post('/init');

  // ── Dashboard ─────────────────────────────────────────────────────────────

  Future<List<AccountSummary>> listAccounts() async {
    final data = await _api.get('/accounts') as List<dynamic>;
    return data
        .cast<Map<String, dynamic>>()
        .map(AccountSummary.fromJson)
        .toList();
  }

  Future<List<NetWorthResult>> getNetWorth({String? currency}) async {
    final data = await _api.get(
      '/net-worth',
      query: currency != null ? {'currency': currency} : null,
    ) as List<dynamic>;
    return data
        .cast<Map<String, dynamic>>()
        .map(NetWorthResult.fromJson)
        .toList();
  }

  // ── Account management ────────────────────────────────────────────────────

  Future<void> openCashAccount({
    required String name,
    required String accountNumber,
    required String bank,
    required String currency,
    required String initialBalance,
  }) => _api.post('/accounts/cash', {
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
  }) => _api.post('/accounts/wallet', {
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
  }) => _api.post('/accounts/digital-wallet', {
        'name': name,
        'provider': provider,
        'provider_account_id': providerAccountId,
        'currency': currency,
        'initial_balance': initialBalance,
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
  }) => _api.post('/accounts/credit-card', {
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
  }) => _api.post('/accounts/loan', {
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

  // ── Transactions ──────────────────────────────────────────────────────────

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
  }) => _api.post('/accounts/$debtAccountId/pay', {
        'from_account_id': fromAccountId,
        'amount': amount,
        'currency': currency,
      });
}
