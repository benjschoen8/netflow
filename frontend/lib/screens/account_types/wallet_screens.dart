import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../core/providers.dart';
import '../../models/account_summary.dart';
import '../../shared/widgets/form_sheet.dart';
import '../../theme/app_theme.dart';
import '../transactions/transaction_sheets.dart';
import 'shared.dart';

// ── Physical Wallet ───────────────────────────────────────────────────────────

class PhysicalWalletScreen extends ConsumerWidget {
  const PhysicalWalletScreen({super.key, required this.account, this.allAccounts = const []});
  final AccountSummary       account;
  final List<AccountSummary> allAccounts;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final detailAsync = ref.watch(walletDetailProvider(account.accountId));
    final live        = ref.watch(accountsProvider).asData?.value ?? allAccounts;

    return DetailScaffold(
      account:   account,
      onRefresh: () => refreshDashboard(ref),
      overviewTab: detailAsync.when(
        loading: () => const Center(child: CircularProgressIndicator()),
        error:   (e, _) => Center(child: Text(e.toString())),
        data:    (d) => ListView(children: [
          BalanceHero(label: 'Cash on Hand', amount: d.balance,
              currency: d.currency, icon: Icons.wallet_rounded),
          SectionCard(title: 'Actions', children: [
            ActionButton(icon: Icons.add_rounded,        label: 'Add Cash',  color: AppTheme.green,
                onTap: () => _open(context, ref, DepositSheet(account: account))),
            const SizedBox(height: 8),
            ActionButton(icon: Icons.remove_rounded,     label: 'Spend Cash', color: AppTheme.amber,
                onTap: () => _open(context, ref, WithdrawSheet(account: account))),
            const SizedBox(height: 8),
            ActionButton(icon: Icons.swap_horiz_rounded, label: 'Transfer',   color: AppTheme.purple,
                onTap: () => _open(context, ref, TransferSheet(account: account, allAccounts: live))),
          ]),
          const SizedBox(height: 32),
        ]),
      ),
    );
  }

  Future<void> _open(BuildContext context, WidgetRef ref, Widget sheet) async {
    final result = await showFormSheet<bool>(context, sheet);
    if (result == true) refreshDashboard(ref);
  }
}

// ── Digital Wallet ────────────────────────────────────────────────────────────

class DigitalWalletScreen extends ConsumerWidget {
  const DigitalWalletScreen({super.key, required this.account, this.allAccounts = const []});
  final AccountSummary       account;
  final List<AccountSummary> allAccounts;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final detailAsync = ref.watch(digitalWalletDetailProvider(account.accountId));
    final live        = ref.watch(accountsProvider).asData?.value ?? allAccounts;

    return DetailScaffold(
      account:   account,
      onRefresh: () => refreshDashboard(ref),
      overviewTab: detailAsync.when(
        loading: () => const Center(child: CircularProgressIndicator()),
        error:   (e, _) => Center(child: Text(e.toString())),
        data:    (d) => ListView(children: [
          BalanceHero(label: 'Wallet Balance', amount: d.balance,
              currency: d.currency, icon: Icons.contactless_rounded),
          SectionCard(title: 'Provider', children: [
            InfoRow(label: 'Service',    value: d.provider,          icon: Icons.smartphone_rounded),
            InfoRow(label: 'Account ID', value: d.providerAccountId, icon: Icons.tag_rounded),
            InfoRow(label: 'Currency',   value: d.currency,          icon: Icons.language_rounded),
          ]),
          SectionCard(title: 'Actions', children: [
            ActionButton(icon: Icons.add_rounded,        label: 'Top Up',   color: AppTheme.green,
                onTap: () => _open(context, ref, DepositSheet(account: account))),
            const SizedBox(height: 8),
            ActionButton(icon: Icons.remove_rounded,     label: 'Spend',    color: AppTheme.amber,
                onTap: () => _open(context, ref, WithdrawSheet(account: account))),
            const SizedBox(height: 8),
            ActionButton(icon: Icons.swap_horiz_rounded, label: 'Transfer', color: AppTheme.purple,
                onTap: () => _open(context, ref, TransferSheet(account: account, allAccounts: live))),
          ]),
          const SizedBox(height: 32),
        ]),
      ),
    );
  }

  Future<void> _open(BuildContext context, WidgetRef ref, Widget sheet) async {
    final result = await showFormSheet<bool>(context, sheet);
    if (result == true) refreshDashboard(ref);
  }
}
