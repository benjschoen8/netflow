import '../../theme/app_theme.dart';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../core/providers.dart';
import '../../models/account_summary.dart';
import '../../shared/widgets/form_sheet.dart';
import '../transactions/transaction_sheets.dart';
import '../holdings/holding_sheets.dart';
import 'account_entries_screen.dart';
import 'edit_account_sheet.dart';
import '../credit_card/credit_card_detail_screen.dart';

class AccountDetailScreen extends ConsumerStatefulWidget {
  const AccountDetailScreen({super.key, required this.account});
  final AccountSummary account;

  @override
  ConsumerState<AccountDetailScreen> createState() =>
      _AccountDetailScreenState();
}

class _AccountDetailScreenState
    extends ConsumerState<AccountDetailScreen>
    with SingleTickerProviderStateMixin {
  late TabController _tabs;

  @override
  void initState() {
    super.initState();
    _tabs = TabController(length: 2, vsync: this);
  }

  @override
  void dispose() {
    _tabs.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final accountsAsync = ref.watch(accountsProvider);
    final live = accountsAsync.when(
      data: (list) => list.firstWhere(
        (a) => a.accountId == widget.account.accountId,
        orElse: () => widget.account,
      ),
      loading: () => widget.account,
      error:   (_, __) => widget.account,
    );
    final allAccounts = accountsAsync.asData?.value ?? [];

    return Scaffold(
      appBar: AppBar(
        title: Text(live.accountName),
        actions: [
          IconButton(
            icon: const Icon(Icons.edit_outlined),
            tooltip: 'Edit account',
            onPressed: () => _openEdit(context),
          ),
          IconButton(
            icon: const Icon(Icons.delete_outline_rounded),
            tooltip: 'Remove account',
            onPressed: () => _confirmRemove(context),
          ),
        ],
        bottom: TabBar(
          controller: _tabs,
          tabs: const [
            Tab(icon: Icon(Icons.dashboard_outlined), text: 'Overview'),
            Tab(icon: Icon(Icons.receipt_long_outlined), text: 'History'),
          ],
        ),
      ),
      body: TabBarView(
        controller: _tabs,
        children: [
          // ── Tab 0: Overview ─────────────────────────────────────────────
          RefreshIndicator(
            onRefresh: () async => refreshDashboard(ref),
            child: ListView(
              padding: const EdgeInsets.all(16),
              children: [
                _BalanceCard(account: live),
                const SizedBox(height: 20),
                _sectionHeader(context, 'Actions'),
                const SizedBox(height: 8),
                _ActionGrid(account: live, allAccounts: allAccounts, ref: ref),
                if (live.accountType == 'investment') ...[
                  const SizedBox(height: 20),
                  _sectionHeader(context, 'Holdings'),
                  const SizedBox(height: 8),
                  _HoldingsInfo(account: live, ref: ref),
                ],
              ],
            ),
          ),

          // ── Tab 1: History ───────────────────────────────────────────────
          AccountEntriesScreen(account: live),
        ],
      ),
    );
  }

  Widget _sectionHeader(BuildContext context, String title) => Text(
        title.toUpperCase(),
        style: Theme.of(context).textTheme.labelSmall?.copyWith(
              color: Theme.of(context).colorScheme.outline,
              letterSpacing: 1.2,
              fontWeight: FontWeight.w600,
            ),
      );

  Future<void> _openEdit(BuildContext context) async {
    final result = await showFormSheet<bool>(
      context,
      EditAccountSheet(account: widget.account),
    );
    if (result == true) refreshDashboard(ref);
  }

  Future<void> _confirmRemove(BuildContext context) async {
    final confirmed = await showDialog<bool>(
      context: context,
      builder: (_) => AlertDialog(
        title: const Text('Remove Account?'),
        content: Text(
            'This will permanently remove "${widget.account.accountName}" '
            'and all its history.'),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(context, false),
            child: const Text('Cancel'),
          ),
          FilledButton(
            style: FilledButton.styleFrom(
              backgroundColor: Theme.of(context).colorScheme.error,
            ),
            onPressed: () => Navigator.pop(context, true),
            child: const Text('Remove'),
          ),
        ],
      ),
    );
    if (confirmed != true || !context.mounted) return;
    try {
      await ref.read(ledgerRepoProvider).removeAccount(widget.account.accountId);
      refreshDashboard(ref);
      if (context.mounted) Navigator.pop(context);
    } catch (e) {
      if (context.mounted) {
        ScaffoldMessenger.of(context)
            .showSnackBar(SnackBar(content: Text(e.toString())));
      }
    }
  }
}

// ── Balance card ──────────────────────────────────────────────────────────────

class _BalanceCard extends StatelessWidget {
  const _BalanceCard({required this.account});
  final AccountSummary account;

  @override
  Widget build(BuildContext context) {
    final cs      = Theme.of(context).colorScheme;
    final tt      = Theme.of(context).textTheme;
    final isDebt  = account.isDebt;
    final color   = isDebt ? cs.error : cs.primary;
    final symbol  = account.currency == 'TWD' ? 'NT\$' : '\$';

    return Card(
      color: cs.surfaceContainerLow,
      child: Padding(
        padding: const EdgeInsets.all(20),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Row(
              children: [
                _typeIcon(context),
                const SizedBox(width: 12),
                Expanded(
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Text(account.typeLabel,
                          style: tt.labelMedium?.copyWith(color: cs.outline)),
                      Text(account.accountName,
                          style: tt.titleMedium
                              ?.copyWith(fontWeight: FontWeight.w700)),
                    ],
                  ),
                ),
                if (account.isOverdue)
                  Container(
                    padding: const EdgeInsets.symmetric(
                        horizontal: 8, vertical: 4),
                    decoration: BoxDecoration(
                      color: cs.errorContainer,
                      borderRadius: BorderRadius.circular(8),
                    ),
                    child: Text('OVERDUE',
                        style: tt.labelSmall?.copyWith(
                            color: cs.onErrorContainer,
                            fontWeight: FontWeight.w700)),
                  ),
              ],
            ),
            const SizedBox(height: 20),
            Text(isDebt ? 'Outstanding' : 'Balance',
                style: tt.labelSmall?.copyWith(color: cs.outline)),
            const SizedBox(height: 4),
            Text(
              '$symbol ${account.balance}',
              style: tt.headlineSmall?.copyWith(
                  color: color, fontWeight: FontWeight.w800),
            ),
          ],
        ),
      ),
    );
  }

  Widget _typeIcon(BuildContext context) {
    final cs = Theme.of(context).colorScheme;
    final (icon, bg) = switch (account.accountType) {
      'cash'            => (Icons.account_balance_rounded,  cs.primaryContainer),
      'physical_wallet' => (Icons.wallet_rounded,            cs.secondaryContainer),
      'digital_wallet'  => (Icons.contactless_rounded,       cs.tertiaryContainer),
      'investment'      => (Icons.show_chart_rounded,        cs.primaryContainer),
      'credit_card'     => (Icons.credit_card_rounded,       cs.errorContainer),
      'loan'            => (Icons.request_quote_rounded,     cs.errorContainer),
      _                 => (Icons.account_balance_wallet,    cs.surfaceContainerHighest),
    };
    return Container(
      width: 48, height: 48,
      decoration: BoxDecoration(color: bg, borderRadius: BorderRadius.circular(14)),
      child: Icon(icon, size: 24, color: cs.onSurface),
    );
  }
}

// ── Action grid ───────────────────────────────────────────────────────────────

class _ActionGrid extends StatelessWidget {
  const _ActionGrid({
    required this.account,
    required this.allAccounts,
    required this.ref,
  });
  final AccountSummary account;
  final List<AccountSummary> allAccounts;
  final WidgetRef ref;

  @override
  Widget build(BuildContext context) {
    final actions = _actionsFor(context);
    return GridView.count(
      crossAxisCount: 2,
      shrinkWrap: true,
      physics: const NeverScrollableScrollPhysics(),
      mainAxisSpacing: 8,
      crossAxisSpacing: 8,
      childAspectRatio: 2.4,
      children: actions,
    );
  }

  List<Widget> _actionsFor(BuildContext context) {
    switch (account.accountType) {
      case 'cash':
      case 'physical_wallet':
      case 'digital_wallet':
        return [
          _ActionButton(icon: Icons.add_rounded,    label: 'Deposit',  color: AppTheme.green,
              onTap: () => _open(context, DepositSheet(account: account))),
          _ActionButton(icon: Icons.remove_rounded,  label: 'Withdraw', color: AppTheme.amber,
              onTap: () => _open(context, WithdrawSheet(account: account))),
        ];
      case 'investment':
        return [
          _ActionButton(icon: Icons.add_rounded,         label: 'Deposit Cash',   color: AppTheme.green,
              onTap: () => _open(context, DepositSheet(account: account))),
          _ActionButton(icon: Icons.remove_rounded,       label: 'Withdraw Cash',  color: AppTheme.amber,
              onTap: () => _open(context, WithdrawSheet(account: account))),
          _ActionButton(icon: Icons.add_chart_rounded,    label: 'Add Holding',    color: AppTheme.purple,
              onTap: () => _open(context, AddHoldingSheet(account: account))),
        ];
      case 'credit_card':
        return [
          _ActionButton(icon: Icons.shopping_bag_outlined,  label: 'Charge',           color: AppTheme.amber,
              onTap: () => _open(context, ChargeSheet(account: account))),
          _ActionButton(icon: Icons.payment_rounded,         label: 'Pay',              color: AppTheme.green,
              onTap: () => _open(context, PaymentSheet(debtAccount: account, allAccounts: allAccounts))),
          _ActionButton(icon: Icons.receipt_long_rounded,    label: 'Close Statement',  color: AppTheme.purple,
              onTap: () => _open(context, CloseStatementSheet(account: account))),
          _ActionButton(icon: Icons.credit_score_rounded,    label: 'Grant Limit',      color: AppTheme.purpleLight,
              onTap: () => _open(context, GrantLimitSheet(account: account))),
          _ActionButton(icon: Icons.cancel_outlined,         label: 'Revoke Limit',     color: AppTheme.red,
              onTap: () => _revokeLimit(context)),
          _ActionButton(icon: Icons.percent_rounded,         label: 'Accrue Interest',  color: AppTheme.textSecondary,
              onTap: () => _accrueInterest(context)),
        ];
      case 'loan':
        return [
          _ActionButton(icon: Icons.payment_rounded,  label: 'Make Payment',    color: AppTheme.green,
              onTap: () => _open(context, PaymentSheet(debtAccount: account, allAccounts: allAccounts))),
          _ActionButton(icon: Icons.percent_rounded,   label: 'Accrue Interest', color: AppTheme.textSecondary,
              onTap: () => _accrueInterest(context)),
        ];
      default:
        return [];
    }
  }

  Future<void> _open(BuildContext context, Widget sheet) async {
    final result = await showFormSheet<bool>(context, sheet);
    if (result == true) refreshDashboard(ref);
  }

  Future<void> _revokeLimit(BuildContext context) async {
    try {
      await ref.read(ledgerRepoProvider).revokeTemporaryLimit(account.accountId);
      refreshDashboard(ref);
      if (context.mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
            const SnackBar(content: Text('Temporary limit revoked')));
      }
    } catch (e) {
      if (context.mounted) {
        ScaffoldMessenger.of(context)
            .showSnackBar(SnackBar(content: Text(e.toString())));
      }
    }
  }

  Future<void> _accrueInterest(BuildContext context) async {
    try {
      await ref.read(ledgerRepoProvider).accrueInterest(account.accountId);
      refreshDashboard(ref);
      if (context.mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
            const SnackBar(content: Text('Interest accrued')));
      }
    } catch (e) {
      if (context.mounted) {
        ScaffoldMessenger.of(context)
            .showSnackBar(SnackBar(content: Text(e.toString())));
      }
    }
  }
}

class _ActionButton extends StatelessWidget {
  const _ActionButton({
    required this.icon,
    required this.label,
    required this.color,
    required this.onTap,
  });
  final IconData icon;
  final String label;
  final Color color;
  final VoidCallback onTap;

  @override
  Widget build(BuildContext context) {
    return InkWell(
      borderRadius: BorderRadius.circular(12),
      onTap: onTap,
      child: Container(
        padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 10),
        decoration: BoxDecoration(
          color: color.withOpacity(0.1),
          borderRadius: BorderRadius.circular(12),
          border: Border.all(color: color.withOpacity(0.2)),
        ),
        child: Row(
          children: [
            Icon(icon, size: 18, color: color),
            const SizedBox(width: 8),
            Flexible(
              child: Text(
                label,
                style: Theme.of(context).textTheme.labelMedium?.copyWith(
                      color: color, fontWeight: FontWeight.w600),
                overflow: TextOverflow.ellipsis,
              ),
            ),
          ],
        ),
      ),
    );
  }
}

class _HoldingsInfo extends StatelessWidget {
  const _HoldingsInfo({required this.account, required this.ref});
  final AccountSummary account;
  final WidgetRef ref;

  @override
  Widget build(BuildContext context) {
    final cs = Theme.of(context).colorScheme;
    return Card(
      color: cs.surfaceContainerLow,
      child: Padding(
        padding: const EdgeInsets.all(16),
        child: Row(
          children: [
            Icon(Icons.info_outline_rounded, size: 18, color: cs.outline),
            const SizedBox(width: 8),
            Expanded(
              child: Text(
                'Use "Add Holding" above to record stock/ETF/crypto positions. '
                'Tap a holding entry in the History tab to add a note.',
                style: Theme.of(context)
                    .textTheme
                    .bodySmall
                    ?.copyWith(color: cs.outline),
              ),
            ),
          ],
        ),
      ),
    );
  }
}
