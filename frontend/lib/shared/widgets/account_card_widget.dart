/// Flutter translation of AccountCard.tsx
///
/// Dark card with purple-glow border, coloured initial avatar,
/// account name, balance stat, and an action button.
/// Maps: sales → balance, revenue → balance label, "Manage" → "Details".
library;

import 'package:flutter/material.dart';
import '../../theme/app_theme.dart';
import '../../models/account_summary.dart';

class AccountCardWidget extends StatelessWidget {
  const AccountCardWidget({
    super.key,
    required this.account,
    required this.onTap,
    this.avatarColor,
  });

  final AccountSummary account;
  final VoidCallback onTap;
  final Color? avatarColor;

  @override
  Widget build(BuildContext context) {
    final color  = avatarColor ?? _colorForType(account.accountType);
    final symbol = account.currency == 'TWD' ? 'NT\$' : '\$';
    final balColor = account.isDebt ? AppTheme.red : AppTheme.green;

    return Container(
      decoration: AppTheme.glowCard(),
      padding: const EdgeInsets.all(20),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          // ── Avatar (matches AccountCard.tsx top-left avatar) ──────────
          _Avatar(initial: account.accountName[0].toUpperCase(), color: color),
          const SizedBox(height: 16),

          // ── Account name ──────────────────────────────────────────────
          Text(
            account.accountName,
            style: const TextStyle(
              color: AppTheme.textPrimary,
              fontSize: 16,
              fontWeight: FontWeight.w600,
            ),
            maxLines: 1,
            overflow: TextOverflow.ellipsis,
          ),
          const SizedBox(height: 16),

          // ── Stats (maps sales+revenue → type+balance) ─────────────────
          _Stat(
            label: account.typeLabel,
            value: account.accountType,
            valueColor: AppTheme.textSecondary,
            isSmall: true,
          ),
          const SizedBox(height: 10),
          _Stat(
            label: account.isDebt ? 'Outstanding' : 'Balance',
            value: '$symbol ${_fmt(account.balance)}',
            valueColor: balColor,
          ),
          const SizedBox(height: 16),

          // ── "Manage" button (outline, purple) ─────────────────────────
          SizedBox(
            width: double.infinity,
            child: OutlinedButton(
              onPressed: onTap,
              style: OutlinedButton.styleFrom(
                foregroundColor: AppTheme.purpleLight,
                side: const BorderSide(color: AppTheme.purpleBorder),
                shape: RoundedRectangleBorder(
                    borderRadius: BorderRadius.circular(20)),
                padding: const EdgeInsets.symmetric(vertical: 10),
              ),
              child: const Text('Details',
                  style: TextStyle(
                      fontWeight: FontWeight.w500, fontSize: 14)),
            ),
          ),
        ],
      ),
    );
  }

  String _fmt(String raw) {
    final parts = raw.replaceAll('-', '').split('.');
    final int   = parts[0].replaceAllMapped(
        RegExp(r'(\d{1,3})(?=(\d{3})+(?!\d))'), (m) => '${m[1]},');
    return parts.length > 1 ? '$int.${parts[1]}' : int;
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

// ── Sub-widgets ────────────────────────────────────────────────────────────────

class _Avatar extends StatelessWidget {
  const _Avatar({required this.initial, required this.color});
  final String initial;
  final Color  color;

  @override
  Widget build(BuildContext context) => Container(
        width: 48,
        height: 48,
        decoration: BoxDecoration(
          color: color,
          borderRadius: BorderRadius.circular(12),
        ),
        alignment: Alignment.center,
        child: Text(
          initial,
          style: const TextStyle(
            color: Colors.white,
            fontWeight: FontWeight.w700,
            fontSize: 20,
          ),
        ),
      );
}

class _Stat extends StatelessWidget {
  const _Stat({
    required this.label,
    required this.value,
    this.valueColor,
    this.isSmall = false,
  });
  final String label;
  final String value;
  final Color? valueColor;
  final bool   isSmall;

  @override
  Widget build(BuildContext context) => Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text(label,
              style: const TextStyle(
                  color: AppTheme.textSecondary,
                  fontSize: 13,
                  fontWeight: FontWeight.w500)),
          const SizedBox(height: 2),
          Text(value,
              style: TextStyle(
                color: valueColor ?? AppTheme.purpleValue,
                fontSize: isSmall ? 13 : 20,
                fontWeight: FontWeight.w700,
              )),
        ],
      );
}
