import '../../theme/app_theme.dart';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../core/providers.dart';
import '../../models/account_summary.dart';
import '../../shared/widgets/form_sheet.dart';
import '../transactions/transaction_sheets.dart';
import 'shared.dart';


class LoanAccountScreen extends ConsumerWidget {
  const LoanAccountScreen({super.key, required this.account,
      required this.allAccounts});
  final AccountSummary       account;
  final List<AccountSummary> allAccounts;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final detailAsync = ref.watch(loanDetailProvider(account.accountId));

    return DetailScaffold(
      account: account,
      onRefresh: () => refreshDashboard(ref),
      overviewTab: detailAsync.when(
        loading: () => const Center(child: CircularProgressIndicator()),
        error:   (e, _) => Center(child: Text(e.toString())),
        data:    (d) => ListView(children: [
          // ── Outstanding hero ───────────────────────────────────────────
          BalanceHero(
            label: d.isSettled ? 'Loan Settled ✓' : 'Outstanding',
            amount: d.outstanding,
            currency: d.currency,
            icon: Icons.request_quote_rounded,
            isDebt: !d.isSettled,
          ),

          // ── Repayment progress ─────────────────────────────────────────
          if (!d.isSettled)
            Padding(
              padding: const EdgeInsets.symmetric(horizontal: 16),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Row(
                    mainAxisAlignment: MainAxisAlignment.spaceBetween,
                    children: [
                      Text('Repaid',
                          style: Theme.of(context).textTheme.labelMedium
                              ?.copyWith(color: Theme.of(context).colorScheme.outline)),
                      Text('${d.percentPaid}%',
                          style: Theme.of(context).textTheme.labelMedium?.copyWith(
                                color: Theme.of(context).colorScheme.primary,
                                fontWeight: FontWeight.w700,
                              )),
                    ],
                  ),
                  const SizedBox(height: 6),
                  ClipRRect(
                    borderRadius: BorderRadius.circular(4),
                    child: LinearProgressIndicator(
                      value: (d.percentPaidDouble / 100).clamp(0.0, 1.0),
                      minHeight: 8,
                      backgroundColor:
                          Theme.of(context).colorScheme.surfaceContainerHighest,
                    ),
                  ),
                  const SizedBox(height: 4),
                  Row(
                    mainAxisAlignment: MainAxisAlignment.spaceBetween,
                    children: [
                      Text('Paid: ${d.symbol}${_fmt(d.amountPaid)}',
                          style: Theme.of(context).textTheme.labelSmall
                              ?.copyWith(color: Theme.of(context).colorScheme.outline)),
                      Text('Principal: ${d.symbol}${_fmt(d.principal)}',
                          style: Theme.of(context).textTheme.labelSmall
                              ?.copyWith(color: Theme.of(context).colorScheme.outline)),
                    ],
                  ),
                  const SizedBox(height: 12),
                ],
              ),
            ),

          // ── Loan details ────────────────────────────────────────────────
          SectionCard(title: 'Loan Details', children: [
            InfoRow(label: 'Creditor',  value: d.creditor, icon: Icons.business_rounded),
            InfoRow(label: 'Bank',      value: d.bank,     icon: Icons.account_balance_rounded),
            if (d.accountNumber != null)
              InfoRow(label: 'Account No.', value: d.accountNumber!,  icon: Icons.tag_rounded),
            if (d.interestRate != null)
              InfoRow(label: 'Interest Rate',
                  value: '${d.interestRate!.toStringAsFixed(2)}% p.a.',
                  icon: Icons.percent_rounded),
            if (d.dueDay != null)
              InfoRow(label: 'Monthly Due', value: _ordinal(d.dueDay!),
                  icon: Icons.calendar_today_rounded),
            if (d.maturityDate != null)
              InfoRow(label: 'Maturity Date', value: d.maturityDate!,
                  icon: Icons.event_rounded),
            if (d.minimumPayment != null)
              InfoRow(
                label: 'Min. Payment',
                value: '${d.symbol}${_fmt(d.minimumPayment!)}',
                icon: Icons.payments_rounded,
                valueColor: d.isOverdue
                    ? Theme.of(context).colorScheme.error
                    : null,
              ),
            if (d.isOverdue)
              Padding(
                padding: const EdgeInsets.only(top: 8),
                child: Container(
                  padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 8),
                  decoration: BoxDecoration(
                    color: Theme.of(context).colorScheme.errorContainer,
                    borderRadius: BorderRadius.circular(8),
                  ),
                  child: Row(children: [
                    Icon(Icons.warning_amber_rounded,
                        color: Theme.of(context).colorScheme.onErrorContainer,
                        size: 18),
                    const SizedBox(width: 8),
                    Text('Payment overdue',
                        style: TextStyle(
                            color: Theme.of(context).colorScheme.onErrorContainer,
                            fontWeight: FontWeight.w600)),
                  ]),
                ),
              ),
          ]),

          // ── Actions ─────────────────────────────────────────────────────
          if (!d.isSettled)
            SectionCard(title: 'Actions', children: [
              FilledButton.tonalIcon(
                onPressed: () async {
                  final result = await showFormSheet<bool>(
                    context,
                    PaymentSheet(
                        debtAccount: account, allAccounts: allAccounts),
                  );
                  if (result == true) {
                    refreshDashboard(ref);
                    refreshDashboard(ref);
                  }
                },
                icon: const Icon(Icons.payment_rounded, color: AppTheme.green),
                label: const Text('Make Payment',
                    style: TextStyle(
                        color: AppTheme.green, fontWeight: FontWeight.w600)),
                style: FilledButton.styleFrom(
                  backgroundColor: AppTheme.green.withOpacity(0.1),
                  minimumSize: const Size(double.infinity, 48),
                ),
              ),
              const SizedBox(height: 8),
              OutlinedButton.icon(
                onPressed: () async {
                  await ref
                      .read(ledgerRepoProvider)
                      .accrueInterest(account.accountId);
                  refreshDashboard(ref);
                  refreshDashboard(ref);
                },
                icon: const Icon(Icons.percent_rounded),
                label: const Text('Accrue Interest'),
                style: OutlinedButton.styleFrom(
                    minimumSize: const Size(double.infinity, 44)),
              ),
            ]),

          const SizedBox(height: 32),
        ]),
      ),
    );
  }

  String _fmt(String raw) {
    final parts = raw.split('.');
    final i = parts[0].replaceAllMapped(
      RegExp(r'(\d{1,3})(?=(\d{3})+(?!\d))'), (m) => '${m[1]},');
    return parts.length > 1 ? '$i.${parts[1]}' : i;
  }

  String _ordinal(int day) {
    if (day >= 11 && day <= 13) return '${day}th';
    return switch (day % 10) {
      1 => '${day}st', 2 => '${day}nd', 3 => '${day}rd', _ => '${day}th',
    };
  }
}
