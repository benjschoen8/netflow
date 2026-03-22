import '../../theme/app_theme.dart';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:intl/intl.dart';
import '../../core/providers.dart';
import '../../models/account_summary.dart';
import '../../models/ledger_entry.dart';

// ── Screen ────────────────────────────────────────────────────────────────────

/// The History tab for a specific account.
/// Conceptually this is the Ledger screen with a predefined account filter —
/// it reads from [allEntriesProvider] (the shared cache) rather than making
/// its own per-account fetch, so data is always consistent with the Ledger.
class AccountEntriesScreen extends ConsumerWidget {
  const AccountEntriesScreen({super.key, required this.account});
  final AccountSummary account;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final entriesAsync = ref.watch(allEntriesProvider);

    return Scaffold(
      body: entriesAsync.when(
        loading: () => const Center(child: CircularProgressIndicator()),
        error:   (e, _) => _ErrorView(message: e.toString()),
        data:    (all) {
          // Filter to this account only — both sides of transfers are shown
          // (the user wants to see the full picture for their account).
          final entries = all
              .where((e) => e.accountId == account.accountId)
              .toList();
          return entries.isEmpty
              ? const _EmptyView()
              : _EntryList(entries: entries, account: account, ref: ref);
        },
      ),
    );
  }
}

// ── Entry list ────────────────────────────────────────────────────────────────

class _EntryList extends StatelessWidget {
  const _EntryList({
    required this.entries,
    required this.account,
    required this.ref,
  });
  final List<LedgerEntry> entries;
  final AccountSummary account;
  final WidgetRef ref;

  @override
  Widget build(BuildContext context) {
    // Group by date
    final groups = <String, List<LedgerEntry>>{};
    final dateFmt = DateFormat('EEEE, d MMMM yyyy');
    for (final e in entries) {
      final key = dateFmt.format(e.occurredAt);
      groups.putIfAbsent(key, () => []).add(e);
    }

    return ListView.builder(
      padding: const EdgeInsets.only(bottom: 32),
      itemCount: groups.length,
      itemBuilder: (context, i) {
        final date = groups.keys.elementAt(i);
        final dayEntries = groups.values.elementAt(i);
        return Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            _DateHeader(date: date),
            ...dayEntries.map((e) => _EntryTile(
                  entry: e,
                  account: account,
                  onTap: () => _openAnnotationSheet(context, e),
                )),
          ],
        );
      },
    );
  }

  void _openAnnotationSheet(BuildContext context, LedgerEntry entry) {
    showDialog(
      context: context,
      barrierColor: Colors.black.withOpacity(0.5),
      builder: (_) => _AnnotationSheet(
        entry: entry,
        onSaved: () {
          ref.invalidate(allEntriesProvider);
          Navigator.pop(context);
        },
      ),
    );
  }
}

// ── Date header ───────────────────────────────────────────────────────────────

class _DateHeader extends StatelessWidget {
  const _DateHeader({required this.date});
  final String date;

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.fromLTRB(16, 20, 16, 6),
      child: Text(
        date,
        style: Theme.of(context).textTheme.labelSmall?.copyWith(
              color: Theme.of(context).colorScheme.outline,
              letterSpacing: 0.5,
              fontWeight: FontWeight.w600,
            ),
      ),
    );
  }
}

// ── Entry tile ────────────────────────────────────────────────────────────────

class _EntryTile extends StatelessWidget {
  const _EntryTile({
    required this.entry,
    required this.account,
    required this.onTap,
  });
  final LedgerEntry entry;
  final AccountSummary account;
  final VoidCallback onTap;

  @override
  Widget build(BuildContext context) {
    final cs     = Theme.of(context).colorScheme;
    final tt     = Theme.of(context).textTheme;
    final isDebt = account.isDebt;
    // For asset accounts: deposit = green, withdrawal = red
    // For debt accounts:  payment = green, charge = red
    final isPositive = isDebt
        ? entry.entryType == 'payment_received'
        : entry.isCredit;
    final amountColor = isPositive ? AppTheme.green : cs.error;
    final sign        = isPositive ? '+' : '-';
    final symbol      = entry.currency == 'TWD' ? 'NT\$' : '\$';
    final timeFmt     = DateFormat('HH:mm');

    return InkWell(
      onTap: onTap,
      child: Padding(
        padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 2),
        child: Card(
          color: cs.surfaceContainerLow,
          child: Padding(
            padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 12),
            child: Row(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                // ── Type icon ─────────────────────────────────────────────
                Container(
                  width: 40, height: 40,
                  decoration: BoxDecoration(
                    color: isPositive
                        ? AppTheme.green.withOpacity(0.12)
                        : cs.errorContainer,
                    borderRadius: BorderRadius.circular(10),
                  ),
                  child: Icon(
                    _iconForType(entry.entryType),
                    size: 20,
                    color: isPositive ? AppTheme.green : cs.onErrorContainer,
                  ),
                ),
                const SizedBox(width: 12),

                // ── Label / description ───────────────────────────────────
                Expanded(
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      // Primary: user label if set, else entry type name
                      Text(
                        entry.label?.isNotEmpty == true
                            ? entry.label!
                            : entry.typeDisplayName,
                        style: tt.titleSmall?.copyWith(
                          fontWeight: FontWeight.w600,
                        ),
                      ),
                      // Secondary: description if set
                      if (entry.description?.isNotEmpty == true) ...[
                        const SizedBox(height: 2),
                        Text(
                          entry.description!,
                          style: tt.bodySmall?.copyWith(color: cs.outline),
                          maxLines: 2,
                          overflow: TextOverflow.ellipsis,
                        ),
                      ] else if (entry.label?.isNotEmpty == true) ...[
                        // Show type name as subtitle when label is custom
                        const SizedBox(height: 2),
                        Text(
                          entry.typeDisplayName,
                          style: tt.bodySmall?.copyWith(color: cs.outline),
                        ),
                      ],
                    ],
                  ),
                ),

                // ── Amount + time ─────────────────────────────────────────
                Column(
                  crossAxisAlignment: CrossAxisAlignment.end,
                  children: [
                    Text(
                      '$sign$symbol${_fmtAmount(entry.amount)}',
                      style: tt.titleSmall?.copyWith(
                        color: amountColor,
                        fontWeight: FontWeight.w700,
                      ),
                    ),
                    const SizedBox(height: 2),
                    Text(
                      timeFmt.format(entry.occurredAt),
                      style: tt.labelSmall?.copyWith(color: cs.outline),
                    ),
                    // Edit indicator
                    const SizedBox(height: 2),
                    Icon(Icons.edit_note_rounded, size: 14, color: cs.outlineVariant),
                  ],
                ),
              ],
            ),
          ),
        ),
      ),
    );
  }

  IconData _iconForType(String type) => switch (type) {
        'deposit'          => Icons.add_rounded,
        'withdrawal'       => Icons.remove_rounded,
        'charge'           => Icons.shopping_bag_outlined,
        'payment_made'     => Icons.payment_rounded,
        'payment_received' => Icons.check_circle_outline_rounded,
        'interest_accrued' => Icons.percent_rounded,
        'statement_closed' => Icons.receipt_long_rounded,
        _                  => Icons.swap_horiz_rounded,
      };

  String _fmtAmount(String raw) {
    final parts   = raw.replaceAll('-', '').split('.');
    final intPart = parts[0].replaceAllMapped(
      RegExp(r'(\d{1,3})(?=(\d{3})+(?!\d))'),
      (m) => '${m[1]},',
    );
    return parts.length > 1 ? '$intPart.${parts[1]}' : intPart;
  }
}

// ── Annotation sheet — edit label + description ───────────────────────────────

class _AnnotationSheet extends ConsumerStatefulWidget {
  const _AnnotationSheet({required this.entry, required this.onSaved});
  final LedgerEntry entry;
  final VoidCallback onSaved;

  @override
  ConsumerState<_AnnotationSheet> createState() => _AnnotationSheetState();
}

class _AnnotationSheetState extends ConsumerState<_AnnotationSheet> {
  late final TextEditingController _label;
  late final TextEditingController _description;
  bool _loading = false;
  String? _error;

  @override
  void initState() {
    super.initState();
    _label       = TextEditingController(text: widget.entry.label ?? '');
    _description = TextEditingController(text: widget.entry.description ?? '');
  }

  @override
  void dispose() {
    _label.dispose();
    _description.dispose();
    super.dispose();
  }

  Future<void> _save() async {
    setState(() { _loading = true; _error = null; });
    try {
      await ref.read(ledgerRepoProvider).updateAnnotation(
        widget.entry.id,
        label:       _label.text.trim().isNotEmpty ? _label.text.trim() : null,
        description: _description.text.trim().isNotEmpty ? _description.text.trim() : null,
      );
      widget.onSaved();
    } catch (e) {
      setState(() { _error = e.toString(); _loading = false; });
    }
  }

  @override
  Widget build(BuildContext context) {
    final cs     = Theme.of(context).colorScheme;
    final tt     = Theme.of(context).textTheme;
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
            child: Padding(
              padding: const EdgeInsets.fromLTRB(20, 20, 20, 24),
              child: Column(
                mainAxisSize: MainAxisSize.min,
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [

              // Entry summary header
              Row(
                children: [
                  Expanded(
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        Text('Add Note',
                            style: tt.titleLarge?.copyWith(
                                fontWeight: FontWeight.w700)),
                        const SizedBox(height: 2),
                        Text(
                          '${widget.entry.typeDisplayName} · '
                          '${widget.entry.currency} ${widget.entry.amount}',
                          style: tt.bodySmall?.copyWith(color: cs.outline),
                        ),
                      ],
                    ),
                  ),
                  IconButton(
                    onPressed: () => Navigator.pop(context),
                    icon: const Icon(Icons.close_rounded),
                  ),
                ],
              ),
              const SizedBox(height: 20),

              // Label field
              TextField(
                controller: _label,
                textCapitalization: TextCapitalization.words,
                decoration: InputDecoration(
                  labelText: 'Label',
                  hintText: 'e.g. Lunch, Groceries, Salary',
                  helperText: 'Short tag shown in the list',
                  border: const OutlineInputBorder(
                    borderRadius: BorderRadius.all(Radius.circular(12)),
                  ),
                  filled: true,
                  prefixIcon: const Icon(Icons.label_outline_rounded),
                ),
              ),
              const SizedBox(height: 12),

              // Description field
              TextField(
                controller: _description,
                maxLines: 3,
                textCapitalization: TextCapitalization.sentences,
                decoration: InputDecoration(
                  labelText: 'Description',
                  hintText: "e.g. McDonald's double cheeseburger + fries",
                  helperText: 'Full details of this transaction',
                  border: const OutlineInputBorder(
                    borderRadius: BorderRadius.all(Radius.circular(12)),
                  ),
                  filled: true,
                  prefixIcon: const Padding(
                    padding: EdgeInsets.only(bottom: 48),
                    child: Icon(Icons.notes_rounded),
                  ),
                ),
              ),

              // Error
              if (_error != null) ...[
                const SizedBox(height: 12),
                Text(_error!,
                    style: tt.bodySmall?.copyWith(color: cs.error)),
              ],

              const SizedBox(height: 20),

              // Save button
              SizedBox(
                width: double.infinity,
                height: 50,
                child: FilledButton(
                  onPressed: _loading ? null : _save,
                  child: _loading
                      ? const SizedBox(
                          width: 20, height: 20,
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
  ),
);
  }
}

// ── Empty / error states ──────────────────────────────────────────────────────

class _EmptyView extends StatelessWidget {
  const _EmptyView();

  @override
  Widget build(BuildContext context) {
    return Center(
      child: Column(
        mainAxisAlignment: MainAxisAlignment.center,
        children: [
          Icon(Icons.receipt_long_outlined,
              size: 56, color: Theme.of(context).colorScheme.outline),
          const SizedBox(height: 16),
          Text('No transactions yet',
              style: Theme.of(context).textTheme.titleMedium),
          const SizedBox(height: 8),
          Text(
            'Every deposit, withdrawal and charge\nwill appear here.',
            style: Theme.of(context).textTheme.bodySmall?.copyWith(
                  color: Theme.of(context).colorScheme.outline,
                ),
            textAlign: TextAlign.center,
          ),
        ],
      ),
    );
  }
}

class _ErrorView extends StatelessWidget {
  const _ErrorView({required this.message});
  final String message;

  @override
  Widget build(BuildContext context) {
    return Center(
      child: Text(message,
          style: TextStyle(color: Theme.of(context).colorScheme.error)),
    );
  }
}
