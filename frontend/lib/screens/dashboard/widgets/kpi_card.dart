import 'package:flutter/material.dart';
import '../../../theme/app_theme.dart';

class KpiCard extends StatelessWidget {
  const KpiCard({
    super.key,
    required this.label,
    required this.value,
    required this.icon,
    this.subtitle,
    this.delta,
    this.iconColor,
  });

  final String   label;
  final String   value;
  final IconData icon;
  final String?  subtitle;
  final double?  delta;   // % change; positive = good
  final Color?   iconColor;

  @override
  Widget build(BuildContext context) {
    final tt = Theme.of(context).textTheme;
    final ic = iconColor ?? AppTheme.purple;

    return Container(
      padding: const EdgeInsets.all(20),
      decoration: BoxDecoration(
        color: AppTheme.surface,
        borderRadius: BorderRadius.circular(16),
        border: Border.all(color: AppTheme.purple.withOpacity(0.2)),
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(
            mainAxisAlignment: MainAxisAlignment.spaceBetween,
            children: [
              Expanded(
                child: Text(label,
                    style: tt.bodySmall
                        ?.copyWith(color: AppTheme.textSecondary)),
              ),
              Container(
                width: 36, height: 36,
                decoration: BoxDecoration(
                  color: ic.withOpacity(0.15),
                  borderRadius: BorderRadius.circular(10),
                ),
                child: Icon(icon, size: 16, color: ic),
              ),
            ],
          ),
          const SizedBox(height: 14),
          Text(
            value,
            style: const TextStyle(
              color: AppTheme.textPrimary,
              fontSize: 26,
              fontWeight: FontWeight.w700,
              letterSpacing: -0.5,
            ),
          ),
          if (delta != null || subtitle != null) ...[
            const SizedBox(height: 8),
            if (delta != null) _DeltaBadge(delta: delta!),
            if (subtitle != null)
              Text(subtitle!,
                  style: tt.labelSmall
                      ?.copyWith(color: AppTheme.textSecondary)),
          ],
        ],
      ),
    );
  }
}

class _DeltaBadge extends StatelessWidget {
  const _DeltaBadge({required this.delta});
  final double delta;

  @override
  Widget build(BuildContext context) {
    final isNeutral  = delta.abs() < 0.5;
    final isPositive = delta > 0;
    final color = isNeutral
        ? AppTheme.textSecondary
        : isPositive
            ? AppTheme.green
            : AppTheme.red;
    final arrow = isNeutral ? '▬' : (isPositive ? '▲' : '▼');
    final label = isNeutral
        ? '0%'
        : '${isPositive ? '+' : ''}${delta.toStringAsFixed(1)}%';

    return Row(
      children: [
        Text(arrow, style: TextStyle(fontSize: 10, color: color)),
        const SizedBox(width: 4),
        Text(label,
            style: TextStyle(
                fontSize: 12, fontWeight: FontWeight.w500, color: color)),
      ],
    );
  }
}
