import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../core/providers.dart';
import '../../models/ledger_entry.dart';
import '../../models/account_summary.dart';
import '../../theme/app_theme.dart';

class LedgerScreen extends ConsumerStatefulWidget {
  const LedgerScreen({super.key});
  @override
  ConsumerState<LedgerScreen> createState() => _LedgerScreenState();
}

class _LedgerScreenState extends ConsumerState<LedgerScreen> {
  final _search = TextEditingController();

  // ── Filter state ──────────────────────────────────────────────────────────
  String  _query          = '';
  String? _accountId;           // null = all accounts
  String? _entryType;           // null = all types
  DateTime? _from;
  DateTime? _to;
  bool    _onlyCredits    = false;
  bool    _onlyDebits     = false;

  static const _typeOptions = {
    'deposit':          'Deposit',
    'withdrawal':       'Withdrawal',
    'charge':           'Purchase',
    'payment_made':     'Payment Made',
    'payment_received': 'Payment Received',
    'interest_accrued': 'Interest',
    'statement_closed': 'Statement Closed',
  };

  @override
  void dispose() {
    _search.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final entriesAsync  = ref.watch(allEntriesProvider);
    final accountsAsync = ref.watch(accountsProvider);
    final accounts      = accountsAsync.asData?.value ?? [];

    return Scaffold(
      backgroundColor: AppTheme.pageBg,
      appBar: AppBar(
        automaticallyImplyLeading: false,
        title: const Text('Ledger'),
        actions: [
          IconButton(
            icon: const Icon(Icons.refresh_rounded),
            onPressed: () => refreshDashboard(ref),
          ),
        ],
      ),
      body: Column(
        children: [
          // ── Search bar ─────────────────────────────────────────────────────
          Padding(
            padding: const EdgeInsets.fromLTRB(20, 12, 20, 0),
            child: TextField(
              controller: _search,
              onChanged: (v) => setState(() => _query = v.trim()),
              style: const TextStyle(color: AppTheme.textPrimary),
              decoration: InputDecoration(
                hintText: 'Search labels, descriptions, amounts…',
                prefixIcon: const Icon(Icons.search_rounded,
                    color: AppTheme.textSecondary),
                suffixIcon: _query.isNotEmpty
                    ? IconButton(
                        icon: const Icon(Icons.close_rounded,
                            size: 18, color: AppTheme.textSecondary),
                        onPressed: () {
                          _search.clear();
                          setState(() => _query = '');
                        },
                      )
                    : null,
              ),
            ),
          ),

          // ── Filter chips row ───────────────────────────────────────────────
          SizedBox(
            height: 52,
            child: ListView(
              scrollDirection: Axis.horizontal,
              padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 10),
              children: [
                // Account filter
                _FilterChip(
                  label: _accountId == null
                      ? 'All accounts'
                      : accounts
                              .where((a) => a.accountId == _accountId)
                              .firstOrNull
                              ?.accountName ??
                          'Account',
                  icon: Icons.account_balance_rounded,
                  active: _accountId != null,
                  onTap: () => _pickAccount(context, accounts),
                ),
                const SizedBox(width: 8),

                // Type filter
                _FilterChip(
                  label: _entryType == null
                      ? 'All types'
                      : _typeOptions[_entryType] ?? _entryType!,
                  icon: Icons.filter_list_rounded,
                  active: _entryType != null,
                  onTap: () => _pickType(context),
                ),
                const SizedBox(width: 8),

                // Date range filter
                _FilterChip(
                  label: (_from == null && _to == null)
                      ? 'Any date'
                      : _from != null && _to != null
                          ? '${_fmtDate(_from!)} – ${_fmtDate(_to!)}'
                          : _from != null
                              ? 'From ${_fmtDate(_from!)}'
                              : 'Until ${_fmtDate(_to!)}',
                  icon: Icons.calendar_today_rounded,
                  active: _from != null || _to != null,
                  onTap: () => _pickDateRange(context),
                ),
                const SizedBox(width: 8),

                // Credits only
                _FilterChip(
                  label: 'Credits',
                  icon: Icons.arrow_downward_rounded,
                  active: _onlyCredits,
                  onTap: () => setState(() {
                    _onlyCredits = !_onlyCredits;
                    if (_onlyCredits) _onlyDebits = false;
                  }),
                ),
                const SizedBox(width: 8),

                // Debits only
                _FilterChip(
                  label: 'Debits',
                  icon: Icons.arrow_upward_rounded,
                  active: _onlyDebits,
                  onTap: () => setState(() {
                    _onlyDebits = !_onlyDebits;
                    if (_onlyDebits) _onlyCredits = false;
                  }),
                ),
                const SizedBox(width: 8),

                // Clear all
                if (_hasAnyFilter)
                  _FilterChip(
                    label: 'Clear',
                    icon: Icons.close_rounded,
                    active: false,
                    isDestructive: true,
                    onTap: _clearFilters,
                  ),
              ],
            ),
          ),

          const Divider(height: 1, color: AppTheme.border),

          // ── Entry list ─────────────────────────────────────────────────────
          Expanded(
            child: entriesAsync.when(
              loading: () => const Center(child: CircularProgressIndicator()),
              error: (e, _) => Center(
                  child: Text('Error: $e',
                      style: const TextStyle(color: AppTheme.red))),
              data: (entries) {
                final filtered = _applyFilters(entries, accounts);
                if (filtered.isEmpty) {
                  return const Center(
                    child: Column(
                      mainAxisSize: MainAxisSize.min,
                      children: [
                        Icon(Icons.receipt_long_outlined,
                            size: 48, color: AppTheme.textSecondary),
                        SizedBox(height: 12),
                        Text('No transactions match',
                            style: TextStyle(
                                color: AppTheme.textSecondary, fontSize: 14)),
                      ],
                    ),
                  );
                }
                return ListView.builder(
                  padding: const EdgeInsets.only(bottom: 32),
                  itemCount: filtered.length,
                  itemBuilder: (_, i) => _EntryRow(
                    entry: filtered[i],
                    accountName: accounts
                            .where((a) => a.accountId == filtered[i].accountId)
                            .firstOrNull
                            ?.accountName ??
                        '',
                  ),
                );
              },
            ),
          ),
        ],
      ),
    );
  }

  bool get _hasAnyFilter =>
      _accountId != null ||
      _entryType != null ||
      _from != null ||
      _to != null ||
      _onlyCredits ||
      _onlyDebits ||
      _query.isNotEmpty;

  void _clearFilters() => setState(() {
        _accountId   = null;
        _entryType   = null;
        _from        = null;
        _to          = null;
        _onlyCredits = false;
        _onlyDebits  = false;
        _query       = '';
        _search.clear();
      });

  List<LedgerEntry> _applyFilters(
      List<LedgerEntry> entries, List<AccountSummary> accounts) {
    return entries.where((e) {
      if (_accountId != null && e.accountId != _accountId) return false;
      if (_entryType != null && e.entryType != _entryType) return false;
      if (_from != null && e.occurredAt.isBefore(_from!)) return false;
      if (_to != null) {
        final endOfDay = DateTime(_to!.year, _to!.month, _to!.day, 23, 59, 59);
        if (e.occurredAt.isAfter(endOfDay)) return false;
      }
      if (_onlyCredits && !e.isCredit) return false;
      if (_onlyDebits  && e.isCredit)  return false;

      // In the global ledger (no account filter), hide the incoming " ← " side
      // of transfers so each transfer appears exactly once. The outgoing " → "
      // side is kept as the canonical row and rendered with a swap icon.
      if (_accountId == null && _isIncomingTransfer(e)) return false;

      if (_query.isNotEmpty) {
        final q = _query.toLowerCase();
        final matchLabel = e.label?.toLowerCase().contains(q) ?? false;
        final matchDesc  = e.description?.toLowerCase().contains(q) ?? false;
        final matchAmt   = e.amount.contains(q);
        final matchType  = e.typeDisplayName.toLowerCase().contains(q);
        final matchAcct  = accounts
            .where((a) => a.accountId == e.accountId)
            .firstOrNull
            ?.accountName
            .toLowerCase()
            .contains(q) ?? false;
        if (!matchLabel && !matchDesc && !matchAmt && !matchType && !matchAcct) {
          return false;
        }
      }
      return true;
    }).toList();
  }

  /// Returns true when this entry is the destination/incoming half of a transfer,
  /// identified by a label containing " ← " (set by transfer_funds use case).
  static bool _isIncomingTransfer(LedgerEntry e) =>
      e.isCredit && (e.label?.contains(' ← ') ?? false);


  Future<void> _pickAccount(
      BuildContext context, List<AccountSummary> accounts) async {
    final picked = await showDialog<String?>(
      context: context,
      barrierColor: Colors.black.withOpacity(0.5),
      builder: (_) => Center(
        child: ConstrainedBox(
          constraints: const BoxConstraints(maxWidth: 400),
          child: Material(
            color: Colors.transparent,
            child: Container(
              margin: const EdgeInsets.symmetric(horizontal: 24, vertical: 32),
              decoration: BoxDecoration(
                color: AppTheme.surface,
                borderRadius: BorderRadius.circular(20),
                boxShadow: [
                  BoxShadow(
                    color: Colors.black.withOpacity(0.25),
                    blurRadius: 40,
                    offset: const Offset(0, 8),
                  ),
                ],
              ),
              child: Column(
                mainAxisSize: MainAxisSize.min,
                children: [
                  Padding(
                    padding: const EdgeInsets.fromLTRB(20, 20, 16, 12),
                    child: Row(
                      children: [
                        const Expanded(
                          child: Text('Filter by account',
                              style: TextStyle(
                                  color: AppTheme.textPrimary,
                                  fontSize: 16,
                                  fontWeight: FontWeight.w600)),
                        ),
                        IconButton(
                          onPressed: () => Navigator.pop(context),
                          icon: const Icon(Icons.close_rounded,
                              color: AppTheme.textSecondary),
                          style: IconButton.styleFrom(
                            minimumSize: const Size(36, 36),
                            maximumSize: const Size(36, 36),
                            padding: EdgeInsets.zero,
                          ),
                        ),
                      ],
                    ),
                  ),
                  const Divider(height: 1),
                  ListTile(
                    leading: const Icon(Icons.all_inclusive_rounded,
                        color: AppTheme.textSecondary),
                    title: const Text('All accounts',
                        style: TextStyle(color: AppTheme.textPrimary)),
                    onTap: () => Navigator.pop(context, ''),
                  ),
                  ...accounts.map((a) => ListTile(
                        leading: const Icon(Icons.account_circle_outlined,
                            color: AppTheme.purple),
                        title: Text(a.accountName,
                            style: const TextStyle(color: AppTheme.textPrimary)),
                        subtitle: Text(a.typeLabel,
                            style: const TextStyle(
                                color: AppTheme.textSecondary, fontSize: 12)),
                        onTap: () => Navigator.pop(context, a.accountId),
                      )),
                  const SizedBox(height: 8),
                ],
              ),
            ),
          ),
        ),
      ),
    );
    if (picked == null) return;
    setState(() => _accountId = picked.isEmpty ? null : picked);
  }

  Future<void> _pickType(BuildContext context) async {
    final picked = await showDialog<String?>(
      context: context,
      barrierColor: Colors.black.withOpacity(0.5),
      builder: (_) => Center(
        child: ConstrainedBox(
          constraints: const BoxConstraints(maxWidth: 400),
          child: Material(
            color: Colors.transparent,
            child: Container(
              margin: const EdgeInsets.symmetric(horizontal: 24, vertical: 32),
              decoration: BoxDecoration(
                color: AppTheme.surface,
                borderRadius: BorderRadius.circular(20),
                boxShadow: [
                  BoxShadow(
                    color: Colors.black.withOpacity(0.25),
                    blurRadius: 40,
                    offset: const Offset(0, 8),
                  ),
                ],
              ),
              child: Column(
                mainAxisSize: MainAxisSize.min,
                children: [
                  Padding(
                    padding: const EdgeInsets.fromLTRB(20, 20, 16, 12),
                    child: Row(
                      children: [
                        const Expanded(
                          child: Text('Filter by type',
                              style: TextStyle(
                                  color: AppTheme.textPrimary,
                                  fontSize: 16,
                                  fontWeight: FontWeight.w600)),
                        ),
                        IconButton(
                          onPressed: () => Navigator.pop(context),
                          icon: const Icon(Icons.close_rounded,
                              color: AppTheme.textSecondary),
                          style: IconButton.styleFrom(
                            minimumSize: const Size(36, 36),
                            maximumSize: const Size(36, 36),
                            padding: EdgeInsets.zero,
                          ),
                        ),
                      ],
                    ),
                  ),
                  const Divider(height: 1),
                  ListTile(
                    leading: const Icon(Icons.all_inclusive_rounded,
                        color: AppTheme.textSecondary),
                    title: const Text('All types',
                        style: TextStyle(color: AppTheme.textPrimary)),
                    onTap: () => Navigator.pop(context, ''),
                  ),
                  ..._typeOptions.entries.map((e) => ListTile(
                        leading: Icon(
                          _iconForType(e.key),
                          color: _colorForType(e.key),
                          size: 20,
                        ),
                        title: Text(e.value,
                            style: const TextStyle(color: AppTheme.textPrimary)),
                        onTap: () => Navigator.pop(context, e.key),
                      )),
                  const SizedBox(height: 8),
                ],
              ),
            ),
          ),
        ),
      ),
    );
    if (picked == null) return;
    setState(() => _entryType = picked.isEmpty ? null : picked);
  }

  Future<void> _pickDateRange(BuildContext context) async {
    final now = DateTime.now();
    final range = await showDateRangePicker(
      context: context,
      firstDate: DateTime(now.year - 5),
      lastDate: now,
      initialDateRange: _from != null && _to != null
          ? DateTimeRange(start: _from!, end: _to!)
          : null,
      builder: (ctx, child) => Theme(
        data: Theme.of(ctx).copyWith(
          colorScheme: const ColorScheme.dark(
            primary: AppTheme.purple,
            onPrimary: Colors.white,
            surface: AppTheme.surface,
            onSurface: AppTheme.textPrimary,
          ),
        ),
        child: child!,
      ),
    );
    if (range == null) return;
    setState(() {
      _from = range.start;
      _to   = range.end;
    });
  }

  String _fmtDate(DateTime d) =>
      '${d.month}/${d.day}/${d.year.toString().substring(2)}';

  IconData _iconForType(String type) => switch (type) {
        'deposit'          => Icons.arrow_downward_rounded,
        'withdrawal'       => Icons.arrow_upward_rounded,
        'charge'           => Icons.shopping_bag_outlined,
        'payment_made'     => Icons.payment_rounded,
        'payment_received' => Icons.payments_outlined,
        'interest_accrued' => Icons.percent_rounded,
        'statement_closed' => Icons.receipt_long_rounded,
        _                  => Icons.swap_horiz_rounded,
      };

  Color _colorForType(String type) => switch (type) {
        'deposit' || 'payment_received' => AppTheme.green,
        'charge'  || 'interest_accrued' => AppTheme.red,
        _                               => AppTheme.textSecondary,
      };
}

// ── Entry row ─────────────────────────────────────────────────────────────────

class _EntryRow extends StatelessWidget {
  const _EntryRow({required this.entry, required this.accountName});
  final LedgerEntry entry;
  final String      accountName;

  /// True when this is the outgoing half of a transfer (label has " → ").
  bool get _isTransfer => entry.label?.contains(' → ') ?? false;

  /// Extracts just the base label before the arrow, e.g. "Transfer" from "Transfer → Savings".
  String get _baseLabel {
    final lbl = entry.label ?? '';
    final idx = lbl.indexOf(' → ');
    return idx > 0 ? lbl.substring(0, idx) : lbl;
  }

  /// Extracts the destination account name from the arrow label.
  String get _destination {
    final lbl = entry.label ?? '';
    final idx = lbl.indexOf(' → ');
    return idx >= 0 ? lbl.substring(idx + 3) : '';
  }

  @override
  Widget build(BuildContext context) {
    final symbol = entry.currency == 'TWD' ? 'NT\$' : '\$';
    final date   = entry.occurredAt.toLocal();

    // Transfer row — neutral purple with swap icon
    if (_isTransfer) {
      return Container(
        margin: const EdgeInsets.symmetric(horizontal: 16, vertical: 4),
        decoration: AppTheme.glowCard(),
        padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 14),
        child: Row(
          children: [
            Container(
              width: 40, height: 40,
              decoration: BoxDecoration(
                color: AppTheme.purple.withOpacity(0.12),
                borderRadius: BorderRadius.circular(10),
              ),
              child: const Icon(Icons.swap_horiz_rounded,
                  color: AppTheme.purpleLight, size: 20),
            ),
            const SizedBox(width: 14),
            Expanded(
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(_baseLabel,
                      style: const TextStyle(
                          color: AppTheme.textPrimary,
                          fontWeight: FontWeight.w500,
                          fontSize: 14)),
                  const SizedBox(height: 3),
                  Row(children: [
                    // Source → Destination
                    Text(accountName,
                        style: const TextStyle(
                            color: AppTheme.purple,
                            fontSize: 11,
                            fontWeight: FontWeight.w500)),
                    const Text(' → ',
                        style: TextStyle(
                            color: AppTheme.textTertiary, fontSize: 11)),
                    Text(_destination,
                        style: const TextStyle(
                            color: AppTheme.purple,
                            fontSize: 11,
                            fontWeight: FontWeight.w500)),
                    const Text(' · ',
                        style: TextStyle(
                            color: AppTheme.textTertiary, fontSize: 11)),
                    Text('${date.day}/${date.month}/${date.year}',
                        style: const TextStyle(
                            color: AppTheme.textSecondary, fontSize: 11)),
                  ]),
                ],
              ),
            ),
            Text(
              '$symbol${entry.amount}',
              style: const TextStyle(
                  color: AppTheme.purpleLight,
                  fontWeight: FontWeight.w700,
                  fontSize: 14),
            ),
          ],
        ),
      );
    }

    // Regular entry row
    final isCredit = entry.isCredit;
    final color    = isCredit ? AppTheme.green : AppTheme.red;

    return Container(
      margin: const EdgeInsets.symmetric(horizontal: 16, vertical: 4),
      decoration: AppTheme.glowCard(),
      padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 14),
      child: Row(
        children: [
          Container(
            width: 40, height: 40,
            decoration: BoxDecoration(
              color: color.withOpacity(0.1),
              borderRadius: BorderRadius.circular(10),
            ),
            child: Icon(
              isCredit ? Icons.arrow_downward_rounded : Icons.arrow_upward_rounded,
              color: color, size: 18,
            ),
          ),
          const SizedBox(width: 14),
          Expanded(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text(
                  entry.label ?? entry.typeDisplayName,
                  style: const TextStyle(
                      color: AppTheme.textPrimary,
                      fontWeight: FontWeight.w500,
                      fontSize: 14),
                ),
                const SizedBox(height: 3),
                Row(children: [
                  if (accountName.isNotEmpty) ...[
                    Text(accountName,
                        style: const TextStyle(
                            color: AppTheme.purple, fontSize: 11,
                            fontWeight: FontWeight.w500)),
                    const Text(' · ',
                        style: TextStyle(
                            color: AppTheme.textTertiary, fontSize: 11)),
                  ],
                  Text(
                    '${date.day}/${date.month}/${date.year}',
                    style: const TextStyle(
                        color: AppTheme.textSecondary, fontSize: 11),
                  ),
                  if (entry.description != null && entry.description!.isNotEmpty) ...[
                    const Text(' · ',
                        style: TextStyle(
                            color: AppTheme.textTertiary, fontSize: 11)),
                    Flexible(
                      child: Text(
                        entry.description!,
                        style: const TextStyle(
                            color: AppTheme.textTertiary, fontSize: 11),
                        overflow: TextOverflow.ellipsis,
                      ),
                    ),
                  ],
                ]),
              ],
            ),
          ),
          Text(
            '${isCredit ? '+' : '-'}$symbol${entry.amount}',
            style: TextStyle(
                color: color, fontWeight: FontWeight.w700, fontSize: 14),
          ),
        ],
      ),
    );
  }
}


// ── Filter chip ───────────────────────────────────────────────────────────────

class _FilterChip extends StatelessWidget {
  const _FilterChip({
    required this.label,
    required this.icon,
    required this.active,
    required this.onTap,
    this.isDestructive = false,
  });
  final String   label;
  final IconData icon;
  final bool     active;
  final bool     isDestructive;
  final VoidCallback onTap;

  @override
  Widget build(BuildContext context) {
    final color = isDestructive
        ? AppTheme.red
        : active
            ? AppTheme.purple
            : AppTheme.textSecondary;

    return GestureDetector(
      onTap: onTap,
      child: Container(
        padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 6),
        decoration: BoxDecoration(
          color: active || isDestructive
              ? color.withOpacity(0.12)
              : AppTheme.surface,
          borderRadius: BorderRadius.circular(20),
          border: Border.all(
            color: active || isDestructive
                ? color.withOpacity(0.5)
                : AppTheme.border,
          ),
        ),
        child: Row(
          mainAxisSize: MainAxisSize.min,
          children: [
            Icon(icon, size: 13, color: color),
            const SizedBox(width: 6),
            Text(label,
                style: TextStyle(
                    color: color, fontSize: 12, fontWeight: FontWeight.w500)),
          ],
        ),
      ),
    );
  }
}


