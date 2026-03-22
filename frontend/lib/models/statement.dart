import 'ledger_entry.dart';

class Statement {
  final String  id;
  final String  accountId;
  final String  cycleStart;  // YYYY-MM-DD
  final String  cycleEnd;    // YYYY-MM-DD
  final String  statementBalance;
  final String? minimumPayment;
  final String  totalCharged;
  final String  totalPaid;
  final String  remaining;
  final bool    isSettled;
  final DateTime createdAt;

  const Statement({
    required this.id,
    required this.accountId,
    required this.cycleStart,
    required this.cycleEnd,
    required this.statementBalance,
    this.minimumPayment,
    required this.totalCharged,
    required this.totalPaid,
    required this.remaining,
    required this.isSettled,
    required this.createdAt,
  });

  factory Statement.fromJson(Map<String, dynamic> j) => Statement(
        id:               j['id']                as String,
        accountId:        j['account_id']        as String,
        cycleStart:       j['cycle_start']        as String,
        cycleEnd:         j['cycle_end']          as String,
        statementBalance: j['statement_balance']  as String,
        minimumPayment:   j['minimum_payment']    as String?,
        totalCharged:     j['total_charged']      as String,
        totalPaid:        j['total_paid']         as String,
        remaining:        j['remaining']          as String,
        isSettled:        j['is_settled']         as bool,
        createdAt:        DateTime.parse(j['created_at'] as String).toLocal(),
      );

  /// e.g. "Jan 16 – Feb 15"
  String get periodLabel {
    final start = DateTime.parse(cycleStart);
    final end   = DateTime.parse(cycleEnd);
    final months = ['Jan','Feb','Mar','Apr','May','Jun',
                    'Jul','Aug','Sep','Oct','Nov','Dec'];
    if (start.month == end.month && start.year == end.year) {
      return '${months[start.month-1]} ${start.day} – ${end.day}, ${end.year}';
    }
    return '${months[start.month-1]} ${start.day} – '
           '${months[end.month-1]} ${end.day}, ${end.year}';
  }
}

class StatementWithEntries {
  final Statement         statement;
  final List<LedgerEntry> entries;
  const StatementWithEntries(this.statement, this.entries);
}
