/// Flutter translation of RevenueChart.tsx + AverageBasketChart.tsx
///
/// Maps: revenue → total assets, netMargin → net worth,
/// delta → month-over-month change (placeholder).
/// Uses fl_chart LineChart with purple grid and dual-line display.
library;

import 'package:flutter/material.dart';
import 'package:fl_chart/fl_chart.dart';
import '../../theme/app_theme.dart';

class NetWorthChart extends StatelessWidget {
  const NetWorthChart({
    super.key,
    required this.totalAssets,
    required this.totalDebts,
    required this.netWorth,
    required this.currency,
    this.isDeficit = false,
    /// Simulated sparkline data — in a real app, pass historical snapshots.
    this.assetsData,
    this.netWorthData,
  });

  final String totalAssets;
  final String totalDebts;
  final String netWorth;
  final String currency;
  final bool   isDeficit;
  final List<FlSpot>? assetsData;
  final List<FlSpot>? netWorthData;

  @override
  Widget build(BuildContext context) {
    final symbol = currency == 'TWD' ? 'NT\$' : '\$';
    final nwColor = isDeficit ? AppTheme.red : AppTheme.green;

    // Default sparkline data (6-month simulated curve)
    final spots1 = assetsData ?? _defaultAssets();
    final spots2 = netWorthData ?? _defaultNetWorth();

    return Container(
      margin: const EdgeInsets.symmetric(horizontal: 16, vertical: 6),
      decoration: AppTheme.glowCard(),
      padding: const EdgeInsets.all(20),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          // ── Header row (matches RevenueChart header) ──────────────────
          Row(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Expanded(
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    // Title
                    const Text(
                      'Net Worth & Assets',
                      style: TextStyle(
                        color: AppTheme.textPrimary,
                        fontSize: 18,
                        fontWeight: FontWeight.w600,
                      ),
                    ),
                    const SizedBox(height: 10),
                    // Big net worth number
                    Text(
                      '$symbol ${_fmt(netWorth)}',
                      style: TextStyle(
                        color: nwColor,
                        fontSize: 26,
                        fontWeight: FontWeight.w700,
                        letterSpacing: -0.5,
                      ),
                    ),
                    const SizedBox(height: 4),
                    // Delta badge (static placeholder — extend for real data)
                    _DeltaBadge(value: isDeficit ? -2.4 : 12.4),
                  ],
                ),
              ),
            ],
          ),

          const SizedBox(height: 20),

          // ── Line chart (300px, matches ResponsiveContainer height) ────
          SizedBox(
            height: 180,
            child: LineChart(
              LineChartData(
                gridData: FlGridData(
                  show: true,
                  drawVerticalLine: false,
                  getDrawingHorizontalLine: (_) => FlLine(
                    color: AppTheme.chartGrid,
                    strokeWidth: 1,
                  ),
                ),
                titlesData: FlTitlesData(
                  leftTitles: AxisTitles(
                    sideTitles: SideTitles(
                      showTitles: true,
                      reservedSize: 52,
                      getTitlesWidget: (v, _) => Text(
                        _shortFmt(v, symbol),
                        style: const TextStyle(
                            color: AppTheme.textSecondary, fontSize: 10),
                      ),
                    ),
                  ),
                  rightTitles: const AxisTitles(
                      sideTitles: SideTitles(showTitles: false)),
                  topTitles: const AxisTitles(
                      sideTitles: SideTitles(showTitles: false)),
                  bottomTitles: AxisTitles(
                    sideTitles: SideTitles(
                      showTitles: true,
                      interval: 1,
                      getTitlesWidget: (v, _) {
                        const months = ['Oct','Nov','Dec','Jan','Feb','Mar'];
                        final i = v.toInt();
                        if (i < 0 || i >= months.length) return const SizedBox();
                        return Text(months[i],
                            style: const TextStyle(
                                color: AppTheme.textSecondary, fontSize: 10));
                      },
                    ),
                  ),
                ),
                borderData: FlBorderData(show: false),
                lineBarsData: [
                  // Assets line (purple — matches revenue line)
                  LineChartBarData(
                    spots: spots1,
                    isCurved: true,
                    color: AppTheme.chartPurple,
                    barWidth: 2,
                    dotData: const FlDotData(show: false),
                    belowBarData: BarAreaData(
                      show: true,
                      color: AppTheme.chartPurple.withOpacity(0.08),
                    ),
                  ),
                  // Net worth line (cyan — matches netMargin line)
                  LineChartBarData(
                    spots: spots2,
                    isCurved: true,
                    color: AppTheme.chartCyan,
                    barWidth: 2,
                    dotData: const FlDotData(show: false),
                    belowBarData: BarAreaData(
                      show: true,
                      color: AppTheme.chartCyan.withOpacity(0.05),
                    ),
                  ),
                ],
                lineTouchData: LineTouchData(
                  touchTooltipData: LineTouchTooltipData(
                    getTooltipColor: (_) => AppTheme.cardBg,
                    tooltipBorder: const BorderSide(color: AppTheme.purpleBorder),
                    tooltipRoundedRadius: 10,
                    getTooltipItems: (spots) => spots.map((s) {
                      final isAssets = s.barIndex == 0;
                      return LineTooltipItem(
                        '$symbol ${_fmt(s.y.toStringAsFixed(2))}',
                        TextStyle(
                          color: isAssets ? AppTheme.chartPurple : AppTheme.chartCyan,
                          fontWeight: FontWeight.w600,
                          fontSize: 12,
                        ),
                        children: [
                          TextSpan(
                            text: '\n${isAssets ? 'Assets' : 'Net Worth'}',
                            style: const TextStyle(
                              color: AppTheme.textSecondary,
                              fontWeight: FontWeight.w400,
                              fontSize: 11,
                            ),
                          ),
                        ],
                      );
                    }).toList(),
                  ),
                ),
              ),
            ),
          ),

          const SizedBox(height: 16),

          // ── Legend + mini stats (matches assets/debts row) ────────────
          Row(
            children: [
              _LegendDot(color: AppTheme.chartPurple, label: 'Assets'),
              const SizedBox(width: 16),
              _LegendDot(color: AppTheme.chartCyan, label: 'Net Worth'),
              const Spacer(),
              _MiniStat(label: 'Debts', value: '$symbol ${_fmt(totalDebts)}',
                  color: AppTheme.red),
            ],
          ),
        ],
      ),
    );
  }

  // ── Helpers ──────────────────────────────────────────────────────────────

  String _fmt(String raw) {
    final isNeg = raw.startsWith('-');
    final clean = raw.replaceAll('-', '').replaceAll(',', '');
    final parts = clean.split('.');
    final intPart = parts[0].replaceAllMapped(
        RegExp(r'(\d{1,3})(?=(\d{3})+(?!\d))'), (m) => '${m[1]},');
    final decimals = parts.length > 1 ? parts[1] : '00';
    final formatted = '$intPart.$decimals';
    return isNeg ? '-$formatted' : formatted;
  }

  String _shortFmt(double v, String symbol) {
    if (v >= 1000000) return '$symbol${(v/1000000).toStringAsFixed(1)}M';
    if (v >= 1000)    return '$symbol${(v/1000).toStringAsFixed(0)}k';
    return '$symbol${v.toStringAsFixed(0)}';
  }

  List<FlSpot> _defaultAssets() => const [
        FlSpot(0, 75000), FlSpot(1, 78000), FlSpot(2, 82000),
        FlSpot(3, 80000), FlSpot(4, 88000), FlSpot(5, 95000),
      ];

  List<FlSpot> _defaultNetWorth() => const [
        FlSpot(0, 5000),  FlSpot(1, 3000),  FlSpot(2, 8000),
        FlSpot(3, 6000),  FlSpot(4, 10000), FlSpot(5, 12000),
      ];
}

// ── Sub-widgets ────────────────────────────────────────────────────────────────

class _DeltaBadge extends StatelessWidget {
  const _DeltaBadge({required this.value});
  final double value;

  @override
  Widget build(BuildContext context) {
    final isPos = value > 0;
    final isNeutral = value.abs() < 0.5;
    final color = isNeutral
        ? AppTheme.textSecondary
        : isPos ? AppTheme.green : AppTheme.red;
    final sign = isPos ? '+' : '';
    final label = isNeutral ? '0%' : '$sign${value.toStringAsFixed(1)}%';

    return Row(
      mainAxisSize: MainAxisSize.min,
      children: [
        Icon(
          isNeutral
              ? Icons.remove_rounded
              : isPos ? Icons.trending_up_rounded : Icons.trending_down_rounded,
          color: color,
          size: 14,
        ),
        const SizedBox(width: 4),
        Text(label,
            style: TextStyle(
                color: color, fontSize: 12, fontWeight: FontWeight.w500)),
      ],
    );
  }
}

class _LegendDot extends StatelessWidget {
  const _LegendDot({required this.color, required this.label});
  final Color  color;
  final String label;

  @override
  Widget build(BuildContext context) => Row(
        mainAxisSize: MainAxisSize.min,
        children: [
          Container(
            width: 10, height: 10,
            decoration: BoxDecoration(
                color: color, borderRadius: BorderRadius.circular(2)),
          ),
          const SizedBox(width: 6),
          Text(label,
              style: const TextStyle(
                  color: AppTheme.textSecondary, fontSize: 12)),
        ],
      );
}

class _MiniStat extends StatelessWidget {
  const _MiniStat({
    required this.label,
    required this.value,
    required this.color,
  });
  final String label;
  final String value;
  final Color  color;

  @override
  Widget build(BuildContext context) => Column(
        crossAxisAlignment: CrossAxisAlignment.end,
        children: [
          Text(label,
              style: const TextStyle(
                  color: AppTheme.textSecondary, fontSize: 11)),
          Text(value,
              style: TextStyle(
                  color: color, fontSize: 13, fontWeight: FontWeight.w600)),
        ],
      );
}
