import '../../theme/app_theme.dart';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../core/providers.dart';
import '../../models/account_detail.dart';
import '../../models/account_summary.dart';
import '../../shared/widgets/form_sheet.dart';
import '../holdings/holding_sheets.dart';
import '../transactions/transaction_sheets.dart';
import 'shared.dart';

class InvestmentAccountScreen extends ConsumerWidget {
  const InvestmentAccountScreen({super.key, required this.account});
  final AccountSummary account;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final detailAsync = ref.watch(investmentDetailProvider(account.accountId));

    return DetailScaffold(
      account: account,
      onRefresh: () => refreshDashboard(ref),
      overviewTab: detailAsync.when(
        loading: () => const Center(child: CircularProgressIndicator()),
        error:   (e, _) => Center(child: Text(e.toString())),
        data:    (d) => RefreshIndicator(
          onRefresh: () async {
            refreshDashboard(ref);
          },
          child: ListView(children: [
            // ── Total value hero ─────────────────────────────────────────
            BalanceHero(
              label: 'Total Portfolio Value',
              amount: d.totalValue,
              currency: d.currency,
              icon: Icons.show_chart_rounded,
            ),

            // ── Breakdown ─────────────────────────────────────────────────
            SectionCard(title: 'Breakdown', children: [
              InfoRow(label: 'Cash Balance',    value: '${d.symbol}${_fmt(d.cashBalance)}',   icon: Icons.savings_rounded),
              InfoRow(label: 'Holdings Value',  value: '${d.symbol}${_fmt(d.holdingsValue)}', icon: Icons.trending_up_rounded),
              const Divider(),
              InfoRow(label: 'Total',           value: '${d.symbol}${_fmt(d.totalValue)}',    icon: Icons.account_balance_rounded),
            ]),

            // ── Account info ──────────────────────────────────────────────
            SectionCard(title: 'Account Details', children: [
              InfoRow(label: 'Brokerage',       value: d.bank,          icon: Icons.business_rounded),
              InfoRow(label: 'Account Number',  value: d.accountNumber, icon: Icons.tag_rounded),
              InfoRow(label: 'Currency',        value: d.currency,      icon: Icons.language_rounded),
            ]),

            // ── Holdings ─────────────────────────────────────────────────
            SectionCard(title: 'Holdings (${d.holdings.length})', children: [
              if (d.holdings.isEmpty)
                Padding(
                  padding: const EdgeInsets.symmetric(vertical: 8),
                  child: Text('No holdings yet.',
                      style: TextStyle(color: Theme.of(context).colorScheme.outline)),
                )
              else
                ...d.holdings.map((h) => _HoldingTile(
                      holding: h,
                      account: account,
                      ref: ref,
                    )),
              const SizedBox(height: 8),
              FilledButton.tonalIcon(
                onPressed: () async {
                  final result = await showFormSheet<bool>(
                      context, AddHoldingSheet(account: account));
                  if (result == true) {
                    refreshDashboard(ref);
                    refreshDashboard(ref);
                  }
                },
                icon: const Icon(Icons.add_chart_rounded),
                label: const Text('Add Holding'),
                style: FilledButton.styleFrom(
                    minimumSize: const Size(double.infinity, 44)),
              ),
            ]),

            // ── Cash actions ──────────────────────────────────────────────
            SectionCard(title: 'Cash Actions', children: [
              Row(children: [
                Expanded(child: _SmallAction(
                  icon: Icons.add_rounded, label: 'Deposit',
                  color: AppTheme.green,
                  onTap: () async {
                    final r = await showFormSheet<bool>(
                        context, DepositSheet(account: account));
                    if (r == true) refreshDashboard(ref);
                  },
                )),
                const SizedBox(width: 8),
                Expanded(child: _SmallAction(
                  icon: Icons.remove_rounded, label: 'Withdraw',
                  color: AppTheme.amber,
                  onTap: () async {
                    final r = await showFormSheet<bool>(
                        context, WithdrawSheet(account: account));
                    if (r == true) refreshDashboard(ref);
                  },
                )),
              ]),
            ]),
            const SizedBox(height: 32),
          ]),
        ),
      ),
    );
  }
}

class _HoldingTile extends StatelessWidget {
  const _HoldingTile({
    required this.holding,
    required this.account,
    required this.ref,
  });
  final HoldingDetail  holding;
  final AccountSummary account;
  final WidgetRef      ref;

  @override
  Widget build(BuildContext context) {
    final cs = Theme.of(context).colorScheme;
    final tt = Theme.of(context).textTheme;

    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 6),
      child: Row(
        children: [
          Container(
            width: 40, height: 40,
            decoration: BoxDecoration(
              color: cs.primaryContainer,
              borderRadius: BorderRadius.circular(10),
            ),
            child: Center(
              child: Text(
                holding.ticker.length > 4
                    ? holding.ticker.substring(0, 4)
                    : holding.ticker,
                style: tt.labelSmall?.copyWith(
                    fontWeight: FontWeight.w800, color: cs.onPrimaryContainer),
              ),
            ),
          ),
          const SizedBox(width: 12),
          Expanded(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text(holding.ticker,
                    style: tt.titleSmall?.copyWith(fontWeight: FontWeight.w700)),
                Text('${holding.quantity} @ ${holding.symbol}${_fmt(holding.unitPrice)}',
                    style: tt.bodySmall?.copyWith(color: cs.outline)),
              ],
            ),
          ),
          Column(
            crossAxisAlignment: CrossAxisAlignment.end,
            children: [
              Text('${holding.symbol}${_fmt(holding.marketValue)}',
                  style: tt.titleSmall?.copyWith(fontWeight: FontWeight.w700)),
              Text(holding.investmentType.toLowerCase(),
                  style: tt.labelSmall?.copyWith(color: cs.outline)),
            ],
          ),
          const SizedBox(width: 8),
          PopupMenuButton<String>(
            icon: Icon(Icons.more_vert_rounded, size: 18, color: cs.outline),
            onSelected: (action) async {
              if (action == 'update_price') {
                final r = await showFormSheet<bool>(
                  context,
                  UpdatePriceSheet(account: account, ticker: holding.ticker),
                );
                if (r == true) {
                  refreshDashboard(ref);
                  refreshDashboard(ref);
                }
              } else if (action == 'remove') {
                await ref.read(ledgerRepoProvider)
                    .removeHolding(account.accountId, holding.ticker);
                refreshDashboard(ref);
                refreshDashboard(ref);
              }
            },
            itemBuilder: (_) => [
              const PopupMenuItem(value: 'update_price',
                  child: Text('Update Price')),
              const PopupMenuItem(value: 'remove',
                  child: Text('Remove Holding')),
            ],
          ),
        ],
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

class _SmallAction extends StatelessWidget {
  const _SmallAction({required this.icon, required this.label,
      required this.color, required this.onTap});
  final IconData icon; final String label;
  final Color color; final VoidCallback onTap;

  @override
  Widget build(BuildContext context) => OutlinedButton.icon(
        onPressed: onTap,
        icon: Icon(icon, size: 16, color: color),
        label: Text(label,
            style: TextStyle(color: color, fontWeight: FontWeight.w600)),
        style: OutlinedButton.styleFrom(
          side: BorderSide(color: color.withOpacity(0.4)),
          minimumSize: const Size(0, 44),
        ),
      );
}

String _fmt(String raw) {
  final parts = raw.split('.');
  final i = parts[0].replaceAllMapped(
    RegExp(r'(\d{1,3})(?=(\d{3})+(?!\d))'), (m) => '${m[1]},');
  return parts.length > 1 ? '$i.${parts[1]}' : i;
}
