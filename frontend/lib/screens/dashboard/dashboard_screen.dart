import 'package:fl_chart/fl_chart.dart';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../core/providers.dart';
import '../../models/account_summary.dart';
import '../../models/ledger_entry.dart';
import '../../models/net_worth_result.dart';
import '../../screens/accounts/add_account_sheet.dart';
import '../../theme/app_theme.dart';
import 'widgets/kpi_card.dart';


class DashboardScreen extends ConsumerWidget {
  const DashboardScreen({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final accountsAsync = ref.watch(accountsProvider);
    final nwAsync       = ref.watch(netWorthProvider);
    final accounts      = accountsAsync.asData?.value ?? [];
    final nwList        = nwAsync.asData?.value ?? [];

    return Scaffold(
      backgroundColor: AppTheme.pageBg,
      body: RefreshIndicator(
        onRefresh: () async => refreshDashboard(ref),
        color: AppTheme.purple,
        child: CustomScrollView(
          slivers: [
            // ── Page header ───────────────────────────────────────────
            SliverToBoxAdapter(
              child: Padding(
                padding: const EdgeInsets.fromLTRB(24, 24, 24, 0),
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Text('Dashboard',
                        style: Theme.of(context)
                            .textTheme
                            .headlineMedium
                            ?.copyWith(fontWeight: FontWeight.w700)),
                    const SizedBox(height: 4),
                    const Text('Overview of your financial portfolio',
                        style: TextStyle(
                            color: AppTheme.textSecondary, fontSize: 14)),
                  ],
                ),
              ),
            ),

            // ── KPI cards ─────────────────────────────────────────────
            SliverToBoxAdapter(
              child: Padding(
                padding: const EdgeInsets.fromLTRB(24, 20, 24, 0),
                child: _KpiRow(accounts: accounts, nwList: nwList),
              ),
            ),

            // ── Charts row ────────────────────────────────────────────
            SliverToBoxAdapter(
              child: Padding(
                padding: const EdgeInsets.fromLTRB(24, 20, 24, 0),
                child: LayoutBuilder(builder: (ctx, c) {
                  final wide = c.maxWidth > 700;
                  final children = [
                    _NetWorthTrendCard(nwList: nwList),
                    _AssetAllocationCard(accounts: accounts),
                  ];
                  return wide
                      ? Row(
                          crossAxisAlignment: CrossAxisAlignment.start,
                          children: [
                            Expanded(child: children[0]),
                            const SizedBox(width: 16),
                            Expanded(child: children[1]),
                          ],
                        )
                      : Column(children: [
                          children[0],
                          const SizedBox(height: 16),
                          children[1],
                        ]);
                }),
              ),
            ),

            // ── Bottom row ─────────────────────────────────────────────
            SliverToBoxAdapter(
              child: Padding(
                padding: const EdgeInsets.fromLTRB(24, 20, 24, 0),
                child: LayoutBuilder(builder: (ctx, c) {
                  final wide = c.maxWidth > 700;
                  final children = [
                    _RecentActivityCard(accounts: accounts),
                    _LiabilitiesBreakdownCard(accounts: accounts),
                  ];
                  return wide
                      ? Row(
                          crossAxisAlignment: CrossAxisAlignment.start,
                          children: [
                            Expanded(child: children[0]),
                            const SizedBox(width: 16),
                            Expanded(child: children[1]),
                          ],
                        )
                      : Column(children: [
                          children[0],
                          const SizedBox(height: 16),
                          children[1],
                        ]);
                }),
              ),
            ),

            const SliverToBoxAdapter(child: SizedBox(height: 40)),
          ],
        ),
      ),
      floatingActionButton: FloatingActionButton.extended(
        onPressed: () => showAddAccountSheet(context, ref),
        icon: const Icon(Icons.add_rounded),
        label: const Text('Add Account'),
        backgroundColor: AppTheme.purple,
        foregroundColor: Colors.white,
      ),
    );
  }
}

// ── KPI row ───────────────────────────────────────────────────────────────────

class _KpiRow extends ConsumerWidget {
  const _KpiRow({required this.accounts, required this.nwList});
  final List<AccountSummary>  accounts;
  final List<NetWorthResult>  nwList;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final entriesAsync = ref.watch(allEntriesProvider);
    final nw = nwList.isNotEmpty ? nwList.first : null;

    double totalAssets  = 0;
    double totalDebts   = 0;
    double liquidAssets = 0;

    for (final r in nwList) {
      totalAssets += double.tryParse(r.totalAssets) ?? 0;
      totalDebts  += double.tryParse(r.totalDebts)  ?? 0;
    }
    for (final a in accounts) {
      if (const {'cash', 'physical_wallet', 'digital_wallet'}
          .contains(a.accountType)) {
        liquidAssets += double.tryParse(a.balance) ?? 0;
      }
    }

    final netWorth = totalAssets - totalDebts;
    final sym = (nw?.currency == 'TWD') ? 'NT\$' : '\$';

    // Compute month-to-date change from real entries
    final double? nwDelta = entriesAsync.asData?.value != null
        ? _computeMonthDelta(entriesAsync.asData!.value, netWorth)
        : null;

    String fmt(double v) {
      final abs   = v.abs();
      final s     = abs.toStringAsFixed(2);
      final parts = s.split('.');
      final intPart = parts[0].replaceAllMapped(
          RegExp(r'(\d{1,3})(?=(\d{3})+(?!\d))'), (m) => '${m[1]},');
      return '${v < 0 ? '-' : ''}$sym$intPart.${parts[1]}';
    }

    final cards = [
      KpiCard(
        label: 'Net Worth',
        value: fmt(netWorth),
        icon: Icons.trending_up_rounded,
        delta: nwDelta,
      ),
      KpiCard(
        label: 'Total Assets',
        value: fmt(totalAssets),
        icon: Icons.account_balance_wallet_rounded,
        iconColor: AppTheme.green,
      ),
      KpiCard(
        label: 'Total Liabilities',
        value: fmt(totalDebts),
        icon: Icons.credit_card_rounded,
        iconColor: AppTheme.red,
      ),
      KpiCard(
        label: 'Liquid Assets',
        value: fmt(liquidAssets),
        icon: Icons.water_drop_rounded,
        iconColor: const Color(0xFF22D3EE),
      ),
    ];

    return LayoutBuilder(builder: (_, c) {
      final wide = c.maxWidth > 700;
      if (wide) {
        return Row(
          children: cards
              .map((w) => Expanded(
                    child: Padding(
                      padding: EdgeInsets.only(
                          right: cards.indexOf(w) < cards.length - 1 ? 12 : 0),
                      child: w,
                    ),
                  ))
              .toList(),
        );
      }
      return Column(children: [
        Row(children: [
          Expanded(child: cards[0]),
          const SizedBox(width: 12),
          Expanded(child: cards[1]),
        ]),
        const SizedBox(height: 12),
        Row(children: [
          Expanded(child: cards[2]),
          const SizedBox(width: 12),
          Expanded(child: cards[3]),
        ]),
      ]);
    });
  }

  /// Computes percentage change in net worth for the current calendar month.
  /// Sum all credits and debits that occurred this month, then express as
  /// a percentage of (currentNW - thisMonthChange) = start-of-month NW.
  double? _computeMonthDelta(List<LedgerEntry> entries, double currentNW) {
    final now   = DateTime.now();
    final start = DateTime(now.year, now.month, 1);
    double monthChange = 0;
    for (final e in entries) {
      if (e.occurredAt.isBefore(start)) continue;
      final amt = double.tryParse(e.amount) ?? 0;
      monthChange += e.isCredit ? amt : -amt;
    }
    if (monthChange == 0) return null;
    final startNW = currentNW - monthChange;
    if (startNW == 0) return null;
    return (monthChange / startNW.abs()) * 100;
  }
}

// ── Net Worth Trend ───────────────────────────────────────────────────────────

class _NetWorthTrendCard extends ConsumerWidget {
  const _NetWorthTrendCard({required this.nwList});
  final List<NetWorthResult> nwList;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final entriesAsync = ref.watch(allEntriesProvider);

    // Current net worth (sum across all currencies, simplified to numeric)
    double currentNW = 0;
    for (final r in nwList) {
      currentNW += (double.tryParse(r.totalAssets) ?? 0) -
          (double.tryParse(r.totalDebts) ?? 0);
    }

    return _PremiumCard(
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(
            mainAxisAlignment: MainAxisAlignment.spaceBetween,
            children: [
              const Text('Net Worth Trend',
                  style: TextStyle(
                      color: AppTheme.textPrimary,
                      fontWeight: FontWeight.w600,
                      fontSize: 15)),
              Container(
                padding: const EdgeInsets.symmetric(horizontal: 10, vertical: 4),
                decoration: BoxDecoration(
                  color: AppTheme.purple.withOpacity(0.15),
                  borderRadius: BorderRadius.circular(8),
                ),
                child: const Text('30d',
                    style: TextStyle(
                        color: AppTheme.purple,
                        fontSize: 11,
                        fontWeight: FontWeight.w600)),
              ),
            ],
          ),
          const SizedBox(height: 20),
          entriesAsync.when(
            loading: () => const SizedBox(
                height: 180,
                child: Center(child: CircularProgressIndicator())),
            error: (_, __) => const SizedBox(height: 180),
            data: (entries) {
              final spots = _buildSpots(entries, currentNW);
              final labels = _buildLabels(entries);

              if (spots.length < 2) {
                return SizedBox(
                  height: 180,
                  child: Center(
                    child: Text(
                      spots.isEmpty
                          ? 'No transactions yet'
                          : 'Add more transactions to see the trend',
                      style: const TextStyle(
                          color: AppTheme.textSecondary, fontSize: 13),
                    ),
                  ),
                );
              }

              return SizedBox(
                height: 180,
                child: LineChart(
                  LineChartData(
                    gridData: FlGridData(
                      show: true,
                      drawVerticalLine: false,
                      getDrawingHorizontalLine: (_) => FlLine(
                        color: AppTheme.purple.withOpacity(0.08),
                        strokeWidth: 1,
                      ),
                    ),
                    borderData: FlBorderData(show: false),
                    titlesData: FlTitlesData(
                      leftTitles: const AxisTitles(
                          sideTitles: SideTitles(showTitles: false)),
                      topTitles: const AxisTitles(
                          sideTitles: SideTitles(showTitles: false)),
                      rightTitles: const AxisTitles(
                          sideTitles: SideTitles(showTitles: false)),
                      bottomTitles: AxisTitles(
                        sideTitles: SideTitles(
                          showTitles: true,
                          interval: (spots.length / 4).ceilToDouble().clamp(1, 999),
                          getTitlesWidget: (v, _) {
                            final i = v.toInt();
                            if (i < 0 || i >= labels.length) return const SizedBox.shrink();
                            return Text(labels[i],
                                style: const TextStyle(
                                    color: AppTheme.textSecondary, fontSize: 10));
                          },
                        ),
                      ),
                    ),
                    lineBarsData: [
                      LineChartBarData(
                        spots: spots,
                        isCurved: true,
                        color: AppTheme.purple,
                        barWidth: 2,
                        dotData: const FlDotData(show: false),
                        belowBarData: BarAreaData(
                          show: true,
                          gradient: LinearGradient(
                            begin: Alignment.topCenter,
                            end: Alignment.bottomCenter,
                            colors: [
                              AppTheme.purple.withOpacity(0.3),
                              AppTheme.purple.withOpacity(0.0),
                            ],
                          ),
                        ),
                      ),
                    ],
                    lineTouchData: LineTouchData(
                      touchTooltipData: LineTouchTooltipData(
                        getTooltipColor: (_) => AppTheme.cardBg,
                        tooltipBorder: const BorderSide(color: AppTheme.purpleBorder),
                        tooltipRoundedRadius: 8,
                        getTooltipItems: (touchedSpots) =>
                            touchedSpots.map((s) {
                          final sym = (nwList.firstOrNull?.currency ?? 'USD') == 'TWD' ? 'NT\$' : '\$';
                          return LineTooltipItem(
                            '$sym ${_fmtNum(s.y)}',
                            const TextStyle(
                                color: AppTheme.purpleValue,
                                fontWeight: FontWeight.w600,
                                fontSize: 12),
                          );
                        }).toList(),
                      ),
                    ),
                  ),
                ),
              );
            },
          ),
        ],
      ),
    );
  }

  /// Build daily running net-worth spots over the last 30 days.
  /// Starts from (currentNW - total of all changes) and replays forward.
  List<FlSpot> _buildSpots(List<LedgerEntry> entries, double currentNW) {
    final now = DateTime.now();
    final cutoff = now.subtract(const Duration(days: 30));
    final recent = entries.where((e) => e.occurredAt.isAfter(cutoff)).toList()
      ..sort((a, b) => a.occurredAt.compareTo(b.occurredAt));

    if (recent.isEmpty) return [];

    // Compute total change over the window so we can back-calculate start value
    double totalChange = 0;
    for (final e in recent) {
      final amt = double.tryParse(e.amount) ?? 0;
      totalChange += e.isCredit ? amt : -amt;
    }
    double running = currentNW - totalChange;

    // Group by day
    final Map<String, double> dailyChange = {};
    for (final e in recent) {
      final key = '${e.occurredAt.year}-${e.occurredAt.month.toString().padLeft(2,'0')}-${e.occurredAt.day.toString().padLeft(2,'0')}';
      final amt = double.tryParse(e.amount) ?? 0;
      dailyChange[key] = (dailyChange[key] ?? 0) + (e.isCredit ? amt : -amt);
    }

    final sortedDays = dailyChange.keys.toList()..sort();
    final spots = <FlSpot>[];
    for (var i = 0; i < sortedDays.length; i++) {
      running += dailyChange[sortedDays[i]]!;
      spots.add(FlSpot(i.toDouble(), running));
    }
    // Always include current value as last point if not already today
    if (spots.isNotEmpty && spots.last.y != currentNW) {
      spots.add(FlSpot(spots.length.toDouble(), currentNW));
    }
    return spots;
  }

  List<String> _buildLabels(List<LedgerEntry> entries) {
    final now = DateTime.now();
    final cutoff = now.subtract(const Duration(days: 30));
    final recent = entries.where((e) => e.occurredAt.isAfter(cutoff)).toList()
      ..sort((a, b) => a.occurredAt.compareTo(b.occurredAt));

    final seen = <String>{};
    final labels = <String>[];
    for (final e in recent) {
      final key = '${e.occurredAt.year}-${e.occurredAt.month.toString().padLeft(2,'0')}-${e.occurredAt.day.toString().padLeft(2,'0')}';
      if (seen.add(key)) {
        labels.add('${e.occurredAt.month}/${e.occurredAt.day}');
      }
    }
    if (labels.isNotEmpty) labels.add('Now');
    return labels;
  }

  String _fmtNum(double v) {
    if (v.abs() >= 1000000) return '${(v / 1000000).toStringAsFixed(1)}M';
    if (v.abs() >= 1000)    return '${(v / 1000).toStringAsFixed(1)}k';
    return v.toStringAsFixed(0);
  }
}

// ── Asset Allocation ──────────────────────────────────────────────────────────

class _AssetAllocationCard extends StatelessWidget {
  const _AssetAllocationCard({required this.accounts});
  final List<AccountSummary> accounts;

  @override
  Widget build(BuildContext context) {
    final assetAccounts = accounts
        .where((a) => !a.isDebt)
        .toList();

    if (assetAccounts.isEmpty) {
      return _PremiumCard(
        child: const Center(
            child: Padding(
          padding: EdgeInsets.all(40),
          child: Text('No assets yet',
              style: TextStyle(color: AppTheme.textSecondary)),
        )),
      );
    }

    final colors = [
      AppTheme.purple,
      AppTheme.purpleLight,
      const Color(0xFF22D3EE),
      const Color(0xFFEC4899),
      const Color(0xFF22C55E),
      const Color(0xFFF59E0B),
    ];

    final sections = <PieChartSectionData>[];
    final legendItems = <_LegendItem>[];
    double total = 0;
    for (final a in assetAccounts) {
      total += double.tryParse(a.balance) ?? 0;
    }

    for (var i = 0; i < assetAccounts.length; i++) {
      final a   = assetAccounts[i];
      final val = double.tryParse(a.balance) ?? 0;
      final pct = total > 0 ? val / total * 100 : 0;
      final col = colors[i % colors.length];
      sections.add(PieChartSectionData(
        value: val > 0 ? val : 0.1,
        color: col,
        radius: 52,
        title: '',
        showTitle: false,
      ));
      legendItems.add(_LegendItem(label: a.accountName, color: col));
    }

    return _PremiumCard(
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          const Text('Asset Allocation',
              style: TextStyle(
                  color: AppTheme.textPrimary,
                  fontWeight: FontWeight.w600,
                  fontSize: 15)),
          const SizedBox(height: 20),
          SizedBox(
            height: 180,
            child: PieChart(
              PieChartData(
                sections: sections,
                centerSpaceRadius: 44,
                sectionsSpace: 2,
              ),
            ),
          ),
          const SizedBox(height: 16),
          Wrap(
            spacing: 16,
            runSpacing: 8,
            children: legendItems.map((item) => Row(
                  mainAxisSize: MainAxisSize.min,
                  children: [
                    Container(
                        width: 10,
                        height: 10,
                        decoration: BoxDecoration(
                            color: item.color,
                            shape: BoxShape.circle)),
                    const SizedBox(width: 6),
                    Text(item.label,
                        style: const TextStyle(
                            color: AppTheme.textSecondary,
                            fontSize: 12)),
                  ],
                )).toList(),
          ),
        ],
      ),
    );
  }
}

class _LegendItem {
  const _LegendItem({required this.label, required this.color});
  final String label;
  final Color  color;
}

// ── Recent Activity ───────────────────────────────────────────────────────────

class _RecentActivityCard extends ConsumerWidget {
  const _RecentActivityCard({required this.accounts});
  final List<AccountSummary> accounts;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    // Use allEntriesProvider so every account's transactions appear here
    final entriesAsync = ref.watch(allEntriesProvider);

    // Build a quick lookup: accountId → accountName
    final nameMap = {for (final a in accounts) a.accountId: a.accountName};

    return _PremiumCard(
      padding: EdgeInsets.zero,
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          const Padding(
            padding: EdgeInsets.fromLTRB(20, 20, 20, 12),
            child: Text('Recent Activity',
                style: TextStyle(
                    color: AppTheme.textPrimary,
                    fontWeight: FontWeight.w600,
                    fontSize: 15)),
          ),
          const Divider(height: 1, color: AppTheme.border),
          if (accounts.isEmpty)
            const Padding(
              padding: EdgeInsets.all(24),
              child: Center(
                  child: Text('Add accounts to see activity',
                      style: TextStyle(
                          color: AppTheme.textSecondary, fontSize: 13))),
            )
          else
            entriesAsync.when(
              loading: () => const Padding(
                  padding: EdgeInsets.all(24),
                  child: Center(child: CircularProgressIndicator())),
              error: (e, _) => Padding(
                padding: const EdgeInsets.all(16),
                child: Text('Could not load activity: $e',
                    style: const TextStyle(
                        color: AppTheme.textSecondary, fontSize: 12)),
              ),
              data: (entries) {
                // entries are already sorted newest-first by allEntriesProvider
                final recent = entries.take(5).toList();
                if (recent.isEmpty) {
                  return const Padding(
                    padding: EdgeInsets.all(24),
                    child: Center(
                        child: Text('No transactions yet',
                            style: TextStyle(
                                color: AppTheme.textSecondary, fontSize: 13))),
                  );
                }
                return Column(
                  children: recent.map((e) {
                    final isIncome = e.isCredit;
                    final color = isIncome ? AppTheme.green : AppTheme.red;
                    final accountName = nameMap[e.accountId] ?? '';
                    final symbol = e.currency == 'TWD' ? 'NT\$' : '\$';
                    final date = e.occurredAt.toLocal();
                    final dateStr =
                        '${date.month}/${date.day}/${date.year}';
                    return Container(
                      padding: const EdgeInsets.symmetric(
                          horizontal: 20, vertical: 14),
                      decoration: const BoxDecoration(
                          border: Border(
                              bottom: BorderSide(
                                  color: AppTheme.border, width: 0.5))),
                      child: Row(
                        children: [
                          Container(
                            width: 40,
                            height: 40,
                            decoration: BoxDecoration(
                              color: color.withOpacity(0.1),
                              borderRadius: BorderRadius.circular(10),
                            ),
                            child: Icon(
                              isIncome
                                  ? Icons.arrow_downward_rounded
                                  : Icons.arrow_upward_rounded,
                              color: color,
                              size: 18,
                            ),
                          ),
                          const SizedBox(width: 12),
                          Expanded(
                            child: Column(
                              crossAxisAlignment: CrossAxisAlignment.start,
                              children: [
                                Text(
                                  e.label ?? e.typeDisplayName,
                                  style: const TextStyle(
                                      color: AppTheme.textPrimary,
                                      fontWeight: FontWeight.w500,
                                      fontSize: 13),
                                ),
                                const SizedBox(height: 2),
                                Text(
                                  accountName.isNotEmpty
                                      ? '$accountName · $dateStr'
                                      : dateStr,
                                  style: const TextStyle(
                                      color: AppTheme.textSecondary,
                                      fontSize: 11),
                                ),
                              ],
                            ),
                          ),
                          Text(
                            '${isIncome ? '+' : '-'}$symbol${e.amount}',
                            style: TextStyle(
                                color: color,
                                fontWeight: FontWeight.w600,
                                fontSize: 13),
                          ),
                        ],
                      ),
                    );
                  }).toList(),
                );
              },
            ),
        ],
      ),
    );
  }
}

// ── Liabilities Breakdown ─────────────────────────────────────────────────────

class _LiabilitiesBreakdownCard extends StatelessWidget {
  const _LiabilitiesBreakdownCard({required this.accounts});
  final List<AccountSummary> accounts;

  @override
  Widget build(BuildContext context) {
    final liabilities =
        accounts.where((a) => a.isDebt).toList();
    double total = 0;
    for (final a in liabilities) {
      total += double.tryParse(a.balance) ?? 0;
    }

    final colors = [AppTheme.red, AppTheme.amber, AppTheme.green,
        const Color(0xFF6366F1)];

    return _PremiumCard(
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          const Text('Liabilities Breakdown',
              style: TextStyle(
                  color: AppTheme.textPrimary,
                  fontWeight: FontWeight.w600,
                  fontSize: 15)),
          const SizedBox(height: 20),
          if (liabilities.isEmpty)
            const Center(
                child: Padding(
              padding: EdgeInsets.all(24),
              child: Text('No liabilities',
                  style: TextStyle(
                      color: AppTheme.textSecondary, fontSize: 13)),
            ))
          else
            ...liabilities.asMap().entries.map((e) {
              final a   = e.value;
              final val = double.tryParse(a.balance) ?? 0;
              final pct = total > 0 ? val / total : 0;
              final col = colors[e.key % colors.length];
              return Padding(
                padding: const EdgeInsets.only(bottom: 14),
                child: Column(
                  children: [
                    Row(
                      mainAxisAlignment: MainAxisAlignment.spaceBetween,
                      children: [
                        Text(a.accountName,
                            style: const TextStyle(
                                color: AppTheme.textPrimary,
                                fontSize: 13)),
                        Text('\$${a.balance}',
                            style: const TextStyle(
                                color: AppTheme.textSecondary,
                                fontSize: 13)),
                      ],
                    ),
                    const SizedBox(height: 6),
                    ClipRRect(
                      borderRadius: BorderRadius.circular(4),
                      child: LinearProgressIndicator(
                        value: pct.toDouble(),
                        minHeight: 6,
                        backgroundColor: AppTheme.border,
                        color: col,
                      ),
                    ),
                  ],
                ),
              );
            }),
        ],
      ),
    );
  }
}

// ── Premium card shell ────────────────────────────────────────────────────────

class _PremiumCard extends StatelessWidget {
  const _PremiumCard({required this.child, this.padding});
  final Widget  child;
  final EdgeInsetsGeometry? padding;

  @override
  Widget build(BuildContext context) => Container(
        padding: padding ?? const EdgeInsets.all(20),
        decoration: BoxDecoration(
          color: AppTheme.surface,
          borderRadius: BorderRadius.circular(16),
          border: Border.all(color: AppTheme.border),
        ),
        child: child,
      );
}

