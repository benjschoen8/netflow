class LedgerEntry {
  final String id;
  final String accountId;
  final String entryType;
  final String amount;
  final String currency;
  final DateTime occurredAt;
  final String? label;
  final String? description;

  const LedgerEntry({
    required this.id,
    required this.accountId,
    required this.entryType,
    required this.amount,
    required this.currency,
    required this.occurredAt,
    this.label,
    this.description,
  });

  factory LedgerEntry.fromJson(Map<String, dynamic> json) => LedgerEntry(
        id:          json['id'] as String,
        accountId:   json['account_id'] as String,
        entryType:   json['entry_type'] as String,
        amount:      json['amount'] as String,
        currency:    json['currency'] as String,
        occurredAt:  DateTime.parse(json['occurred_at'] as String).toLocal(),
        label:       json['label'] as String?,
        description: json['description'] as String?,
      );

  /// Human-readable label for the entry type.
  String get typeDisplayName => switch (entryType) {
        'deposit'          => 'Deposit',
        'withdrawal'       => 'Withdrawal',
        'charge'           => 'Purchase',
        'payment_made'     => 'Payment',
        'payment_received' => 'Payment Received',
        'interest_accrued' => 'Interest',
        'statement_closed' => 'Statement Closed',
        _                  => entryType,
      };

  /// Whether the entry increases the balance (asset) or decreases it.
  bool get isCredit => switch (entryType) {
        'deposit' || 'payment_received' => true,
        _                               => false,
      };

  LedgerEntry copyWith({String? label, String? description}) => LedgerEntry(
        id:          id,
        accountId:   accountId,
        entryType:   entryType,
        amount:      amount,
        currency:    currency,
        occurredAt:  occurredAt,
        label:       label ?? this.label,
        description: description ?? this.description,
      );
}
