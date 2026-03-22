/// Shared widgets used across all account-type detail screens.
/// Design translated from AccountLauncherPage.tsx profile cards and
/// ListingCardNew.tsx card/badge/button patterns.
library;

import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../core/providers.dart';
import '../../models/account_summary.dart';
import '../../models/ledger_entry.dart';
import '../../theme/app_theme.dart';
import '../../screens/accounts/account_entries_screen.dart';
import '../../screens/accounts/edit_account_sheet.dart';
import '../../shared/widgets/form_sheet.dart';

// ── Provider ──────────────────────────────────────────────────────────────────

final accountEntriesProvider =
    FutureProvider.family<List<LedgerEntry>, String>(
  (ref, accountId) => ref.watch(ledgerRepoProvider).listEntries(accountId),
);

// ── Info row ──────────────────────────────────────────────────────────────────

class InfoRow extends StatelessWidget {
  const InfoRow({
    super.key,
    required this.label,
    required this.value,
    this.valueColor,
    this.icon,
  });
  final String   label;
  final String   value;
  final Color?   valueColor;
  final IconData? icon;

  @override
  Widget build(BuildContext context) => Padding(
        padding: const EdgeInsets.symmetric(vertical: 9),
        child: Row(
          children: [
            if (icon != null) ...[
              Icon(icon, size: 16, color: AppTheme.textSecondary),
              const SizedBox(width: 8),
            ],
            Text(label,
                style: const TextStyle(
                    color: AppTheme.textSecondary, fontSize: 13)),
            const Spacer(),
            Text(value,
                style: TextStyle(
                  color: valueColor ?? AppTheme.textPrimary,
                  fontSize: 13,
                  fontWeight: FontWeight.w600,
                )),
          ],
        ),
      );
}

// ── Section card (translated from ListingCardNew.tsx content area) ────────────

class SectionCard extends StatelessWidget {
  const SectionCard({super.key, required this.title, required this.children});
  final String       title;
  final List<Widget> children;

  @override
  Widget build(BuildContext context) => Container(
        margin: const EdgeInsets.symmetric(horizontal: 16, vertical: 6),
        decoration: AppTheme.glowCard(),
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text(
              title.toUpperCase(),
              style: const TextStyle(
                color: AppTheme.textSecondary,
                fontSize: 11,
                fontWeight: FontWeight.w700,
                letterSpacing: 1.2,
              ),
            ),
            const SizedBox(height: 12),
            const Divider(color: AppTheme.border, height: 1),
            const SizedBox(height: 8),
            ...children,
          ],
        ),
      );
}

// ── Balance hero (translated from AccountLauncherPage identity block) ─────────

class BalanceHero extends StatelessWidget {
  const BalanceHero({
    super.key,
    required this.label,
    required this.amount,
    required this.currency,
    required this.icon,
    this.isDebt = false,
  });
  final String   label;
  final String   amount;
  final String   currency;
  final IconData icon;
  final bool     isDebt;

  @override
  Widget build(BuildContext context) {
    final color  = isDebt ? AppTheme.red : AppTheme.green;
    final symbol = currency == 'TWD' ? 'NT\$' : '\$';

    return Container(
      margin: const EdgeInsets.all(16),
      decoration: AppTheme.glowCard(
        color: color.withOpacity(0.06),
        radius: 20,
      ),
      padding: const EdgeInsets.all(20),
      child: Row(
        children: [
          // Icon avatar (matches AccountLauncherPage avatar)
          Container(
            width: 52, height: 52,
            decoration: BoxDecoration(
              color: color.withOpacity(0.15),
              borderRadius: BorderRadius.circular(14),
              border: Border.all(color: color.withOpacity(0.3)),
            ),
            child: Icon(icon, color: color, size: 24),
          ),
          const SizedBox(width: 16),
          Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Text(label,
                  style: const TextStyle(
                      color: AppTheme.textSecondary,
                      fontSize: 12,
                      fontWeight: FontWeight.w500)),
              const SizedBox(height: 4),
              Text(
                '$symbol ${_fmt(amount)}',
                style: TextStyle(
                  color: color,
                  fontSize: 26,
                  fontWeight: FontWeight.w800,
                  letterSpacing: -0.5,
                ),
              ),
              Text(currency,
                  style: const TextStyle(
                      color: AppTheme.textSecondary, fontSize: 12)),
            ],
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
}

// ── Status badge (translated from StatusBadge / Badge from ListingCardNew) ────

class StatusBadge extends StatelessWidget {
  const StatusBadge({super.key, required this.label, required this.type});
  final String label;
  final StatusType type;

  @override
  Widget build(BuildContext context) {
    final (bg, fg, border) = switch (type) {
      StatusType.running    => (AppTheme.green.withOpacity(0.18),  AppTheme.green,  AppTheme.green),
      StatusType.stopped    => (AppTheme.amber.withOpacity(0.18),  AppTheme.amber,  AppTheme.amber),
      StatusType.overdue    => (AppTheme.red.withOpacity(0.18),    AppTheme.red,    AppTheme.red),
      StatusType.connected  => (AppTheme.green.withOpacity(0.18),  AppTheme.green,  AppTheme.green),
      StatusType.disconnected => (AppTheme.red.withOpacity(0.18),  AppTheme.red,    AppTheme.red),
    };
    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 10, vertical: 3),
      decoration: BoxDecoration(
        color: bg,
        borderRadius: BorderRadius.circular(20),
        border: Border.all(color: border.withOpacity(0.6)),
      ),
      child: Text(label,
          style: TextStyle(
              color: fg, fontSize: 10, fontWeight: FontWeight.w700,
              letterSpacing: 0.3)),
    );
  }
}

enum StatusType { running, stopped, overdue, connected, disconnected }

// ── Action button (translated from AccountLauncherPage action buttons) ─────────

class ActionButton extends StatelessWidget {
  const ActionButton({
    super.key,
    required this.icon,
    required this.label,
    required this.color,
    required this.onTap,
    this.isPrimary = false,
  });
  final IconData     icon;
  final String       label;
  final Color        color;
  final VoidCallback onTap;
  final bool         isPrimary;

  @override
  Widget build(BuildContext context) {
    if (isPrimary) {
      return SizedBox(
        width: double.infinity,
        height: 44,
        child: FilledButton.icon(
          onPressed: onTap,
          icon: Icon(icon, size: 16, color: Colors.white),
          label: Text(label,
              style: const TextStyle(
                  color: Colors.white, fontWeight: FontWeight.w600)),
          style: FilledButton.styleFrom(
            backgroundColor: AppTheme.purple,
            shape: RoundedRectangleBorder(
                borderRadius: BorderRadius.circular(10)),
          ),
        ),
      );
    }
    return SizedBox(
      width: double.infinity,
      height: 44,
      child: OutlinedButton.icon(
        onPressed: onTap,
        icon: Icon(icon, size: 16, color: color),
        label: Text(label,
            style: TextStyle(color: color, fontWeight: FontWeight.w600)),
        style: OutlinedButton.styleFrom(
          side: BorderSide(color: color.withOpacity(0.4)),
          backgroundColor: color.withOpacity(0.08),
          shape: RoundedRectangleBorder(
              borderRadius: BorderRadius.circular(10)),
        ),
      ),
    );
  }
}

// ── Tabbed detail scaffold (translated from AccountLauncherPage main layout) ───

class DetailScaffold extends ConsumerStatefulWidget {
  const DetailScaffold({
    super.key,
    required this.account,
    required this.overviewTab,
    this.extraTabs       = const [],
    this.extraTabLabels  = const [],
    /// Called after every successful edit/action so the screen can invalidate
    /// its own detail provider in addition to the global dashboard refresh.
    this.onRefresh,
  });
  final AccountSummary account;
  final Widget         overviewTab;
  final List<Widget>   extraTabs;
  final List<String>   extraTabLabels;
  final VoidCallback?  onRefresh;

  @override
  ConsumerState<DetailScaffold> createState() => _DetailScaffoldState();
}

class _DetailScaffoldState extends ConsumerState<DetailScaffold>
    with SingleTickerProviderStateMixin {
  late TabController _tabs;

  @override
  void initState() {
    super.initState();
    _tabs = TabController(length: 2 + widget.extraTabs.length, vsync: this);
  }

  @override
  void dispose() {
    _tabs.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: AppTheme.pageBg,
      appBar: AppBar(
        // ── Header (AccountLauncherPage-style) ────────────────────────────
        title: Row(
          children: [
            // Avatar initial
            Container(
              width: 32, height: 32,
              decoration: BoxDecoration(
                color: AppTheme.purple.withOpacity(0.25),
                borderRadius: BorderRadius.circular(8),
                border: Border.all(color: AppTheme.purpleBorder),
              ),
              alignment: Alignment.center,
              child: Text(
                widget.account.accountName[0].toUpperCase(),
                style: const TextStyle(
                    color: AppTheme.purpleLight,
                    fontWeight: FontWeight.w700,
                    fontSize: 14),
              ),
            ),
            const SizedBox(width: 10),
            Expanded(
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                mainAxisSize: MainAxisSize.min,
                children: [
                  Text(widget.account.accountName,
                      style: const TextStyle(
                          color: AppTheme.textPrimary,
                          fontSize: 15,
                          fontWeight: FontWeight.w600)),
                  Text(widget.account.typeLabel,
                      style: const TextStyle(
                          color: AppTheme.textSecondary, fontSize: 12)),
                ],
              ),
            ),
          ],
        ),
        actions: [
          // Edit button
          IconButton(
            icon: const Icon(Icons.edit_outlined, size: 20),
            tooltip: 'Edit',
            onPressed: () => _openEdit(context),
          ),
          IconButton(
            icon: const Icon(Icons.refresh_rounded, size: 20),
            onPressed: () => _refresh(),
          ),
        ],
        bottom: TabBar(
          controller: _tabs,
          tabs: [
            const Tab(text: 'Overview'),
            ...widget.extraTabLabels.map((l) => Tab(text: l)),
            const Tab(text: 'History'),
          ],
        ),
      ),
      body: TabBarView(
        controller: _tabs,
        children: [
          widget.overviewTab,
          ...widget.extraTabs,
          AccountEntriesScreen(account: widget.account),
        ],
      ),
    );
  }

  Future<void> _openEdit(BuildContext context) async {
    final result = await showFormSheet<bool>(
        context, EditAccountSheet(account: widget.account));
    if (result == true) {
      widget.onRefresh?.call();
      refreshDashboard(ref);
    }
  }

  void _refresh() {
    widget.onRefresh?.call();
    refreshDashboard(ref);
  }
}
