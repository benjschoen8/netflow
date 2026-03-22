import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../core/providers.dart';
import '../../models/account_summary.dart';
import '../../shared/widgets/form_sheet.dart';
import '../../theme/app_theme.dart';
import '../transactions/transaction_sheets.dart';
import 'shared.dart';

class CashAccountScreen extends ConsumerWidget {
  const CashAccountScreen({super.key, required this.account, this.allAccounts = const []});
  final AccountSummary       account;
  final List<AccountSummary> allAccounts;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final detailAsync = ref.watch(cashDetailProvider(account.accountId));
    final live        = ref.watch(accountsProvider).asData?.value ?? allAccounts;

    return DetailScaffold(
      account:   account,
      onRefresh: () => refreshDashboard(ref),
      overviewTab: detailAsync.when(
        loading: () => const Center(child: CircularProgressIndicator()),
        error:   (e, _) => Center(child: Text(e.toString())),
        data:    (d) => ListView(children: [
          BalanceHero(label: 'Available Balance', amount: d.balance,
              currency: d.currency, icon: Icons.account_balance_rounded),
          SectionCard(title: 'Account Details', children: [
            InfoRow(label: 'Bank',           value: d.bank,          icon: Icons.business_rounded),
            InfoRow(label: 'Account Number', value: d.accountNumber, icon: Icons.tag_rounded),
            InfoRow(label: 'Currency',       value: d.currency,      icon: Icons.language_rounded),
          ]),
          SectionCard(title: 'Actions', children: [
            ActionButton(icon: Icons.add_rounded,          label: 'Deposit',  color: AppTheme.green,
                onTap: () => _open(context, ref, DepositSheet(account: account))),
            const SizedBox(height: 8),
            ActionButton(icon: Icons.remove_rounded,       label: 'Withdraw', color: AppTheme.amber,
                onTap: () => _open(context, ref, WithdrawSheet(account: account))),
            const SizedBox(height: 8),
            ActionButton(icon: Icons.swap_horiz_rounded,   label: 'Transfer', color: AppTheme.purple,
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
