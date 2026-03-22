/// Flutter translation of the AccountCard.tsx grid layout.
/// Cards are displayed in a 2-column grid on wide screens, single column on narrow.
library;

import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../../core/providers.dart';
import '../../../models/account_summary.dart';
import '../../../theme/app_theme.dart';
import '../../../shared/widgets/account_card_widget.dart';
import '../../account_types/cash_screen.dart';
import '../../account_types/wallet_screens.dart';
import '../../account_types/investment_screen.dart';
import '../../account_types/loan_screen.dart';
import '../../credit_card/credit_card_detail_screen.dart';

class AccountList extends ConsumerWidget {
  const AccountList({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final accountsAsync = ref.watch(accountsProvider);
    return accountsAsync.when(
      loading: () => const _Skeleton(),
      error:   (e, _) => _Error(message: e.toString()),
      data:    (accounts) => accounts.isEmpty
          ? const SizedBox.shrink()
          : _Body(accounts: accounts),
    );
  }
}

class _Body extends ConsumerWidget {
  const _Body({required this.accounts});
  final List<AccountSummary> accounts;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final allAccounts = accounts;
    // Group by category matching AccountLauncherPage sections
    final groups = <String, List<AccountSummary>>{};
    for (final a in accounts) {
      groups.putIfAbsent(a.typeLabel, () => []).add(a);
    }

    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: groups.entries.map((e) => _Group(
        label: e.key,
        accounts: e.value,
        allAccounts: allAccounts,
      )).toList(),
    );
  }
}

class _Group extends ConsumerWidget {
  const _Group({
    required this.label,
    required this.accounts,
    required this.allAccounts,
  });
  final String label;
  final List<AccountSummary> accounts;
  final List<AccountSummary> allAccounts;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final isWide = MediaQuery.of(context).size.width > 800;
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        // Section label (matches AccountLauncherPage section headers)
        Padding(
          padding: const EdgeInsets.fromLTRB(16, 20, 16, 10),
          child: Text(
            label.toUpperCase(),
            style: const TextStyle(
              color: AppTheme.textSecondary,
              fontSize: 11,
              fontWeight: FontWeight.w700,
              letterSpacing: 1.2,
            ),
          ),
        ),

        // Card grid (2-col on wide, 1-col on narrow)
        Padding(
          padding: const EdgeInsets.symmetric(horizontal: 16),
          child: isWide
              ? _Grid(accounts: accounts, allAccounts: allAccounts)
              : _List(accounts: accounts, allAccounts: allAccounts),
        ),
      ],
    );
  }
}

// ── Wide: 2-column AccountCard grid (matches AccountCard.tsx grid) ─────────────

class _Grid extends ConsumerWidget {
  const _Grid({required this.accounts, required this.allAccounts});
  final List<AccountSummary> accounts;
  final List<AccountSummary> allAccounts;

  @override
  Widget build(BuildContext context, WidgetRef ref) => Wrap(
        spacing: 12,
        runSpacing: 12,
        children: accounts
            .map((a) => SizedBox(
                  width: 220,
                  child: AccountCardWidget(
                    account:  a,
                    onTap:    () => _navigate(context, ref, a, allAccounts),
                  ),
                ))
            .toList(),
      );
}

// ── Narrow: compact list rows (matches AccountLauncherPage profile rows) ────────

class _List extends ConsumerWidget {
  const _List({required this.accounts, required this.allAccounts});
  final List<AccountSummary> accounts;
  final List<AccountSummary> allAccounts;

  @override
  Widget build(BuildContext context, WidgetRef ref) => Column(
        children: accounts
            .map((a) => _CompactRow(
                  account:     a,
                  allAccounts: allAccounts,
                ))
            .toList(),
      );
}

class _CompactRow extends ConsumerWidget {
  const _CompactRow({required this.account, required this.allAccounts});
  final AccountSummary account;
  final List<AccountSummary> allAccounts;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final symbol    = account.currency == 'TWD' ? 'NT\$' : '\$';
    final balColor  = account.isDebt ? AppTheme.red : AppTheme.green;
    final typeColor = _colorForType(account.accountType);

    return Container(
      margin: const EdgeInsets.only(bottom: 8),
      decoration: AppTheme.glowCard(),
      padding: const EdgeInsets.all(16),
      child: Row(
        children: [
          // Avatar
          Container(
            width: 44, height: 44,
            decoration: BoxDecoration(
                color: typeColor, borderRadius: BorderRadius.circular(11)),
            alignment: Alignment.center,
            child: Text(
              account.accountName[0].toUpperCase(),
              style: const TextStyle(
                  color: Colors.white,
                  fontWeight: FontWeight.w700,
                  fontSize: 18),
            ),
          ),
          const SizedBox(width: 14),

          // Name + type
          Expanded(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text(account.accountName,
                    style: const TextStyle(
                        color: AppTheme.textPrimary,
                        fontSize: 15,
                        fontWeight: FontWeight.w600)),
                const SizedBox(height: 2),
                Row(children: [
                  Text(account.typeLabel,
                      style: const TextStyle(
                          color: AppTheme.textTertiary, fontSize: 12)),
                  if (account.isOverdue) ...[
                    const SizedBox(width: 8),
                    _OverdueBadge(),
                  ],
                ]),
              ],
            ),
          ),

          // Balance
          Column(
            crossAxisAlignment: CrossAxisAlignment.end,
            children: [
              Text('$symbol ${_fmt(account.balance)}',
                  style: TextStyle(
                      color: balColor,
                      fontSize: 16,
                      fontWeight: FontWeight.w700)),
              Text(account.isDebt ? 'owed' : 'available',
                  style: const TextStyle(
                      color: AppTheme.textTertiary, fontSize: 11)),
            ],
          ),
          const SizedBox(width: 8),

          // Details button (matches "Manage" button)
          OutlinedButton(
            onPressed: () => _navigate(context, ref, account, allAccounts),
            style: OutlinedButton.styleFrom(
              foregroundColor: AppTheme.purpleLight,
              side: const BorderSide(color: AppTheme.purpleBorder),
              shape: RoundedRectangleBorder(
                  borderRadius: BorderRadius.circular(10)),
              padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 8),
              minimumSize: Size.zero,
              tapTargetSize: MaterialTapTargetSize.shrinkWrap,
            ),
            child: const Text('Details',
                style: TextStyle(fontSize: 12, fontWeight: FontWeight.w500)),
          ),
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

  Color _colorForType(String type) => switch (type) {
        'cash'            => const Color(0xFF8B5CF6),
        'physical_wallet' => const Color(0xFF06B6D4),
        'digital_wallet'  => const Color(0xFF22D3EE),
        'investment'      => const Color(0xFF2AF07A),
        'credit_card'     => const Color(0xFFFF5C8A),
        'loan'            => const Color(0xFFFBBF24),
        _                 => const Color(0xFF8B5CF6),
      };
}

// ── Overdue badge ─────────────────────────────────────────────────────────────

class _OverdueBadge extends StatelessWidget {
  @override
  Widget build(BuildContext context) => Container(
        padding: const EdgeInsets.symmetric(horizontal: 6, vertical: 2),
        decoration: BoxDecoration(
            color: AppTheme.red.withOpacity(0.15),
            borderRadius: BorderRadius.circular(4),
            border: Border.all(color: AppTheme.red.withOpacity(0.4))),
        child: const Text('OVERDUE',
            style: TextStyle(
                color: AppTheme.red,
                fontWeight: FontWeight.w700,
                fontSize: 9,
                letterSpacing: 0.5)),
      );
}

// ── Skeleton / Error ──────────────────────────────────────────────────────────

class _Skeleton extends StatelessWidget {
  const _Skeleton();
  @override
  Widget build(BuildContext context) => Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          children: List.generate(
            3,
            (_) => Container(
              margin: const EdgeInsets.only(bottom: 8),
              height: 74,
              decoration: AppTheme.glowCard(),
            ),
          ),
        ),
      );
}

class _Error extends StatelessWidget {
  const _Error({required this.message});
  final String message;
  @override
  Widget build(BuildContext context) => Padding(
        padding: const EdgeInsets.all(16),
        child: Text('Could not load accounts: $message',
            style: const TextStyle(color: AppTheme.red, fontSize: 13)),
      );
}

// ── Navigation helper (shared) ────────────────────────────────────────────────

void _navigate(BuildContext context, WidgetRef ref,
    AccountSummary account, List<AccountSummary> allAccounts) {
  final Widget screen = switch (account.accountType) {
    'cash'            => CashAccountScreen(account: account, allAccounts: allAccounts),
    'physical_wallet' => PhysicalWalletScreen(account: account, allAccounts: allAccounts),
    'digital_wallet'  => DigitalWalletScreen(account: account, allAccounts: allAccounts),
    'investment'      => InvestmentAccountScreen(account: account),
    'loan'            => LoanAccountScreen(account: account, allAccounts: allAccounts),
    'credit_card'     => CreditCardDetailScreen(account: account, allAccounts: allAccounts),
    _                 => CashAccountScreen(account: account),
  };
  Navigator.push(context, MaterialPageRoute(builder: (_) => screen));
}
