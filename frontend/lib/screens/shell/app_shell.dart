import 'dart:async';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../core/providers.dart';
import '../../models/account_summary.dart';
import '../../theme/app_theme.dart';
import '../../screens/accounts/add_account_sheet.dart';
import '../../screens/account_types/cash_screen.dart';
import '../../screens/account_types/wallet_screens.dart';
import '../../screens/account_types/investment_screen.dart';
import '../../screens/account_types/loan_screen.dart';
import '../../screens/credit_card/credit_card_detail_screen.dart';
import '../../screens/dashboard/dashboard_screen.dart';
import '../../screens/ledger/ledger_screen.dart';
import 'category_screen.dart';
import 'settings_screen.dart';

// ── Destination ───────────────────────────────────────────────────────────────

sealed class _Dest {}
class _DashDest     extends _Dest {}
class _LedgerDest   extends _Dest {}
class _CategoryDest extends _Dest {
  _CategoryDest(this.accountType);
  final String accountType;
}
class _AccountDest  extends _Dest {
  _AccountDest(this.id);
  final String id;
}
class _SettingsDest extends _Dest {}

// ── AppShell ──────────────────────────────────────────────────────────────────

class AppShell extends ConsumerStatefulWidget {
  const AppShell({super.key});
  @override
  ConsumerState<AppShell> createState() => _AppShellState();
}

class _AppShellState extends ConsumerState<AppShell>
    with WidgetsBindingObserver {
  _Dest _dest      = _DashDest();
  bool  _collapsed = false;
  Timer? _autoRefresh;

  static const double _sidebarW    = 240;
  static const double _collapsedW  = 72;
  static const double _topbarH     = 64;
  static const _refreshInterval    = Duration(seconds: 30);

  void _go(_Dest dest) => setState(() => _dest = dest);

  @override
  void initState() {
    super.initState();
    WidgetsBinding.instance.addObserver(this);
    _autoRefresh = Timer.periodic(_refreshInterval, (_) {
      if (mounted) refreshDashboard(ref);
    });
  }

  @override
  void dispose() {
    _autoRefresh?.cancel();
    WidgetsBinding.instance.removeObserver(this);
    super.dispose();
  }

  /// Refresh immediately when the app comes back to the foreground.
  @override
  void didChangeAppLifecycleState(AppLifecycleState state) {
    if (state == AppLifecycleState.resumed && mounted) {
      refreshDashboard(ref);
    }
  }

  @override
  Widget build(BuildContext context) {
    final accountsAsync = ref.watch(accountsProvider);
    final allAccounts   = accountsAsync.asData?.value ?? <AccountSummary>[];
    final isWide        = MediaQuery.of(context).size.width >= 800;
    final sideW         = _collapsed ? _collapsedW : _sidebarW;
    final content       = _buildContent(allAccounts);

    if (!isWide) {
      return Scaffold(
        backgroundColor: AppTheme.pageBg,
        appBar: _buildMobileAppBar(context, ref),
        drawer: Drawer(
          backgroundColor: AppTheme.surface,
          child: _SidebarBody(
            dest:      _dest,
            accounts:  allAccounts,
            collapsed: false,
            onNavigate: (d) {
              Navigator.pop(context);
              _go(d);
            },
            onAddAccount: () {
              Navigator.pop(context);
              showAddAccountSheet(context, ref);
            },
            onToggleCollapse: () {},
            onRefresh: () => refreshDashboard(ref),
          ),
        ),
        body: content,
      );
    }

    return Scaffold(
      backgroundColor: AppTheme.pageBg,
      body: Stack(
        children: [
          // ── Content (offset for topbar + sidebar) ──────────────────
          Positioned(
            left: sideW,
            top: _topbarH,
            right: 0,
            bottom: 0,
            child: content,
          ),

          // ── Sidebar ────────────────────────────────────────────────
          Positioned(
            left: 0,
            top: 0,
            bottom: 0,
            child: AnimatedContainer(
              duration: const Duration(milliseconds: 250),
              curve: Curves.easeInOut,
              width: sideW,
              child: _SidebarBody(
                dest:      _dest,
                accounts:  allAccounts,
                collapsed: _collapsed,
                onNavigate: _go,
                onAddAccount: () => showAddAccountSheet(context, ref),
                onToggleCollapse: () =>
                    setState(() => _collapsed = !_collapsed),
                onRefresh: () => refreshDashboard(ref),
              ),
            ),
          ),

          // ── Topbar ─────────────────────────────────────────────────
          Positioned(
            left: sideW,
            top: 0,
            right: 0,
            height: _topbarH,
            child: _Topbar(onSettings: () => _go(_SettingsDest())),
          ),
        ],
      ),
    );
  }

  Widget _buildContent(List<AccountSummary> all) {
    return switch (_dest) {
      _DashDest()     => const DashboardScreen(),
      _LedgerDest()   => const LedgerScreen(),
      _SettingsDest() => const SettingsScreen(),
      _CategoryDest(accountType: final type) => () {
          final group = _buildGroups(all)
              .where((g) => g.accounts.any((a) => a.accountType == type))
              .firstOrNull;
          if (group == null) return const DashboardScreen();
          return CategoryScreen(
            label:       group.label,
            icon:        group.icon,
            accounts:    group.accounts,
            allAccounts: all,
            onSelectAccount: (id) => _go(_AccountDest(id)),
          );
        }(),
      _AccountDest(id: final id) => () {
          final a = all.where((x) => x.accountId == id).firstOrNull;
          if (a == null) return const DashboardScreen();
          return _screenForAccount(a, all);
        }(),
      _ => const DashboardScreen(),
    };
  }

  PreferredSizeWidget _buildMobileAppBar(BuildContext ctx, WidgetRef ref) {
    return AppBar(
      backgroundColor: AppTheme.surface,
      leading: Builder(builder: (ctx) => IconButton(
        icon: const Icon(Icons.menu_rounded, color: AppTheme.textPrimary),
        onPressed: () => Scaffold.of(ctx).openDrawer(),
      )),
      title: const _LogoWidget(),
      actions: [
        IconButton(
          icon: const Icon(Icons.refresh_rounded, color: AppTheme.textSecondary),
          onPressed: () => refreshDashboard(ref),
        ),
      ],
    );
  }
}

Widget _screenForAccount(AccountSummary a, List<AccountSummary> all) =>
    switch (a.accountType) {
      'cash'            => CashAccountScreen(account: a, allAccounts: all),
      'physical_wallet' => PhysicalWalletScreen(account: a, allAccounts: all),
      'digital_wallet'  => DigitalWalletScreen(account: a, allAccounts: all),
      'investment'      => InvestmentAccountScreen(account: a),
      'loan'            => LoanAccountScreen(account: a, allAccounts: all),
      'credit_card'     => CreditCardDetailScreen(account: a, allAccounts: all),
      _                 => CashAccountScreen(account: a, allAccounts: all),
    };

// ── Topbar ────────────────────────────────────────────────────────────────────

class _Topbar extends StatelessWidget {
  const _Topbar({required this.onSettings});
  final VoidCallback onSettings;

  @override
  Widget build(BuildContext context) {
    return Container(
      decoration: BoxDecoration(
        color: AppTheme.surface.withOpacity(0.95),
        border: const Border(bottom: BorderSide(color: AppTheme.border)),
      ),
      padding: const EdgeInsets.symmetric(horizontal: 24),
      child: Row(
        children: [
          const Spacer(),
          // Settings
          _TopbarIconBtn(
            icon: Icons.settings_outlined,
            onTap: onSettings,
          ),
          const SizedBox(width: 4),
          // Account avatar
          Container(
            height: 40,
            padding: const EdgeInsets.symmetric(horizontal: 10),
            decoration: BoxDecoration(
              color: AppTheme.surface2,
              borderRadius: BorderRadius.circular(10),
              border: Border.all(color: AppTheme.border),
            ),
            child: Row(
              children: [
                Container(
                  width: 26, height: 26,
                  decoration: BoxDecoration(
                    color: AppTheme.purple,
                    borderRadius: BorderRadius.circular(8),
                  ),
                  child: const Center(
                    child: Text('VU',
                        style: TextStyle(
                            color: Colors.white,
                            fontSize: 10,
                            fontWeight: FontWeight.w700)),
                  ),
                ),
                const SizedBox(width: 8),
                const Text('Account',
                    style: TextStyle(
                        color: AppTheme.textPrimary,
                        fontSize: 13,
                        fontWeight: FontWeight.w500)),
              ],
            ),
          ),
        ],
      ),
    );
  }
}

class _TopbarIconBtn extends StatelessWidget {
  const _TopbarIconBtn({required this.icon, required this.onTap});
  final IconData     icon;
  final VoidCallback onTap;

  @override
  Widget build(BuildContext context) => Material(
        color: Colors.transparent,
        borderRadius: BorderRadius.circular(10),
        child: InkWell(
          borderRadius: BorderRadius.circular(10),
          onTap: onTap,
          child: Container(
            width: 40, height: 40,
            alignment: Alignment.center,
            child: Icon(icon, size: 20, color: AppTheme.textSecondary),
          ),
        ),
      );
}

// ── Sidebar ───────────────────────────────────────────────────────────────────

class _SidebarBody extends ConsumerWidget {
  const _SidebarBody({
    required this.dest,
    required this.accounts,
    required this.collapsed,
    required this.onNavigate,
    required this.onAddAccount,
    required this.onToggleCollapse,
    required this.onRefresh,
  });
  final _Dest  dest;
  final List<AccountSummary> accounts;
  final bool   collapsed;
  final void Function(_Dest) onNavigate;
  final VoidCallback onAddAccount;
  final VoidCallback onToggleCollapse;
  final VoidCallback onRefresh;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final selId = dest is _AccountDest ? (dest as _AccountDest).id : null;

    // Build type-level groups: label, icon, accounts
    final groups = _buildGroups(accounts);

    return Container(
      color: AppTheme.surface,
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          // ── Logo area ────────────────────────────────────────────────
          SafeArea(
            bottom: false,
            child: SizedBox(
              height: 64,
              child: Padding(
                padding: EdgeInsets.symmetric(
                    horizontal: collapsed ? 16 : 20),
                child: collapsed
                    ? const Center(child: _LogoDot())
                    : const _LogoWidget(),
              ),
            ),
          ),

          // ── Divider ───────────────────────────────────────────────────
          const Divider(height: 1, color: AppTheme.border),

          // ── Nav items ─────────────────────────────────────────────────
          Expanded(
            child: ListView(
              padding: EdgeInsets.symmetric(
                  horizontal: collapsed ? 8 : 10, vertical: 8),
              children: [
                _NavItem(
                  icon: Icons.bar_chart_rounded,
                  label: 'Dashboard',
                  selected: dest is _DashDest,
                  collapsed: collapsed,
                  onTap: () => onNavigate(_DashDest()),
                ),
                const SizedBox(height: 2),
                _NavItem(
                  icon: Icons.receipt_long_rounded,
                  label: 'Ledger',
                  selected: dest is _LedgerDest,
                  collapsed: collapsed,
                  onTap: () => onNavigate(_LedgerDest()),
                ),
                const SizedBox(height: 4),

                // Account category groups
                ...groups.map((g) => _CategoryGroup(
                      group: g,
                      selectedId: selId,
                      activeDest: dest,
                      collapsed: collapsed,
                      onNavigate: onNavigate,
                    )),

                if (accounts.isEmpty && !collapsed)
                  Padding(
                    padding: const EdgeInsets.all(12),
                    child: Text(
                      'No accounts yet.\nTap + Add below.',
                      style: const TextStyle(
                          color: AppTheme.textSecondary, fontSize: 12),
                    ),
                  ),
              ],
            ),
          ),

          // ── Bottom actions ────────────────────────────────────────────
          Container(
            decoration: const BoxDecoration(
                border: Border(top: BorderSide(color: AppTheme.border))),
            padding: EdgeInsets.symmetric(
                horizontal: collapsed ? 8 : 10, vertical: 10),
            child: SafeArea(
              top: false,
              child: Column(
                children: [
                  _BottomBtn(
                    icon: Icons.add_circle_outline_rounded,
                    label: 'Add Account',
                    color: AppTheme.purple,
                    collapsed: collapsed,
                    onTap: onAddAccount,
                  ),
                  const SizedBox(height: 2),
                  _BottomBtn(
                    icon: Icons.refresh_rounded,
                    label: 'Refresh everything',
                    color: AppTheme.textSecondary,
                    collapsed: collapsed,
                    onTap: onRefresh,
                  ),
                  const SizedBox(height: 2),
                  _BottomBtn(
                    icon: collapsed
                        ? Icons.chevron_right_rounded
                        : Icons.chevron_left_rounded,
                    label: 'Collapse sidebar',
                    color: AppTheme.textSecondary,
                    collapsed: collapsed,
                    onTap: onToggleCollapse,
                  ),
                ],
              ),
            ),
          ),
        ],
      ),
    );
  }
}

// ── Logo widgets ──────────────────────────────────────────────────────────────

class _LogoDot extends StatelessWidget {
  const _LogoDot();
  @override
  Widget build(BuildContext context) => Container(
        width: 32, height: 32,
        decoration: BoxDecoration(
          color: AppTheme.purple,
          borderRadius: BorderRadius.circular(8),
        ),
        child: const Center(
          child: Text('N',
              style: TextStyle(
                  color: Colors.white,
                  fontWeight: FontWeight.w800,
                  fontSize: 16)),
        ),
      );
}

class _LogoWidget extends StatelessWidget {
  const _LogoWidget();
  @override
  Widget build(BuildContext context) => Row(
        children: const [
          _LogoDot(),
          SizedBox(width: 10),
          Text('netflow',
              style: TextStyle(
                  color: AppTheme.textPrimary,
                  fontWeight: FontWeight.w700,
                  fontSize: 16)),
        ],
      );
}

// ── Nav item ──────────────────────────────────────────────────────────────────

class _NavItem extends StatelessWidget {
  const _NavItem({
    required this.icon,
    required this.label,
    required this.selected,
    required this.collapsed,
    required this.onTap,
  });
  final IconData     icon;
  final String       label;
  final bool         selected;
  final bool         collapsed;
  final VoidCallback onTap;

  @override
  Widget build(BuildContext context) {
    final fg = selected ? AppTheme.purple : AppTheme.textPrimary;
    final bg = selected ? AppTheme.purple.withOpacity(0.18) : Colors.transparent;

    return Tooltip(
      message: collapsed ? label : '',
      child: Material(
        color: Colors.transparent,
        child: InkWell(
          borderRadius: BorderRadius.circular(10),
          onTap: onTap,
          child: AnimatedContainer(
            duration: const Duration(milliseconds: 150),
            height: 40,
            padding: EdgeInsets.symmetric(
                horizontal: collapsed ? 0 : 12),
            decoration: BoxDecoration(
              color: bg,
              borderRadius: BorderRadius.circular(10),
              border: Border.all(
                  color: selected
                      ? AppTheme.purple.withOpacity(0.4)
                      : Colors.transparent),
            ),
            child: collapsed
                ? Center(child: Icon(icon, size: 20, color: fg))
                : Row(
                    children: [
                      Icon(icon, size: 18, color: fg),
                      const SizedBox(width: 10),
                      Expanded(
                        child: Text(label,
                            style: TextStyle(
                                color: fg,
                                fontSize: 14,
                                fontWeight: selected
                                    ? FontWeight.w600
                                    : FontWeight.w400),
                            overflow: TextOverflow.ellipsis),
                      ),
                    ],
                  ),
          ),
        ),
      ),
    );
  }
}

// ── Account item ──────────────────────────────────────────────────────────────

// ── Account category group model ──────────────────────────────────────────────

class _AccountGroup {
  const _AccountGroup({
    required this.label,
    required this.icon,
    required this.accounts,
  });
  final String             label;
  final IconData           icon;
  final List<AccountSummary> accounts;
}

List<_AccountGroup> _buildGroups(List<AccountSummary> accounts) {
  final order = [
    ('Cash Account',    'cash',            Icons.account_balance_rounded),
    ('Cash Wallet',     'physical_wallet', Icons.wallet_rounded),
    ('Digital Wallet',  'digital_wallet',  Icons.contactless_rounded),
    ('Investment',      'investment',      Icons.show_chart_rounded),
    ('Credit Card',     'credit_card',     Icons.credit_card_rounded),
    ('Loan',            'loan',            Icons.request_quote_rounded),
  ];
  final groups = <_AccountGroup>[];
  for (final (label, type, icon) in order) {
    final list = accounts.where((a) => a.accountType == type).toList();
    if (list.isNotEmpty) {
      groups.add(_AccountGroup(label: label, icon: icon, accounts: list));
    }
  }
  return groups;
}

// ── Collapsible category group ────────────────────────────────────────────────

class _CategoryGroup extends StatefulWidget {
  const _CategoryGroup({
    required this.group,
    required this.selectedId,
    required this.activeDest,
    required this.collapsed,
    required this.onNavigate,
  });
  final _AccountGroup        group;
  final String?              selectedId;
  final _Dest                activeDest;
  final bool                 collapsed;
  final void Function(_Dest) onNavigate;

  @override
  State<_CategoryGroup> createState() => _CategoryGroupState();
}

class _CategoryGroupState extends State<_CategoryGroup> {
  late bool _expanded;

  @override
  void initState() {
    super.initState();
    _expanded = _hasSelected;
  }

  @override
  void didUpdateWidget(_CategoryGroup old) {
    super.didUpdateWidget(old);
    // Auto-expand when an account in this group becomes selected
    if (!_expanded && _hasSelected) {
      setState(() => _expanded = true);
    }
  }

  bool get _hasSelected =>
      widget.group.accounts.any((a) => a.accountId == widget.selectedId) ||
      (widget.activeDest is _CategoryDest &&
          (widget.activeDest as _CategoryDest).accountType ==
              widget.group.accounts.firstOrNull?.accountType);

  @override
  Widget build(BuildContext context) {
    final g          = widget.group;
    final anySelected = _hasSelected;
    final headerColor = anySelected ? AppTheme.purple : AppTheme.textSecondary;

    if (widget.collapsed) {
      // Collapsed sidebar: show just icon with count badge
      return Tooltip(
        message: g.label,
        child: GestureDetector(
          onTap: () => widget.onNavigate(
              _CategoryDest(g.accounts.first.accountType)),
          child: Padding(
            padding: const EdgeInsets.only(bottom: 2),
          child: Stack(
            clipBehavior: Clip.none,
            children: [
              Container(
                height: 38,
                decoration: BoxDecoration(
                  color: anySelected
                      ? AppTheme.purple.withOpacity(0.18)
                      : Colors.transparent,
                  borderRadius: BorderRadius.circular(10),
                ),
                child: Center(
                  child: Icon(g.icon, size: 18,
                      color: anySelected
                          ? AppTheme.purple
                          : AppTheme.textSecondary),
                ),
              ),
              Positioned(
                top: 2, right: 2,
                child: Container(
                  width: 14, height: 14,
                  decoration: BoxDecoration(
                    color: AppTheme.surface2,
                    shape: BoxShape.circle,
                    border: Border.all(color: AppTheme.border),
                  ),
                  child: Center(
                    child: Text(
                      '${g.accounts.length}',
                      style: const TextStyle(
                          color: AppTheme.textSecondary,
                          fontSize: 8,
                          fontWeight: FontWeight.w700),
                    ),
                  ),
                ),
              ),
            ],
          ),
        ),
      ),    // GestureDetector
    );
  }

    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        // ── Category header row — left side navigates, right side toggles ──
        SizedBox(
          height: 34,
          child: Row(
            crossAxisAlignment: CrossAxisAlignment.stretch,
            children: [
              // Left: icon + label → navigate to CategoryScreen
              Expanded(
                child: Material(
                  color: Colors.transparent,
                  borderRadius: const BorderRadius.only(
                    topLeft: Radius.circular(10),
                    bottomLeft: Radius.circular(10),
                  ),
                  child: InkWell(
                    borderRadius: const BorderRadius.only(
                      topLeft: Radius.circular(10),
                      bottomLeft: Radius.circular(10),
                    ),
                    onTap: () => widget.onNavigate(
                        _CategoryDest(g.accounts.first.accountType)),
                    child: Padding(
                      padding: const EdgeInsets.only(left: 10, right: 4),
                      child: Row(
                        children: [
                          Icon(g.icon, size: 14, color: headerColor),
                          const SizedBox(width: 8),
                          Expanded(
                            child: Text(
                              g.label,
                              style: TextStyle(
                                color: headerColor,
                                fontSize: 11,
                                fontWeight: FontWeight.w700,
                                letterSpacing: 0.6,
                              ),
                              overflow: TextOverflow.ellipsis,
                            ),
                          ),
                        ],
                      ),
                    ),
                  ),
                ),
              ),

              // Right: count badge + chevron → toggle expand
              Material(
                color: Colors.transparent,
                borderRadius: const BorderRadius.only(
                  topRight: Radius.circular(10),
                  bottomRight: Radius.circular(10),
                ),
                child: InkWell(
                  borderRadius: const BorderRadius.only(
                    topRight: Radius.circular(10),
                    bottomRight: Radius.circular(10),
                  ),
                  onTap: () => setState(() => _expanded = !_expanded),
                  child: Padding(
                    padding: const EdgeInsets.symmetric(horizontal: 8),
                    child: Row(
                      mainAxisSize: MainAxisSize.min,
                      children: [
                        Container(
                          padding: const EdgeInsets.symmetric(
                              horizontal: 6, vertical: 1),
                          decoration: BoxDecoration(
                            color: anySelected
                                ? AppTheme.purple.withOpacity(0.18)
                                : AppTheme.surface2,
                            borderRadius: BorderRadius.circular(10),
                            border: Border.all(
                              color: anySelected
                                  ? AppTheme.purpleBorder
                                  : AppTheme.border,
                            ),
                          ),
                          child: Text(
                            '${g.accounts.length}',
                            style: TextStyle(
                              color: anySelected
                                  ? AppTheme.purpleLight
                                  : AppTheme.textSecondary,
                              fontSize: 10,
                              fontWeight: FontWeight.w700,
                            ),
                          ),
                        ),
                        const SizedBox(width: 4),
                        Icon(
                          _expanded
                              ? Icons.expand_less_rounded
                              : Icons.expand_more_rounded,
                          size: 14,
                          color: AppTheme.textSecondary,
                        ),
                      ],
                    ),
                  ),
                ),
              ),
            ],
          ),
        ),

        // ── Expanded account list (shown when category or account selected) ─
        if (_expanded)
          ...g.accounts.map((a) => _AccountItem(
                account:  a,
                selected: a.accountId == widget.selectedId,
                collapsed: false,
                onTap: () => widget.onNavigate(_AccountDest(a.accountId)),
              )),

        const SizedBox(height: 2),
      ],
    );
  }
}

// ── Account item ──────────────────────────────────────────────────────────────

class _AccountItem extends StatelessWidget {
  const _AccountItem({
    required this.account,
    required this.selected,
    required this.collapsed,
    required this.onTap,
  });
  final AccountSummary account;
  final bool  selected;
  final bool  collapsed;
  final VoidCallback onTap;

  @override
  Widget build(BuildContext context) {
    final fg     = selected ? AppTheme.purple : AppTheme.textPrimary;
    final bg     = selected ? AppTheme.purple.withOpacity(0.18) : Colors.transparent;
    final icon   = _iconFor(account.accountType);
    final iconFg = selected ? AppTheme.purple : AppTheme.textSecondary;

    return Tooltip(
      message: collapsed ? account.accountName : '',
      child: Material(
        color: Colors.transparent,
        child: InkWell(
          borderRadius: BorderRadius.circular(10),
          onTap: onTap,
          child: AnimatedContainer(
            duration: const Duration(milliseconds: 150),
            height: 38,
            margin: const EdgeInsets.only(bottom: 2),
            padding: EdgeInsets.symmetric(
                horizontal: collapsed ? 0 : 12),
            decoration: BoxDecoration(
              color: bg,
              borderRadius: BorderRadius.circular(10),
            ),
            child: collapsed
                ? Center(child: Icon(icon, size: 18, color: iconFg))
                : Row(
                    children: [
                      Icon(icon, size: 16, color: iconFg),
                      const SizedBox(width: 10),
                      Expanded(
                        child: Text(account.accountName,
                            style: TextStyle(
                                color: fg,
                                fontSize: 13,
                                fontWeight: selected
                                    ? FontWeight.w500
                                    : FontWeight.w400),
                            overflow: TextOverflow.ellipsis),
                      ),
                      if (account.isOverdue)
                        Container(
                          width: 7, height: 7,
                          decoration: const BoxDecoration(
                              color: AppTheme.red,
                              shape: BoxShape.circle),
                        ),
                    ],
                  ),
          ),
        ),
      ),
    );
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

// ── Bottom button ─────────────────────────────────────────────────────────────

class _BottomBtn extends StatelessWidget {
  const _BottomBtn({
    required this.icon,
    required this.label,
    required this.color,
    required this.collapsed,
    required this.onTap,
  });
  final IconData     icon;
  final String       label;
  final Color        color;
  final bool         collapsed;
  final VoidCallback onTap;

  @override
  Widget build(BuildContext context) => Tooltip(
        message: collapsed ? label : '',
        child: Material(
          color: Colors.transparent,
          borderRadius: BorderRadius.circular(10),
          child: InkWell(
            borderRadius: BorderRadius.circular(10),
            onTap: onTap,
            child: Container(
              height: 40,
              padding: EdgeInsets.symmetric(
                  horizontal: collapsed ? 0 : 12),
              child: collapsed
                  ? Center(child: Icon(icon, size: 18, color: color))
                  : Row(
                      children: [
                        Icon(icon, size: 16, color: color),
                        const SizedBox(width: 10),
                        Text(label,
                            style: TextStyle(
                                color: color,
                                fontSize: 13)),
                      ],
                    ),
            ),
          ),
        ),
      );
}

// ── Section label ─────────────────────────────────────────────────────────────

class _SectionLabel extends StatelessWidget {
  const _SectionLabel(this.label);
  final String label;
  @override
  Widget build(BuildContext context) => Padding(
        padding: const EdgeInsets.fromLTRB(12, 12, 12, 4),
        child: Text(
          label.toUpperCase(),
          style: const TextStyle(
              color: AppTheme.purple,
              fontSize: 10,
              fontWeight: FontWeight.w700,
              letterSpacing: 1.2),
        ),
      );
}
