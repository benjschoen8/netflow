/// Mirrors the `AccountSummary` struct returned by `GET /accounts`.
class AccountSummary {
  final String accountId;
  final String accountName;
  final String accountType;
  final String currency;
  final String balance;
  final bool isDebt;
  final bool isOverdue;

  const AccountSummary({
    required this.accountId,
    required this.accountName,
    required this.accountType,
    required this.currency,
    required this.balance,
    required this.isDebt,
    required this.isOverdue,
  });

  factory AccountSummary.fromJson(Map<String, dynamic> json) => AccountSummary(
        accountId:   json['account_id']   as String,
        accountName: json['account_name'] as String,
        accountType: json['account_type'] as String,
        currency:    json['currency']     as String,
        balance:     json['balance']      as String,
        isDebt:      json['is_debt']      as bool,
        isOverdue:   json['is_overdue']   as bool,
      );

  /// Human-readable account type label.
  String get typeLabel => switch (accountType) {
        'cash'           => 'Bank Account',
        'physical_wallet'=> 'Cash Wallet',
        'digital_wallet' => 'Digital Wallet',
        'investment'     => 'Investment',
        'credit_card'    => 'Credit Card',
        'loan'           => 'Loan',
        _                => accountType,
      };
}
