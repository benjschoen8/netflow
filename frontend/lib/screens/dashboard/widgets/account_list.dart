import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../../core/providers.dart';
import '../../../models/account_summary.dart';
import '../../../theme/app_theme.dart';

class AccountList extends ConsumerWidget {
  const AccountList({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final accountsAsync = ref.watch(accountsProvider);

    return accountsAsync.when(
      loading: () => const _AccountListSkeleton(),
      error: (e, _) => _AccountListError(message: e.toString()),
      data: (accounts) {
        if (accounts.isEmpty) return const SizedBox.shrink();
        return _AccountListBody(accounts: accounts);
      },
    );
  }
}

// ── Body ──────────────────────────────────────────────────────────────────────

class _AccountListBody extends StatelessWidget {
  const _AccountListBody({required this.accounts});
  final List<AccountSummary> accounts;

  @override
  Widget build(BuildContext context) {
    // Group by type
    final groups = <String, List<AccountSummary>>{};
    for (final a in accounts) {
      groups.putIfAbsent(a.typeLabel, () => []).add(a);
    }

    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: groups.entries.map((entry) {
        return _AccountGroup(label: entry.key, accounts: entry.value);
      }).toList(),
    );
  }
}

class _AccountGroup extends StatelessWidget {
  const _AccountGroup({required this.label, required this.accounts});
  final String label;
  final List<AccountSummary> accounts;

  @override
  Widget build(BuildContext context) {
    final tt = Theme.of(context).textTheme;
    final cs = Theme.of(context).colorScheme;

    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Padding(
          padding: const EdgeInsets.fromLTRB(20, 20, 20, 8),
          child: Text(
            label.toUpperCase(),
            style: tt.labelSmall?.copyWith(
              color: cs.outline,
              letterSpacing: 1.2,
              fontWeight: FontWeight.w600,
            ),
          ),
        ),
        ...accounts.map((a) => _AccountTile(account: a)),
      ],
    );
  }
}

class _AccountTile extends StatelessWidget {
  const _AccountTile({required this.account});
  final AccountSummary account;

  @override
  Widget build(BuildContext context) {
    final cs = Theme.of(context).colorScheme;
    final tt = Theme.of(context).textTheme;
    final isDebt = account.isDebt;
    final balanceColor =
        isDebt ? AppTheme.debtColor(context) : AppTheme.assetColor(context);

    return Padding(
      padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 3),
      child: Card(
        color: cs.surfaceContainerLow,
        child: ListTile(
          contentPadding:
              const EdgeInsets.symmetric(horizontal: 16, vertical: 4),
          leading: _AccountIcon(accountType: account.accountType),
          title: Text(
            account.accountName,
            style: tt.titleSmall?.copyWith(fontWeight: FontWeight.w600),
          ),
          subtitle: Row(
            children: [
              Text(account.currency,
                  style:
                      tt.labelSmall?.copyWith(color: cs.outline)),
              if (account.isOverdue) ...[
                const SizedBox(width: 6),
                Container(
                  padding:
                      const EdgeInsets.symmetric(horizontal: 6, vertical: 2),
                  decoration: BoxDecoration(
                    color: cs.errorContainer,
                    borderRadius: BorderRadius.circular(4),
                  ),
                  child: Text(
                    'OVERDUE',
                    style: tt.labelSmall?.copyWith(
                      color: cs.onErrorContainer,
                      fontWeight: FontWeight.w700,
                      fontSize: 9,
                    ),
                  ),
                ),
              ],
            ],
          ),
          trailing: Column(
            mainAxisAlignment: MainAxisAlignment.center,
            crossAxisAlignment: CrossAxisAlignment.end,
            children: [
              Text(
                _fmtBalance(account.balance, account.currency),
                style: tt.titleMedium?.copyWith(
                  color: balanceColor,
                  fontWeight: FontWeight.w700,
                ),
              ),
              Text(
                isDebt ? 'owed' : 'available',
                style: tt.labelSmall?.copyWith(color: cs.outline),
              ),
            ],
          ),
        ),
      ),
    );
  }

  String _fmtBalance(String raw, String currency) {
    final symbol = currency == 'TWD' ? 'NT\$' : '\$';
    final isNeg = raw.startsWith('-');
    final clean = raw.replaceAll('-', '');
    final parts = clean.split('.');
    final intPart = parts[0].replaceAllMapped(
      RegExp(r'(\d{1,3})(?=(\d{3})+(?!\d))'),
      (m) => '${m[1]},',
    );
    final decimals = parts.length > 1 ? '.${parts[1]}' : '';
    return '${isNeg ? '-' : ''}$symbol$intPart$decimals';
  }
}

class _AccountIcon extends StatelessWidget {
  const _AccountIcon({required this.accountType});
  final String accountType;

  @override
  Widget build(BuildContext context) {
    final cs = Theme.of(context).colorScheme;
    final (icon, bg) = switch (accountType) {
      'cash'            => (Icons.account_balance_rounded,    cs.primaryContainer),
      'physical_wallet' => (Icons.wallet_rounded,             cs.secondaryContainer),
      'digital_wallet'  => (Icons.contactless_rounded,        cs.tertiaryContainer),
      'investment'      => (Icons.show_chart_rounded,         cs.primaryContainer),
      'credit_card'     => (Icons.credit_card_rounded,        cs.errorContainer),
      'loan'            => (Icons.request_quote_rounded,      cs.errorContainer),
      _                 => (Icons.account_balance_wallet,     cs.surfaceContainerHighest),
    };

    return Container(
      width: 44,
      height: 44,
      decoration: BoxDecoration(color: bg, borderRadius: BorderRadius.circular(12)),
      child: Icon(icon, size: 22, color: cs.onSurface),
    );
  }
}

// ── States ────────────────────────────────────────────────────────────────────

class _AccountListSkeleton extends StatelessWidget {
  const _AccountListSkeleton();

  @override
  Widget build(BuildContext context) {
    return Column(
      children: List.generate(
        3,
        (i) => Padding(
          padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 4),
          child: Card(
            color: Theme.of(context).colorScheme.surfaceContainerLow,
            child: const ListTile(
              leading: _SkeletonBox(size: 44),
              title: _SkeletonBox(height: 14, width: 120),
              subtitle: _SkeletonBox(height: 10, width: 60),
              trailing: _SkeletonBox(height: 20, width: 80),
            ),
          ),
        ),
      ),
    );
  }
}

class _SkeletonBox extends StatelessWidget {
  const _SkeletonBox({this.size, this.height, this.width});
  final double? size;
  final double? height;
  final double? width;

  @override
  Widget build(BuildContext context) {
    return Container(
      width: size ?? width,
      height: size ?? height,
      decoration: BoxDecoration(
        color: Theme.of(context).colorScheme.surfaceContainerHighest,
        borderRadius: BorderRadius.circular(8),
      ),
    );
  }
}

class _AccountListError extends StatelessWidget {
  const _AccountListError({required this.message});
  final String message;

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.all(16),
      child: Text(
        'Could not load accounts: $message',
        style: TextStyle(color: Theme.of(context).colorScheme.error),
      ),
    );
  }
}
