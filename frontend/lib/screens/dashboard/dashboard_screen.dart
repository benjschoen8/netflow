import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../core/providers.dart';
import 'widgets/net_worth_card.dart';
import 'widgets/account_list.dart';

class DashboardScreen extends ConsumerWidget {
  const DashboardScreen({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('netflow'),
        actions: [
          // Refresh button
          IconButton(
            onPressed: () => refreshDashboard(ref),
            icon: const Icon(Icons.refresh_rounded),
            tooltip: 'Refresh',
          ),
          // Theme toggle placeholder (wire up later)
          IconButton(
            onPressed: () {},
            icon: const Icon(Icons.more_vert_rounded),
            tooltip: 'More',
          ),
        ],
      ),
      body: RefreshIndicator(
        onRefresh: () async => refreshDashboard(ref),
        child: CustomScrollView(
          slivers: [
            // ── Net worth section ──────────────────────────────────────────
            const SliverToBoxAdapter(
              child: Padding(
                padding: EdgeInsets.fromLTRB(20, 16, 20, 0),
                child: _SectionHeader(title: 'Overview'),
              ),
            ),
            const SliverToBoxAdapter(child: NetWorthCard()),

            // ── Accounts section ───────────────────────────────────────────
            const SliverToBoxAdapter(
              child: Padding(
                padding: EdgeInsets.fromLTRB(20, 8, 20, 0),
                child: _SectionHeader(title: 'Accounts'),
              ),
            ),
            const SliverToBoxAdapter(child: AccountList()),

            // Bottom padding
            const SliverToBoxAdapter(child: SizedBox(height: 100)),
          ],
        ),
      ),
      floatingActionButton: FloatingActionButton.extended(
        onPressed: () {
          // TODO: navigate to add account sheet
          ScaffoldMessenger.of(context).showSnackBar(
            const SnackBar(content: Text('Add account — coming soon')),
          );
        },
        icon: const Icon(Icons.add_rounded),
        label: const Text('Add Account'),
      ),
    );
  }
}

class _SectionHeader extends StatelessWidget {
  const _SectionHeader({required this.title});
  final String title;

  @override
  Widget build(BuildContext context) {
    return Text(
      title,
      style: Theme.of(context).textTheme.titleMedium?.copyWith(
            fontWeight: FontWeight.w700,
          ),
    );
  }
}
