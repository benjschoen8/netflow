import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../../core/providers.dart';
import '../../../theme/app_theme.dart';
import 'net_worth_chart.dart';

class NetWorthCard extends ConsumerWidget {
  const NetWorthCard({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final nwAsync = ref.watch(netWorthProvider);

    return nwAsync.when(
      loading: () => _skeleton(),
      error:   (e, _) => _error(context, e.toString()),
      data:    (results) {
        if (results.isEmpty) return _empty(context);
        return Column(
          children: results.map((r) => NetWorthChart(
            totalAssets: r.totalAssets,
            totalDebts:  r.totalDebts,
            netWorth:    r.netWorth,
            currency:    r.currency,
            isDeficit:   r.isDeficit,
          )).toList(),
        );
      },
    );
  }

  Widget _skeleton() => Container(
        margin: const EdgeInsets.symmetric(horizontal: 16, vertical: 6),
        decoration: AppTheme.glowCard(),
        padding: const EdgeInsets.all(20),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            _shimmer(140, 16),
            const SizedBox(height: 12),
            _shimmer(200, 32),
            const SizedBox(height: 20),
            _shimmer(double.infinity, 180),
          ],
        ),
      );

  Widget _shimmer(double width, double height) => Container(
        width: width,
        height: height,
        decoration: BoxDecoration(
          color: AppTheme.surface2,
          borderRadius: BorderRadius.circular(8),
        ),
      );

  Widget _error(BuildContext context, String message) => Container(
        margin: const EdgeInsets.symmetric(horizontal: 16, vertical: 6),
        decoration: AppTheme.glowCard(color: const Color(0xFF1A0A0A)),
        padding: const EdgeInsets.all(16),
        child: Row(
          children: [
            const Icon(Icons.error_outline, color: AppTheme.red, size: 18),
            const SizedBox(width: 12),
            Expanded(
              child: Text(message,
                  style: const TextStyle(
                      color: AppTheme.textSecondary, fontSize: 13)),
            ),
          ],
        ),
      );

  Widget _empty(BuildContext context) => Container(
        margin: const EdgeInsets.symmetric(horizontal: 16, vertical: 6),
        decoration: AppTheme.glowCard(),
        padding: const EdgeInsets.all(24),
        child: const Column(
          children: [
            Icon(Icons.account_balance_wallet_outlined,
                size: 40, color: AppTheme.textSecondary),
            SizedBox(height: 12),
            Text('No accounts yet',
                style: TextStyle(
                    color: AppTheme.textPrimary,
                    fontSize: 16,
                    fontWeight: FontWeight.w600)),
            SizedBox(height: 4),
            Text('Add an account to see your net worth.',
                style: TextStyle(color: AppTheme.textSecondary, fontSize: 13),
                textAlign: TextAlign.center),
          ],
        ),
      );
}
