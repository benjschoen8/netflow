import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../../core/providers.dart';
import '../../../theme/app_theme.dart';

class NetWorthCard extends ConsumerWidget {
  const NetWorthCard({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final nwAsync = ref.watch(netWorthProvider);
    final cs = Theme.of(context).colorScheme;
    final tt = Theme.of(context).textTheme;

    return nwAsync.when(
      loading: () => const _NetWorthSkeleton(),
      error: (e, _) => _NetWorthError(message: e.toString()),
      data: (results) {
        if (results.isEmpty) {
          return _NetWorthEmpty();
        }
        return Column(
          children: results.map((r) => _CurrencyBlock(result: r)).toList(),
        );
      },
    );
  }
}

// ── Single currency block ─────────────────────────────────────────────────────

class _CurrencyBlock extends StatelessWidget {
  const _CurrencyBlock({required this.result});
  final dynamic result; // NetWorthResult

  @override
  Widget build(BuildContext context) {
    final cs = Theme.of(context).colorScheme;
    final tt = Theme.of(context).textTheme;
    final isDeficit = result.isDeficit as bool;

    return Card(
      color: cs.surfaceContainerLow,
      margin: const EdgeInsets.symmetric(horizontal: 16, vertical: 6),
      child: Padding(
        padding: const EdgeInsets.all(20),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            // Header row
            Row(
              mainAxisAlignment: MainAxisAlignment.spaceBetween,
              children: [
                Text(
                  'Net Worth · ${result.currency}',
                  style: tt.labelLarge?.copyWith(color: cs.outline),
                ),
                _Badge(isDeficit: isDeficit),
              ],
            ),
            const SizedBox(height: 8),

            // Big net worth number
            Text(
              _fmt(result.netWorth as String, result.currency as String),
              style: tt.displaySmall?.copyWith(
                fontWeight: FontWeight.w700,
                color: isDeficit
                    ? AppTheme.debtColor(context)
                    : AppTheme.assetColor(context),
              ),
            ),
            const SizedBox(height: 16),

            // Assets / Debts row
            Row(
              children: [
                Expanded(
                  child: _StatTile(
                    label: 'Assets',
                    value: _fmt(result.totalAssets as String, result.currency as String),
                    color: AppTheme.assetColor(context),
                    icon: Icons.trending_up_rounded,
                  ),
                ),
                const SizedBox(width: 12),
                Expanded(
                  child: _StatTile(
                    label: 'Debts',
                    value: _fmt(result.totalDebts as String, result.currency as String),
                    color: AppTheme.debtColor(context),
                    icon: Icons.trending_down_rounded,
                  ),
                ),
              ],
            ),
          ],
        ),
      ),
    );
  }

  String _fmt(String raw, String currency) {
    final symbol = currency == 'TWD' ? 'NT\$' : '\$';
    // Insert comma separators
    final parts = raw.replaceAll('-', '').split('.');
    final intPart = parts[0].replaceAllMapped(
      RegExp(r'(\d{1,3})(?=(\d{3})+(?!\d))'),
      (m) => '${m[1]},',
    );
    final formatted = parts.length > 1 ? '$intPart.${parts[1]}' : intPart;
    final sign = raw.startsWith('-') ? '-' : '';
    return '$sign$symbol$formatted';
  }
}

class _StatTile extends StatelessWidget {
  const _StatTile({
    required this.label,
    required this.value,
    required this.color,
    required this.icon,
  });
  final String label;
  final String value;
  final Color color;
  final IconData icon;

  @override
  Widget build(BuildContext context) {
    final cs = Theme.of(context).colorScheme;
    final tt = Theme.of(context).textTheme;
    return Container(
      padding: const EdgeInsets.all(12),
      decoration: BoxDecoration(
        color: color.withOpacity(0.08),
        borderRadius: BorderRadius.circular(12),
      ),
      child: Row(
        children: [
          Icon(icon, size: 18, color: color),
          const SizedBox(width: 6),
          Expanded(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text(label,
                    style: tt.labelSmall?.copyWith(color: cs.outline)),
                const SizedBox(height: 2),
                Text(value,
                    style: tt.titleSmall
                        ?.copyWith(color: color, fontWeight: FontWeight.w600),
                    overflow: TextOverflow.ellipsis),
              ],
            ),
          ),
        ],
      ),
    );
  }
}

class _Badge extends StatelessWidget {
  const _Badge({required this.isDeficit});
  final bool isDeficit;

  @override
  Widget build(BuildContext context) {
    final color = isDeficit
        ? AppTheme.debtColor(context)
        : AppTheme.assetColor(context);
    final label = isDeficit ? 'Deficit' : 'Surplus';
    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 10, vertical: 4),
      decoration: BoxDecoration(
        color: color.withOpacity(0.12),
        borderRadius: BorderRadius.circular(20),
      ),
      child: Text(
        label,
        style: Theme.of(context)
            .textTheme
            .labelSmall
            ?.copyWith(color: color, fontWeight: FontWeight.w600),
      ),
    );
  }
}

// ── States ────────────────────────────────────────────────────────────────────

class _NetWorthSkeleton extends StatelessWidget {
  const _NetWorthSkeleton();

  @override
  Widget build(BuildContext context) {
    final cs = Theme.of(context).colorScheme;
    return Card(
      color: cs.surfaceContainerLow,
      margin: const EdgeInsets.symmetric(horizontal: 16, vertical: 6),
      child: const Padding(
        padding: EdgeInsets.all(20),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            _ShimmerBox(width: 140, height: 16),
            SizedBox(height: 12),
            _ShimmerBox(width: 220, height: 40),
            SizedBox(height: 16),
            Row(children: [
              Expanded(child: _ShimmerBox(height: 60)),
              SizedBox(width: 12),
              Expanded(child: _ShimmerBox(height: 60)),
            ]),
          ],
        ),
      ),
    );
  }
}

class _ShimmerBox extends StatelessWidget {
  const _ShimmerBox({this.width, required this.height});
  final double? width;
  final double height;

  @override
  Widget build(BuildContext context) {
    return Container(
      width: width,
      height: height,
      decoration: BoxDecoration(
        color: Theme.of(context).colorScheme.surfaceContainerHighest,
        borderRadius: BorderRadius.circular(8),
      ),
    );
  }
}

class _NetWorthError extends StatelessWidget {
  const _NetWorthError({required this.message});
  final String message;

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 6),
      child: Card(
        color: Theme.of(context).colorScheme.errorContainer,
        child: Padding(
          padding: const EdgeInsets.all(16),
          child: Row(
            children: [
              Icon(Icons.error_outline,
                  color: Theme.of(context).colorScheme.onErrorContainer),
              const SizedBox(width: 12),
              Expanded(
                child: Text(
                  'Could not load net worth.\n$message',
                  style: Theme.of(context).textTheme.bodySmall?.copyWith(
                        color: Theme.of(context).colorScheme.onErrorContainer,
                      ),
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}

class _NetWorthEmpty extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 6),
      child: Card(
        color: Theme.of(context).colorScheme.surfaceContainerLow,
        child: Padding(
          padding: const EdgeInsets.all(24),
          child: Column(
            children: [
              Icon(Icons.account_balance_wallet_outlined,
                  size: 40,
                  color: Theme.of(context).colorScheme.outline),
              const SizedBox(height: 12),
              Text('No accounts yet',
                  style: Theme.of(context).textTheme.titleMedium),
              const SizedBox(height: 4),
              Text(
                'Add an account to see your net worth.',
                style: Theme.of(context).textTheme.bodySmall?.copyWith(
                      color: Theme.of(context).colorScheme.outline,
                    ),
                textAlign: TextAlign.center,
              ),
            ],
          ),
        ),
      ),
    );
  }
}
