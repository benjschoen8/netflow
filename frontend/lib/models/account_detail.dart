/// Rich per-type account detail models.
/// Fetched on demand when opening an account's detail screen.
/// AccountSummary is still used for the dashboard list only.

// ── Cash ──────────────────────────────────────────────────────────────────────

class CashAccountDetail {
  final String accountId;
  final String accountName;
  final String currency;
  final String balance;
  final String bank;
  final String accountNumber;

  const CashAccountDetail({
    required this.accountId,
    required this.accountName,
    required this.currency,
    required this.balance,
    required this.bank,
    required this.accountNumber,
  });

  factory CashAccountDetail.fromJson(Map<String, dynamic> j) => CashAccountDetail(
        accountId:     j['account_id']     as String,
        accountName:   j['account_name']   as String,
        currency:      j['currency']       as String,
        balance:       j['balance']        as String,
        bank:          j['bank']           as String,
        accountNumber: j['account_number'] as String,
      );

  String get symbol => currency == 'TWD' ? 'NT\$' : '\$';
}

// ── Physical wallet ───────────────────────────────────────────────────────────

class PhysicalWalletDetail {
  final String accountId;
  final String accountName;
  final String currency;
  final String balance;

  const PhysicalWalletDetail({
    required this.accountId,
    required this.accountName,
    required this.currency,
    required this.balance,
  });

  factory PhysicalWalletDetail.fromJson(Map<String, dynamic> j) => PhysicalWalletDetail(
        accountId:   j['account_id']   as String,
        accountName: j['account_name'] as String,
        currency:    j['currency']     as String,
        balance:     j['balance']      as String,
      );

  String get symbol => currency == 'TWD' ? 'NT\$' : '\$';
}

// ── Digital wallet ────────────────────────────────────────────────────────────

class DigitalWalletDetail {
  final String accountId;
  final String accountName;
  final String currency;
  final String balance;
  final String provider;
  final String providerAccountId;

  const DigitalWalletDetail({
    required this.accountId,
    required this.accountName,
    required this.currency,
    required this.balance,
    required this.provider,
    required this.providerAccountId,
  });

  factory DigitalWalletDetail.fromJson(Map<String, dynamic> j) => DigitalWalletDetail(
        accountId:          j['account_id']           as String,
        accountName:        j['account_name']         as String,
        currency:           j['currency']             as String,
        balance:            j['balance']              as String,
        provider:           j['provider']             as String,
        providerAccountId:  j['provider_account_id']  as String,
      );

  String get symbol => currency == 'TWD' ? 'NT\$' : '\$';
}

// ── Investment ────────────────────────────────────────────────────────────────

class InvestmentAccountDetail {
  final String accountId;
  final String accountName;
  final String currency;
  final String cashBalance;
  final String holdingsValue;
  final String totalValue;
  final String bank;
  final String accountNumber;
  final List<HoldingDetail> holdings;

  const InvestmentAccountDetail({
    required this.accountId,
    required this.accountName,
    required this.currency,
    required this.cashBalance,
    required this.holdingsValue,
    required this.totalValue,
    required this.bank,
    required this.accountNumber,
    required this.holdings,
  });

  factory InvestmentAccountDetail.fromJson(Map<String, dynamic> j) =>
      InvestmentAccountDetail(
        accountId:     j['account_id']     as String,
        accountName:   j['account_name']   as String,
        currency:      j['currency']       as String,
        cashBalance:   j['cash_balance']   as String,
        holdingsValue: j['holdings_value'] as String,
        totalValue:    j['total_value']    as String,
        bank:          j['bank']           as String,
        accountNumber: j['account_number'] as String,
        holdings: (j['holdings'] as List<dynamic>)
            .cast<Map<String, dynamic>>()
            .map(HoldingDetail.fromJson)
            .toList(),
      );

  String get symbol => currency == 'TWD' ? 'NT\$' : '\$';
}

class HoldingDetail {
  final String ticker;
  final String investmentType;
  final String quantity;
  final String unitPrice;
  final String marketValue;
  final String currency;

  const HoldingDetail({
    required this.ticker,
    required this.investmentType,
    required this.quantity,
    required this.unitPrice,
    required this.marketValue,
    required this.currency,
  });

  factory HoldingDetail.fromJson(Map<String, dynamic> j) => HoldingDetail(
        ticker:         j['ticker']          as String,
        investmentType: j['investment_type'] as String,
        quantity:       j['quantity']        as String,
        unitPrice:      j['unit_price']      as String,
        marketValue:    j['market_value']    as String,
        currency:       j['currency']        as String,
      );

  String get symbol => currency == 'TWD' ? 'NT\$' : '\$';
}

// ── Loan ──────────────────────────────────────────────────────────────────────

class LoanAccountDetail {
  final String  accountId;
  final String  accountName;
  final String  currency;
  final String  bank;
  final String? accountNumber;
  final String  creditor;
  final String  principal;
  final String  outstanding;
  final String  amountPaid;
  final String  percentPaid;
  final double? interestRate;
  final int?    dueDay;
  final String? maturityDate;
  final String? minimumPayment;
  final bool    isOverdue;
  final bool    isSettled;

  const LoanAccountDetail({
    required this.accountId,
    required this.accountName,
    required this.currency,
    required this.bank,
    this.accountNumber,
    required this.creditor,
    required this.principal,
    required this.outstanding,
    required this.amountPaid,
    required this.percentPaid,
    this.interestRate,
    this.dueDay,
    this.maturityDate,
    this.minimumPayment,
    required this.isOverdue,
    required this.isSettled,
  });

  factory LoanAccountDetail.fromJson(Map<String, dynamic> j) => LoanAccountDetail(
        accountId:     j['account_id']     as String,
        accountName:   j['account_name']   as String,
        currency:      j['currency']       as String,
        bank:          j['bank']           as String,
        accountNumber: j['account_number'] as String?,
        creditor:      j['creditor']       as String,
        principal:     j['principal']      as String,
        outstanding:   j['outstanding']    as String,
        amountPaid:    j['amount_paid']    as String,
        percentPaid:   j['percent_paid']   as String,
        interestRate:  (j['interest_rate'] as num?)?.toDouble(),
        dueDay:        j['due_day']        as int?,
        maturityDate:  j['maturity_date']  as String?,
        minimumPayment:j['minimum_payment']as String?,
        isOverdue:     j['is_overdue']     as bool,
        isSettled:     j['is_settled']     as bool,
      );

  String get symbol => currency == 'TWD' ? 'NT\$' : '\$';

  double get percentPaidDouble => double.tryParse(percentPaid) ?? 0;
}
