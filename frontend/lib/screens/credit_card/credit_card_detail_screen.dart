import '../../theme/app_theme.dart';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:intl/intl.dart';
import '../../core/providers.dart';
import '../../models/account_summary.dart';
import '../../models/credit_card_info.dart';
import '../../models/ledger_entry.dart';
import '../../models/statement.dart';
import '../../shared/widgets/form_sheet.dart';
import '../transactions/transaction_sheets.dart';

// ── Providers ─────────────────────────────────────────────────────────────────

// Statement entries keyed by (accountId, statementId)
typedef _StmtKey = (String, String);
final _stmtEntriesProvider =
    FutureProvider.family<List<LedgerEntry>, _StmtKey>((ref, key) {
  return ref.watch(ledgerRepoProvider).statementEntries(key.$1, key.$2);
});

// ── Screen ────────────────────────────────────────────────────────────────────

class CreditCardDetailScreen extends ConsumerStatefulWidget {
  const CreditCardDetailScreen({
    super.key,
    required this.account,
    required this.allAccounts,
  });
  final AccountSummary       account;
  final List<AccountSummary> allAccounts;

  @override
  ConsumerState<CreditCardDetailScreen> createState() =>
      _CreditCardDetailScreenState();
}

class _CreditCardDetailScreenState
    extends ConsumerState<CreditCardDetailScreen>
    with SingleTickerProviderStateMixin {
  late TabController _tabs;

  @override
  void initState() {
    super.initState();
    _tabs = TabController(length: 3, vsync: this);
  }

  @override
  void dispose() { _tabs.dispose(); super.dispose(); }

  void _refresh() {
    refreshDashboard(ref);
  }

  @override
  Widget build(BuildContext context) {
    final cardAsync = ref.watch(cardInfoProvider(widget.account.accountId));

    return Scaffold(
      appBar: AppBar(
        title: Text(widget.account.accountName),
        actions: [
          IconButton(
            icon: const Icon(Icons.refresh_rounded),
            onPressed: _refresh,
          ),
        ],
        bottom: TabBar(
          controller: _tabs,
          tabs: const [
            Tab(icon: Icon(Icons.dashboard_outlined),       text: 'Overview'),
            Tab(icon: Icon(Icons.receipt_long_outlined),    text: 'Bills'),
            Tab(icon: Icon(Icons.list_alt_rounded),         text: 'All Entries'),
          ],
        ),
      ),
      body: cardAsync.when(
        loading: () => const Center(child: CircularProgressIndicator()),
        error:   (e, _) => Center(child: Text(e.toString())),
        data:    (card) => TabBarView(
          controller: _tabs,
          children: [
            // ── Tab 0: Overview ──────────────────────────────────────────
            _OverviewTab(
              card: card,
              account: widget.account,
              allAccounts: widget.allAccounts,
              onAction: _refresh,
              ref: ref,
            ),

            // ── Tab 1: Bills (statements) ────────────────────────────────
            _BillsTab(
              accountId: widget.account.accountId,
              card: card,
              ref: ref,
            ),

            // ── Tab 2: All entries ───────────────────────────────────────
            _AllEntriesTab(
              accountId: widget.account.accountId,
              card: card,
              ref: ref,
            ),
          ],
        ),
      ),
    );
  }
}

// ── Overview tab ──────────────────────────────────────────────────────────────

class _OverviewTab extends StatelessWidget {
  const _OverviewTab({
    required this.card,
    required this.account,
    required this.allAccounts,
    required this.onAction,
    required this.ref,
  });
  final CreditCardInfo       card;
  final AccountSummary       account;
  final List<AccountSummary> allAccounts;
  final VoidCallback         onAction;
  final WidgetRef            ref;

  @override
  Widget build(BuildContext context) {
    return ListView(
      padding: const EdgeInsets.only(bottom: 40),
      children: [
        _CardHero(card: card),
        _KeyStats(card: card),
        const SizedBox(height: 8),
        Padding(
          padding: const EdgeInsets.symmetric(horizontal: 16),
          child: _ActionRow(
            card: card, account: account,
            allAccounts: allAccounts, ref: ref, onAction: onAction,
          ),
        ),
        const SizedBox(height: 16),
      ],
    );
  }
}

// ── Bills tab (real Statement objects) ───────────────────────────────────────

class _BillsTab extends StatelessWidget {
  const _BillsTab({
    required this.accountId,
    required this.card,
    required this.ref,
  });
  final String         accountId;
  final CreditCardInfo card;
  final WidgetRef      ref;

  @override
  Widget build(BuildContext context) {
    final stmtsAsync = ref.watch(statementsProvider(accountId));

    return stmtsAsync.when(
      loading: () => const Center(child: CircularProgressIndicator()),
      error:   (e, _) => Center(child: Text(e.toString())),
      data:    (stmts) => stmts.isEmpty
          ? _NoBillsView()
          : ListView.builder(
              padding: const EdgeInsets.only(bottom: 40),
              itemCount: stmts.length,
              itemBuilder: (context, i) => _BillCard(
                statement: stmts[i],
                card: card,
                accountId: accountId,
                ref: ref,
              ),
            ),
    );
  }
}

class _BillCard extends StatefulWidget {
  const _BillCard({
    required this.statement,
    required this.card,
    required this.accountId,
    required this.ref,
  });
  final Statement      statement;
  final CreditCardInfo card;
  final String         accountId;
  final WidgetRef      ref;

  @override
  State<_BillCard> createState() => _BillCardState();
}

class _BillCardState extends State<_BillCard> {
  bool _expanded = false;

  @override
  Widget build(BuildContext context) {
    final cs  = Theme.of(context).colorScheme;
    final tt  = Theme.of(context).textTheme;
    final s   = widget.statement;
    final sym = widget.card.symbol;

    final statusColor  = s.isSettled ? AppTheme.green : cs.error;
    final statusLabel  = s.isSettled ? 'Settled' : 'Outstanding';
    final statusIcon   = s.isSettled
        ? Icons.check_circle_rounded
        : Icons.pending_outlined;

    return Card(
      margin: const EdgeInsets.symmetric(horizontal: 16, vertical: 6),
      color: cs.surfaceContainerLow,
      child: Column(
        children: [
          // ── Header row ────────────────────────────────────────────────
          InkWell(
            borderRadius: BorderRadius.circular(16),
            onTap: () => setState(() => _expanded = !_expanded),
            child: Padding(
              padding: const EdgeInsets.all(16),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Row(children: [
                    Expanded(
                      child: Text(s.periodLabel,
                          style: tt.titleSmall
                              ?.copyWith(fontWeight: FontWeight.w700)),
                    ),
                    Container(
                      padding: const EdgeInsets.symmetric(
                          horizontal: 10, vertical: 4),
                      decoration: BoxDecoration(
                        color: statusColor.withOpacity(0.12),
                        borderRadius: BorderRadius.circular(20),
                      ),
                      child: Row(mainAxisSize: MainAxisSize.min, children: [
                        Icon(statusIcon, size: 14, color: statusColor),
                        const SizedBox(width: 4),
                        Text(statusLabel,
                            style: tt.labelSmall?.copyWith(
                                color: statusColor,
                                fontWeight: FontWeight.w700)),
                      ]),
                    ),
                  ]),
                  const SizedBox(height: 12),

                  // ── Three-column summary ──────────────────────────────
                  Row(children: [
                    _BillStat(
                      label: 'Charged',
                      value: '$sym${_fmt(s.totalCharged)}',
                      color: cs.error,
                    ),
                    _BillStat(
                      label: 'Paid',
                      value: '$sym${_fmt(s.totalPaid)}',
                      color: AppTheme.green,
                    ),
                    _BillStat(
                      label: s.isSettled ? 'Settled ✓' : 'Remaining',
                      value: s.isSettled
                          ? '${sym}0'
                          : '$sym${_fmt(s.remaining)}',
                      color: s.isSettled
                          ? AppTheme.green
                          : cs.error,
                    ),
                  ]),

                  // ── Statement balance + min payment ───────────────────
                  const SizedBox(height: 8),
                  Row(children: [
                    Icon(Icons.receipt_long_outlined,
                        size: 14, color: cs.outline),
                    const SizedBox(width: 4),
                    Text(
                      'Statement balance: $sym${_fmt(s.statementBalance)}',
                      style: tt.bodySmall?.copyWith(color: cs.outline),
                    ),
                    if (s.minimumPayment != null) ...[
                      const SizedBox(width: 12),
                      Icon(Icons.payments_outlined,
                          size: 14, color: cs.outline),
                      const SizedBox(width: 4),
                      Text(
                        'Min: $sym${_fmt(s.minimumPayment!)}',
                        style: tt.bodySmall?.copyWith(color: cs.outline),
                      ),
                    ],
                  ]),

                  // ── Expand chevron ─────────────────────────────────────
                  const SizedBox(height: 4),
                  Row(
                    mainAxisAlignment: MainAxisAlignment.center,
                    children: [
                      Icon(
                        _expanded
                            ? Icons.expand_less_rounded
                            : Icons.expand_more_rounded,
                        color: cs.outline, size: 20,
                      ),
                      Text(
                        _expanded ? 'Hide transactions' : 'Show transactions',
                        style: tt.labelSmall?.copyWith(color: cs.outline),
                      ),
                    ],
                  ),
                ],
              ),
            ),
          ),

          // ── Expanded entry list ───────────────────────────────────────
          if (_expanded) ...[
            const Divider(height: 1),
            _StatementEntryList(
              accountId: widget.accountId,
              statement: s,
              card: widget.card,
              ref: widget.ref,
            ),
          ],
        ],
      ),
    );
  }

  String _fmt(String raw) {
    final parts = raw.replaceAll('-', '').split('.');
    final i = parts[0].replaceAllMapped(
      RegExp(r'(\d{1,3})(?=(\d{3})+(?!\d))'), (m) => '${m[1]},');
    return parts.length > 1 ? '$i.${parts[1]}' : i;
  }
}

class _BillStat extends StatelessWidget {
  const _BillStat({required this.label, required this.value, required this.color});
  final String label;
  final String value;
  final Color  color;

  @override
  Widget build(BuildContext context) => Expanded(
        child: Column(children: [
          Text(label,
              style: Theme.of(context).textTheme.labelSmall
                  ?.copyWith(color: Theme.of(context).colorScheme.outline)),
          const SizedBox(height: 2),
          Text(value,
              style: Theme.of(context).textTheme.titleSmall?.copyWith(
                  color: color, fontWeight: FontWeight.w700),
              overflow: TextOverflow.ellipsis),
        ]),
      );
}

// ── Statement entry list (lazy-loaded per bill) ───────────────────────────────

class _StatementEntryList extends StatelessWidget {
  const _StatementEntryList({
    required this.accountId,
    required this.statement,
    required this.card,
    required this.ref,
  });
  final String         accountId;
  final Statement      statement;
  final CreditCardInfo card;
  final WidgetRef      ref;

  @override
  Widget build(BuildContext context) {
    final key        = (accountId, statement.id) as _StmtKey;
    final entriesAsync = ref.watch(_stmtEntriesProvider(key));

    return entriesAsync.when(
      loading: () => const Padding(
        padding: EdgeInsets.all(24),
        child: Center(child: CircularProgressIndicator()),
      ),
      error: (e, _) => Padding(
        padding: const EdgeInsets.all(16),
        child: Text(e.toString(),
            style: TextStyle(color: Theme.of(context).colorScheme.error)),
      ),
      data: (entries) {
        if (entries.isEmpty) {
          return Padding(
            padding: const EdgeInsets.all(24),
            child: Text('No transactions in this cycle.',
                style: TextStyle(
                    color: Theme.of(context).colorScheme.outline),
                textAlign: TextAlign.center),
          );
        }
        return Column(
          children: entries
              .map((e) => _EntryRow(entry: e, card: card, ref: ref))
              .toList(),
        );
      },
    );
  }
}

class _EntryRow extends StatelessWidget {
  const _EntryRow({required this.entry, required this.card, required this.ref});
  final LedgerEntry    entry;
  final CreditCardInfo card;
  final WidgetRef      ref;

  @override
  Widget build(BuildContext context) {
    final cs       = Theme.of(context).colorScheme;
    final tt       = Theme.of(context).textTheme;
    final isCredit = entry.entryType == 'payment_received';
    final color    = isCredit ? AppTheme.green : cs.error;
    final sign     = isCredit ? '+' : '-';
    final timeFmt  = DateFormat('d MMM  HH:mm');

    return InkWell(
      onTap: () => _openAnnotation(context),
      child: Padding(
        padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 10),
        child: Row(
          children: [
            Container(
              width: 36, height: 36,
              decoration: BoxDecoration(
                color: isCredit
                    ? AppTheme.green.withOpacity(0.12)
                    : cs.errorContainer,
                borderRadius: BorderRadius.circular(9),
              ),
              child: Icon(
                isCredit ? Icons.payment_rounded : Icons.shopping_bag_outlined,
                size: 18,
                color: isCredit ? AppTheme.green : cs.onErrorContainer,
              ),
            ),
            const SizedBox(width: 12),
            Expanded(
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(
                    entry.label?.isNotEmpty == true
                        ? entry.label!
                        : entry.typeDisplayName,
                    style: tt.bodyMedium
                        ?.copyWith(fontWeight: FontWeight.w600),
                  ),
                  if (entry.description?.isNotEmpty == true)
                    Text(entry.description!,
                        style: tt.bodySmall?.copyWith(color: cs.outline),
                        maxLines: 1,
                        overflow: TextOverflow.ellipsis),
                ],
              ),
            ),
            Column(
              crossAxisAlignment: CrossAxisAlignment.end,
              children: [
                Text(
                  '$sign${card.symbol}${_fmt(entry.amount)}',
                  style: tt.bodyMedium
                      ?.copyWith(color: color, fontWeight: FontWeight.w700),
                ),
                Text(timeFmt.format(entry.occurredAt),
                    style: tt.labelSmall?.copyWith(color: cs.outline)),
              ],
            ),
          ],
        ),
      ),
    );
  }

  void _openAnnotation(BuildContext context) {
    showDialog(
      context: context,
      barrierColor: Colors.black.withOpacity(0.5),
      builder: (_) => _AnnotationSheet(
        entry: entry,
        onSaved: () {
          ref.invalidate(_stmtEntriesProvider(
              (entry.accountId, '') as _StmtKey));
          Navigator.pop(context);
        },
      ),
    );
  }

  String _fmt(String raw) {
    final parts = raw.split('.');
    final i = parts[0].replaceAllMapped(
      RegExp(r'(\d{1,3})(?=(\d{3})+(?!\d))'), (m) => '${m[1]},');
    return parts.length > 1 ? '$i.${parts[1]}' : i;
  }
}

// ── All Entries tab ───────────────────────────────────────────────────────────

class _AllEntriesTab extends StatelessWidget {
  const _AllEntriesTab({
    required this.accountId,
    required this.card,
    required this.ref,
  });
  final String         accountId;
  final CreditCardInfo card;
  final WidgetRef      ref;

  @override
  Widget build(BuildContext context) {
    // Uses the stable file-scope _stmtEntriesProvider to fetch all entries
    // for this account. Inline FutureProvider.family inside build() would
    // create a new provider identity on every rebuild causing infinite refetching.
    final entriesAsync = ref.watch(allEntriesProvider).whenData(
      (all) => all.where((e) => e.accountId == accountId).toList());

    return entriesAsync.when(
      loading: () => const Center(child: CircularProgressIndicator()),
      error:   (e, _) => Center(child: Text(e.toString())),
      data:    (entries) => entries.isEmpty
          ? Center(
              child: Text('No transactions yet',
                  style: TextStyle(
                      color: Theme.of(context).colorScheme.outline)))
          : ListView.builder(
              padding: const EdgeInsets.only(bottom: 40),
              itemCount: entries.length,
              itemBuilder: (ctx, i) =>
                  _EntryRow(entry: entries[i], card: card, ref: ref),
            ),
    );
  }
}

// ── Card hero ─────────────────────────────────────────────────────────────────

class _CardHero extends StatelessWidget {
  const _CardHero({required this.card});
  final CreditCardInfo card;

  @override
  Widget build(BuildContext context) {
    final cs = Theme.of(context).colorScheme;
    final tt = Theme.of(context).textTheme;

    return Container(
      margin: const EdgeInsets.all(16),
      height: 176,
      decoration: BoxDecoration(
        gradient: LinearGradient(
          colors: [cs.primary, cs.tertiary],
          begin: Alignment.topLeft,
          end: Alignment.bottomRight,
        ),
        borderRadius: BorderRadius.circular(20),
        boxShadow: [
          BoxShadow(
            color: cs.primary.withOpacity(0.35),
            blurRadius: 20, offset: const Offset(0, 8),
          ),
        ],
      ),
      child: Padding(
        padding: const EdgeInsets.all(22),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Row(
              mainAxisAlignment: MainAxisAlignment.spaceBetween,
              children: [
                Text(card.accountName,
                    style: tt.titleMedium?.copyWith(
                        color: Colors.white, fontWeight: FontWeight.w700)),
                Container(
                  padding: const EdgeInsets.symmetric(
                      horizontal: 10, vertical: 4),
                  decoration: BoxDecoration(
                    color: Colors.white.withOpacity(0.2),
                    borderRadius: BorderRadius.circular(8),
                  ),
                  child: Text(card.network,
                      style: tt.labelMedium?.copyWith(
                          color: Colors.white, fontWeight: FontWeight.w600)),
                ),
              ],
            ),
            const Spacer(),
            Text('•••• •••• •••• ${card.lastFour}',
                style: tt.titleLarge?.copyWith(
                    color: Colors.white, letterSpacing: 3,
                    fontWeight: FontWeight.w300)),
            const SizedBox(height: 12),
            Row(children: [
              _CardField('EXPIRES', card.expiry),
              const SizedBox(width: 24),
              if (card.interestRate != null)
                _CardField('APR', '${card.interestRate!.toStringAsFixed(2)}%'),
              const Spacer(),
              if (card.isOverdue)
                Container(
                  padding: const EdgeInsets.symmetric(
                      horizontal: 8, vertical: 4),
                  decoration: BoxDecoration(
                    color: AppTheme.red.withOpacity(0.8),
                    borderRadius: BorderRadius.circular(8),
                  ),
                  child: Text('OVERDUE',
                      style: tt.labelSmall?.copyWith(
                          color: Colors.white, fontWeight: FontWeight.w700)),
                ),
            ]),
          ],
        ),
      ),
    );
  }
}

class _CardField extends StatelessWidget {
  const _CardField(this.label, this.value);
  final String label, value;
  @override
  Widget build(BuildContext context) => Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text(label,
              style: const TextStyle(color: Colors.white60, fontSize: 9,
                  fontWeight: FontWeight.w600, letterSpacing: 0.8)),
          Text(value,
              style: const TextStyle(color: Colors.white, fontSize: 13)),
        ],
      );
}

// ── Key stats ─────────────────────────────────────────────────────────────────

class _KeyStats extends StatelessWidget {
  const _KeyStats({required this.card});
  final CreditCardInfo card;

  @override
  Widget build(BuildContext context) {
    final cs = Theme.of(context).colorScheme;
    final limit       = double.tryParse(card.creditLimit)    ?? 1;
    final outstanding = double.tryParse(card.outstanding)    ?? 0;
    final utilisation = limit > 0 ? (outstanding / limit) : 0.0;

    return Padding(
      padding: const EdgeInsets.symmetric(horizontal: 16),
      child: Column(
        children: [
          // Utilisation bar
          Row(
            mainAxisAlignment: MainAxisAlignment.spaceBetween,
            children: [
              Text('Utilisation',
                  style: Theme.of(context).textTheme.labelMedium
                      ?.copyWith(color: cs.outline)),
              Text('${(utilisation * 100).toStringAsFixed(1)}%',
                  style: Theme.of(context).textTheme.labelMedium?.copyWith(
                        color: utilisation > 0.8 ? cs.error : cs.primary,
                        fontWeight: FontWeight.w700,
                      )),
            ],
          ),
          const SizedBox(height: 6),
          ClipRRect(
            borderRadius: BorderRadius.circular(4),
            child: LinearProgressIndicator(
              value: utilisation.clamp(0.0, 1.0),
              minHeight: 8,
              backgroundColor: cs.surfaceContainerHighest,
              valueColor: AlwaysStoppedAnimation(
                  utilisation > 0.8 ? cs.error : cs.primary),
            ),
          ),
          const SizedBox(height: 16),
          Row(children: [
            _StatTile(label: 'Credit Limit', value: '${card.symbol}${_fmt(card.creditLimit)}',
                icon: Icons.credit_score_rounded),
            const SizedBox(width: 8),
            _StatTile(label: 'Available',   value: '${card.symbol}${_fmt(card.availableCredit)}',
                icon: Icons.check_circle_outline_rounded,
                valueColor: AppTheme.green),
          ]),
          const SizedBox(height: 8),
          Row(children: [
            _StatTile(label: 'Outstanding',       value: '${card.symbol}${_fmt(card.outstanding)}',
                icon: Icons.account_balance_wallet_outlined,
                valueColor: Theme.of(context).colorScheme.error),
            const SizedBox(width: 8),
            _StatTile(label: 'Statement Balance', value: card.statementBalance != null
                    ? '${card.symbol}${_fmt(card.statementBalance!)}'
                    : '—',
                icon: Icons.receipt_long_outlined),
          ]),
          const SizedBox(height: 8),
          Row(children: [
            _StatTile(label: 'Statement Day', value: _ord(card.statementDay),
                icon: Icons.calendar_today_rounded),
            const SizedBox(width: 8),
            _StatTile(label: 'Due Day',       value: _ord(card.dueDay),
                icon: Icons.event_available_rounded,
                valueColor: card.isOverdue ? Theme.of(context).colorScheme.error : null),
          ]),
          if (card.minimumPayment != null) ...[
            const SizedBox(height: 8),
            Card(
              color: card.minimumPaymentPaid
                  ? AppTheme.green.withOpacity(0.08)
                  : Theme.of(context).colorScheme.errorContainer,
              child: ListTile(
                dense: true,
                leading: Icon(
                  card.minimumPaymentPaid
                      ? Icons.check_circle_rounded
                      : Icons.warning_amber_rounded,
                  color: card.minimumPaymentPaid
                      ? AppTheme.green
                      : Theme.of(context).colorScheme.onErrorContainer,
                ),
                title: Text(
                  card.minimumPaymentPaid
                      ? 'Minimum payment met'
                      : 'Min due: ${card.symbol}${_fmt(card.minimumPayment!)}',
                  style: Theme.of(context).textTheme.labelMedium?.copyWith(
                        fontWeight: FontWeight.w600,
                        color: card.minimumPaymentPaid
                            ? AppTheme.green
                            : Theme.of(context).colorScheme.onErrorContainer,
                      ),
                ),
              ),
            ),
          ],
          if (card.tempLimit != null) ...[
            const SizedBox(height: 8),
            Card(
              color: Theme.of(context).colorScheme.tertiaryContainer,
              child: ListTile(
                dense: true,
                leading: Icon(Icons.flash_on_rounded,
                    color: Theme.of(context).colorScheme.onTertiaryContainer),
                title: Text(
                  'Temp limit: ${card.symbol}${_fmt(card.tempLimit!.amount)} until ${card.tempLimit!.expiresOn}',
                  style: Theme.of(context).textTheme.labelMedium?.copyWith(
                        color: Theme.of(context).colorScheme.onTertiaryContainer,
                        fontWeight: FontWeight.w600),
                ),
              ),
            ),
          ],
        ],
      ),
    );
  }

  String _fmt(String raw) {
    final parts = raw.replaceAll('-', '').split('.');
    final i = parts[0].replaceAllMapped(
      RegExp(r'(\d{1,3})(?=(\d{3})+(?!\d))'), (m) => '${m[1]},');
    return parts.length > 1 ? '$i.${parts[1]}' : i;
  }

  String _ord(int day) {
    if (day >= 11 && day <= 13) return '${day}th';
    return switch (day % 10) {
      1 => '${day}st', 2 => '${day}nd', 3 => '${day}rd', _ => '${day}th',
    };
  }
}

class _StatTile extends StatelessWidget {
  const _StatTile({required this.label, required this.value,
      required this.icon, this.valueColor});
  final String   label;
  final String   value;
  final IconData icon;
  final Color?   valueColor;

  @override
  Widget build(BuildContext context) {
    final cs = Theme.of(context).colorScheme;
    return Expanded(
      child: Card(
        color: cs.surfaceContainerLow,
        margin: EdgeInsets.zero,
        child: Padding(
          padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 10),
          child: Row(children: [
            Icon(icon, size: 16, color: cs.outline),
            const SizedBox(width: 8),
            Expanded(child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text(label,
                    style: Theme.of(context).textTheme.labelSmall
                        ?.copyWith(color: cs.outline)),
                const SizedBox(height: 2),
                Text(value,
                    style: Theme.of(context).textTheme.titleSmall?.copyWith(
                        fontWeight: FontWeight.w700,
                        color: valueColor ?? cs.onSurface),
                    overflow: TextOverflow.ellipsis),
              ],
            )),
          ]),
        ),
      ),
    );
  }
}

// ── Action row ────────────────────────────────────────────────────────────────

class _ActionRow extends StatelessWidget {
  const _ActionRow({required this.card, required this.account,
      required this.allAccounts, required this.ref, required this.onAction});
  final CreditCardInfo       card;
  final AccountSummary       account;
  final List<AccountSummary> allAccounts;
  final WidgetRef            ref;
  final VoidCallback         onAction;

  @override
  Widget build(BuildContext context) {
    return Wrap(spacing: 8, runSpacing: 8, children: [
      _Chip(icon: Icons.shopping_bag_outlined, label: 'Charge',
          color: AppTheme.amber,
          onTap: () => _open(context, ChargeSheet(account: account))),
      _Chip(icon: Icons.payment_rounded, label: 'Pay',
          color: AppTheme.green,
          onTap: () => _open(context,
              PaymentSheet(debtAccount: account, allAccounts: allAccounts))),
      _Chip(icon: Icons.receipt_long_rounded, label: 'Close Statement',
          color: AppTheme.purple,
          onTap: () => _open(context, CloseStatementSheet(account: account))),
    ]);
  }

  Future<void> _open(BuildContext context, Widget sheet) async {
    final result = await showFormSheet<bool>(context, sheet);
    if (result == true) onAction();
  }
}

class _Chip extends StatelessWidget {
  const _Chip({required this.icon, required this.label,
      required this.color, required this.onTap});
  final IconData icon; final String label;
  final Color color; final VoidCallback onTap;

  @override
  Widget build(BuildContext context) => InkWell(
        borderRadius: BorderRadius.circular(20),
        onTap: onTap,
        child: Container(
          padding: const EdgeInsets.symmetric(horizontal: 14, vertical: 8),
          decoration: BoxDecoration(
            color: color.withOpacity(0.1),
            borderRadius: BorderRadius.circular(20),
            border: Border.all(color: color.withOpacity(0.3)),
          ),
          child: Row(mainAxisSize: MainAxisSize.min, children: [
            Icon(icon, size: 16, color: color),
            const SizedBox(width: 6),
            Text(label, style: TextStyle(color: color,
                fontWeight: FontWeight.w600, fontSize: 13)),
          ]),
        ),
      );
}

// ── No bills placeholder ──────────────────────────────────────────────────────

class _NoBillsView extends StatelessWidget {
  @override
  Widget build(BuildContext context) => Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            Icon(Icons.receipt_long_outlined, size: 56,
                color: Theme.of(context).colorScheme.outline),
            const SizedBox(height: 16),
            Text('No statements yet',
                style: Theme.of(context).textTheme.titleMedium),
            const SizedBox(height: 8),
            Text('Close a statement to create a billing cycle record.',
                style: Theme.of(context).textTheme.bodySmall?.copyWith(
                    color: Theme.of(context).colorScheme.outline),
                textAlign: TextAlign.center),
          ],
        ),
      );
}

// ── Annotation sheet ──────────────────────────────────────────────────────────

class _AnnotationSheet extends ConsumerStatefulWidget {
  const _AnnotationSheet({required this.entry, required this.onSaved});
  final LedgerEntry  entry;
  final VoidCallback onSaved;

  @override
  ConsumerState<_AnnotationSheet> createState() => _AnnotationSheetState();
}

class _AnnotationSheetState extends ConsumerState<_AnnotationSheet> {
  late final TextEditingController _label;
  late final TextEditingController _desc;
  bool _loading = false; String? _error;

  @override
  void initState() {
    super.initState();
    _label = TextEditingController(text: widget.entry.label ?? '');
    _desc  = TextEditingController(text: widget.entry.description ?? '');
  }

  @override
  void dispose() { _label.dispose(); _desc.dispose(); super.dispose(); }

  Future<void> _save() async {
    setState(() { _loading = true; _error = null; });
    try {
      await ref.read(ledgerRepoProvider).updateAnnotation(widget.entry.id,
          label: _label.text.trim().isNotEmpty ? _label.text.trim() : null,
          description: _desc.text.trim().isNotEmpty ? _desc.text.trim() : null);
      widget.onSaved();
    } catch (e) {
      setState(() { _error = e.toString(); _loading = false; });
    }
  }

  @override
  Widget build(BuildContext context) {
    final cs     = Theme.of(context).colorScheme;
    final bottom = MediaQuery.of(context).viewInsets.bottom;

    return Center(
      child: ConstrainedBox(
        constraints: const BoxConstraints(maxWidth: 460),
        child: Material(
          color: Colors.transparent,
          child: Container(
            margin: const EdgeInsets.symmetric(horizontal: 24, vertical: 32),
            decoration: BoxDecoration(
              color: cs.surface,
              borderRadius: BorderRadius.circular(20),
              boxShadow: [
                BoxShadow(
                  color: Colors.black.withOpacity(0.25),
                  blurRadius: 40,
                  offset: const Offset(0, 8),
                ),
              ],
            ),
            padding: const EdgeInsets.fromLTRB(20, 20, 20, 24),
            child: Column(
              mainAxisSize: MainAxisSize.min,
              children: [
                Row(children: [
                  Expanded(child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Text('Add Note',
                          style: Theme.of(context).textTheme.titleLarge
                              ?.copyWith(fontWeight: FontWeight.w700)),
                      Text('${widget.entry.typeDisplayName} · ${widget.entry.amount}',
                          style: Theme.of(context).textTheme.bodySmall
                              ?.copyWith(color: cs.outline)),
                    ],
                  )),
              IconButton(onPressed: () => Navigator.pop(context),
                  icon: const Icon(Icons.close_rounded)),
            ]),
            const SizedBox(height: 16),
            TextField(
              controller: _label,
              textCapitalization: TextCapitalization.words,
              decoration: const InputDecoration(
                labelText: 'Label',
                hintText: 'e.g. Lunch, Groceries',
                prefixIcon: Icon(Icons.label_outline_rounded),
                border: OutlineInputBorder(
                    borderRadius: BorderRadius.all(Radius.circular(12))),
                filled: true,
              ),
            ),
            const SizedBox(height: 12),
            TextField(
              controller: _desc, maxLines: 2,
              textCapitalization: TextCapitalization.sentences,
              decoration: const InputDecoration(
                labelText: 'Description',
                hintText: "e.g. McDonald's double cheeseburger + fries",
                prefixIcon: Padding(padding: EdgeInsets.only(bottom: 24),
                    child: Icon(Icons.notes_rounded)),
                border: OutlineInputBorder(
                    borderRadius: BorderRadius.all(Radius.circular(12))),
                filled: true,
              ),
            ),
            if (_error != null) ...[
              const SizedBox(height: 8),
              Text(_error!, style: TextStyle(color: cs.error, fontSize: 12)),
            ],
            const SizedBox(height: 16),
            SizedBox(
              width: double.infinity, height: 50,
              child: FilledButton(
                onPressed: _loading ? null : _save,
                child: _loading
                    ? const SizedBox(width: 20, height: 20,
                        child: CircularProgressIndicator(strokeWidth: 2))
                    : const Text('Save',
                        style: TextStyle(fontWeight: FontWeight.w600)),
              ),
            ),
          ],
        ),
      ),
    ),
  ),
);
  }
}
