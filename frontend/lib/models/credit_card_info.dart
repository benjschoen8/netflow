class CreditCardInfo {
  final String  accountId;
  final String  accountName;
  final String  currency;
  final String  network;
  final String  lastFour;
  final String  expiry;
  final String  creditLimit;
  final String  availableCredit;
  final String  outstanding;
  final int     statementDay;
  final int     dueDay;
  final double? interestRate;
  final String? minimumPayment;
  final bool    minimumPaymentPaid;
  final bool    isOverdue;
  final bool    isPaid;
  final String? statementBalance;
  final TempLimitInfo? tempLimit;

  const CreditCardInfo({
    required this.accountId,
    required this.accountName,
    required this.currency,
    required this.network,
    required this.lastFour,
    required this.expiry,
    required this.creditLimit,
    required this.availableCredit,
    required this.outstanding,
    required this.statementDay,
    required this.dueDay,
    this.interestRate,
    this.minimumPayment,
    required this.minimumPaymentPaid,
    required this.isOverdue,
    required this.isPaid,
    this.statementBalance,
    this.tempLimit,
  });

  factory CreditCardInfo.fromJson(Map<String, dynamic> j) => CreditCardInfo(
        accountId:           j['account_id']           as String,
        accountName:         j['account_name']         as String,
        currency:            j['currency']             as String,
        network:             j['network']              as String,
        lastFour:            j['last_four']            as String,
        expiry:              j['expiry']               as String,
        creditLimit:         j['credit_limit']         as String,
        availableCredit:     j['available_credit']     as String,
        outstanding:         j['outstanding']          as String,
        statementDay:        j['statement_day']        as int,
        dueDay:              j['due_day']              as int,
        interestRate:        (j['interest_rate'] as num?)?.toDouble(),
        minimumPayment:      j['minimum_payment']      as String?,
        minimumPaymentPaid:  j['minimum_payment_paid'] as bool,
        isOverdue:           j['is_overdue']           as bool,
        isPaid:              j['is_paid']              as bool,
        statementBalance:    j['statement_balance']    as String?,
        tempLimit: j['temp_limit'] == null
            ? null
            : TempLimitInfo.fromJson(j['temp_limit'] as Map<String, dynamic>),
      );

  /// Symbol for this currency.
  String get symbol => currency == 'TWD' ? 'NT\$' : '\$';

  /// The date of the most recent (or next upcoming) statement close.
  /// Returns the latest past statement date relative to [now].
  DateTime lastStatementDate([DateTime? now]) {
    final today = now ?? DateTime.now();
    // This month's statement date
    final thisMonth = DateTime(today.year, today.month, statementDay);
    // If today is on or after this month's statement day, last statement was this month
    return today.day >= statementDay
        ? thisMonth
        : DateTime(today.year, today.month - 1, statementDay);
  }

  /// Start of the current billing cycle = day after last statement.
  DateTime currentCycleStart([DateTime? now]) {
    final stmt = lastStatementDate(now);
    return stmt.add(const Duration(days: 1));
  }

  /// Next payment due date after [lastStatementDate].
  DateTime nextDueDate([DateTime? now]) {
    final stmt = lastStatementDate(now);
    // Due day is typically in the following month
    return DateTime(stmt.year, stmt.month + 1, dueDay);
  }
}

class TempLimitInfo {
  final String amount;
  final String expiresOn;
  const TempLimitInfo({required this.amount, required this.expiresOn});
  factory TempLimitInfo.fromJson(Map<String, dynamic> j) => TempLimitInfo(
        amount:    j['amount']     as String,
        expiresOn: j['expires_on'] as String,
      );
}
