import 'package:flutter/material.dart';
import '../../models/account_summary.dart';
import '../../theme/app_theme.dart';

class CategoryScreen extends StatelessWidget {
  const CategoryScreen({
    super.key,
    required this.label,
    required this.icon,
    required this.accounts,
    required this.allAccounts,
    required this.onSelectAccount,
  });

  final String             label;
  final IconData           icon;
  final List<AccountSummary> accounts;
  final List<AccountSummary> allAccounts;
  /// Called when user taps an account tile — navigates in the shell.
  final void Function(String accountId) onSelectAccount;

  @override
  Widget build(BuildContext context) {
    final isDebtCategory = accounts.isNotEmpty && accounts.first.isDebt;

    // Compute category totals
    double total = 0;
    for (final a in accounts) {
      total += double.tryParse(a.balance) ?? 0;
    }
    final sym = (accounts.firstOrNull?.currency == 'TWD') ? 'NT\$' : '\$';

    return Scaffold(
      backgroundColor: AppTheme.pageBg,
      appBar: AppBar(
        automaticallyImplyLeading: false,
        title: Row(
          children: [
            Container(
              width: 32, height: 32,
              decoration: BoxDecoration(
                color: isDebtCategory
                    ? AppTheme.red.withOpacity(0.15)
                    : AppTheme.purple.withOpacity(0.15),
                borderRadius: BorderRadius.circular(8),
              ),
              child: Icon(icon, size: 16,
                  color: isDebtCategory ? AppTheme.red : AppTheme.purpleLight),
            ),
            const SizedBox(width: 10),
            Text(label),
          ],
        ),
      ),
      body: ListView(
        padding: const EdgeInsets.all(20),
        children: [
          // ── Summary card ──────────────────────────────────────────────
          _SummaryCard(
            label:       label,
            accounts:    accounts,
            total:       total,
            symbol:      sym,
            isDebt:      isDebtCategory,
          ),
          const SizedBox(height: 20),

          // ── Section header ────────────────────────────────────────────
          Text(
            'Accounts (${accounts.length})',
            style: const TextStyle(
              color: AppTheme.textSecondary,
              fontSize: 11,
              fontWeight: FontWeight.w700,
              letterSpacing: 1.2,
            ),
          ),
          const SizedBox(height: 10),

          // ── Account tiles ─────────────────────────────────────────────
          ...accounts.map((a) => _AccountTile(
                account:     a,
                onTap:       () => onSelectAccount(a.accountId),
              )),
        ],
      ),
    );
  }
}

// ── Summary card ──────────────────────────────────────────────────────────────

class _SummaryCard extends StatelessWidget {
  const _SummaryCard({
    required this.label,
    required this.accounts,
    required this.total,
    required this.symbol,
    required this.isDebt,
  });

  final String             label;
  final List<AccountSummary> accounts;
  final double             total;
  final String             symbol;
  final bool               isDebt;

  @override
  Widget build(BuildContext context) {
    final color = isDebt ? AppTheme.red : AppTheme.green;
    final tt    = Theme.of(context).textTheme;

    return Container(
      decoration: AppTheme.glowCard(),
      padding: const EdgeInsets.all(20),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text(
            isDebt ? 'Total Outstanding' : 'Total Balance',
            style: const TextStyle(
                color: AppTheme.textSecondary,
                fontSize: 12,
                fontWeight: FontWeight.w500),
          ),
          const SizedBox(height: 6),
          Text(
            '$symbol ${_fmt(total)}',
            style: TextStyle(
              color: color,
              fontSize: 28,
              fontWeight: FontWeight.w700,
              letterSpacing: -0.5,
            ),
          ),
          const SizedBox(height: 16),
          Row(
            children: [
              _Pill(label: '${accounts.length} account${accounts.length != 1 ? 's' : ''}',
                    icon: Icons.account_balance_wallet_outlined),
              const SizedBox(width: 10),
              // Currency breakdown if mixed
              if (_hasMixedCurrencies)
                _Pill(
                  label: accounts.map((a) => a.currency).toSet().join(' · '),
                  icon: Icons.language_rounded,
                ),
            ],
          ),
        ],
      ),
    );
  }

  bool get _hasMixedCurrencies =>
      accounts.map((a) => a.currency).toSet().length > 1;

  String _fmt(double v) {
    final abs = v.abs();
    if (abs >= 1000000) return '${(abs / 1000000).toStringAsFixed(1)}M';
    if (abs >= 1000)    return '${(abs / 1000).toStringAsFixed(1)}k';
    return abs.toStringAsFixed(2);
  }
}

class _Pill extends StatelessWidget {
  const _Pill({required this.label, required this.icon});
  final String   label;
  final IconData icon;

  @override
  Widget build(BuildContext context) => Container(
        padding: const EdgeInsets.symmetric(horizontal: 10, vertical: 5),
        decoration: BoxDecoration(
          color: AppTheme.surface2,
          borderRadius: BorderRadius.circular(20),
          border: Border.all(color: AppTheme.border),
        ),
        child: Row(
          mainAxisSize: MainAxisSize.min,
          children: [
            Icon(icon, size: 12, color: AppTheme.textSecondary),
            const SizedBox(width: 5),
            Text(label,
                style: const TextStyle(
                    color: AppTheme.textSecondary,
                    fontSize: 11,
                    fontWeight: FontWeight.w500)),
          ],
        ),
      );
}

// ── Account tile ──────────────────────────────────────────────────────────────

class _AccountTile extends StatelessWidget {
  const _AccountTile({required this.account, required this.onTap});
  final AccountSummary account;
  final VoidCallback   onTap;

  @override
  Widget build(BuildContext context) {
    final sym         = account.currency == 'TWD' ? 'NT\$' : '\$';
    final balColor    = account.isDebt ? AppTheme.red : AppTheme.green;
    final accentColor = account.isDebt ? AppTheme.red : AppTheme.purple;

    return GestureDetector(
      onTap: onTap,
      child: Container(
        margin: const EdgeInsets.only(bottom: 12),
        decoration: AppTheme.glowCard(),
        padding: const EdgeInsets.symmetric(horizontal: 18, vertical: 16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            // ── Top row: icon + name/type + balance ────────────────────
            Row(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Container(
                  width: 46, height: 46,
                  decoration: BoxDecoration(
                    color: accentColor.withOpacity(0.15),
                    borderRadius: BorderRadius.circular(12),
                  ),
                  child: Icon(_iconFor(account.accountType),
                      size: 22,
                      color: account.isDebt
                          ? AppTheme.red
                          : AppTheme.purpleLight),
                ),
                const SizedBox(width: 14),

                Expanded(
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Text(account.accountName,
                          style: const TextStyle(
                              color: AppTheme.textPrimary,
                              fontSize: 15,
                              fontWeight: FontWeight.w600),
                          overflow: TextOverflow.ellipsis),
                      const SizedBox(height: 5),
                      Wrap(
                        spacing: 6,
                        runSpacing: 4,
                        children: [
                          _Tag(
                            label: account.typeLabel,
                            color: accentColor.withOpacity(0.18),
                            textColor: account.isDebt
                                ? AppTheme.red
                                : AppTheme.purpleLight,
                          ),
                          _Tag(
                            label: account.currency,
                            color: AppTheme.surface2,
                            textColor: AppTheme.textSecondary,
                          ),
                          if (account.isOverdue)
                            _Tag(
                              label: 'OVERDUE',
                              color: AppTheme.red.withOpacity(0.15),
                              textColor: AppTheme.red,
                            ),
                        ],
                      ),
                    ],
                  ),
                ),

                const SizedBox(width: 12),
                Column(
                  crossAxisAlignment: CrossAxisAlignment.end,
                  children: [
                    Text('$sym ${_fmt(account.balance)}',
                        style: TextStyle(
                            color: balColor,
                            fontSize: 16,
                            fontWeight: FontWeight.w700,
                            letterSpacing: -0.3)),
                    const SizedBox(height: 4),
                    Text(account.isDebt ? 'outstanding' : 'available',
                        style: const TextStyle(
                            color: AppTheme.textTertiary, fontSize: 11)),
                  ],
                ),
              ],
            ),

            // ── Divider ──────────────────────────────────────────────────
            const Padding(
              padding: EdgeInsets.symmetric(vertical: 12),
              child: Divider(height: 1, color: AppTheme.border),
            ),

            // ── Bottom row: account id chip + chevron ─────────────────
            Row(
              children: [
                _Tag(
                  icon: Icons.tag_rounded,
                  label: account.accountId.length > 8
                      ? '···${account.accountId.substring(account.accountId.length - 6)}'
                      : account.accountId,
                  color: AppTheme.surface2,
                  textColor: AppTheme.textTertiary,
                ),
                const Spacer(),
                const Icon(Icons.chevron_right_rounded,
                    color: AppTheme.border, size: 18),
              ],
            ),
          ],
        ),
      ),
    );
  }

  String _fmt(String raw) {
    final parts = raw.replaceAll('-', '').split('.');
    final i     = parts[0].replaceAllMapped(
        RegExp(r'(\d{1,3})(?=(\d{3})+(?!\d))'), (m) => '${m[1]},');
    return parts.length > 1 ? '$i.${parts[1]}' : i;
  }

  IconData _iconFor(String type) => switch (type) {
        'cash'            => Icons.account_balance_rounded,
        'physical_wallet' => Icons.wallet_rounded,
        'digital_wallet'  => Icons.contactless_rounded,
        'investment'      => Icons.show_chart_rounded,
        'credit_card'     => Icons.credit_card_rounded,
        'loan'            => Icons.request_quote_rounded,
        _                 => Icons.account_balance_wallet,
      };
}

// ── Tag chip ──────────────────────────────────────────────────────────────────

class _Tag extends StatelessWidget {
  const _Tag({
    required this.label,
    required this.color,
    required this.textColor,
    this.icon,
  });
  final String    label;
  final Color     color;
  final Color     textColor;
  final IconData? icon;

  @override
  Widget build(BuildContext context) => Container(
        padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 3),
        decoration: BoxDecoration(
          color: color,
          borderRadius: BorderRadius.circular(6),
          border: Border.all(color: textColor.withOpacity(0.2)),
        ),
        child: Row(
          mainAxisSize: MainAxisSize.min,
          children: [
            if (icon != null) ...[
              Icon(icon, size: 10, color: textColor),
              const SizedBox(width: 4),
            ],
            Text(label,
                style: TextStyle(
                    color: textColor,
                    fontSize: 10,
                    fontWeight: FontWeight.w600,
                    letterSpacing: 0.3)),
          ],
        ),
      );
}
