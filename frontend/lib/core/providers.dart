import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'api/api_client.dart';
import 'api/ledger_repository.dart';
import '../models/account_summary.dart';
import '../models/net_worth_result.dart';

// ── Singletons ────────────────────────────────────────────────────────────────

final apiClientProvider = Provider<ApiClient>((_) => ApiClient());

final ledgerRepoProvider = Provider<LedgerRepository>(
  (ref) => LedgerRepository(ref.watch(apiClientProvider)),
);

// ── Dashboard data ─────────────────────────────────────────────────────────

final accountsProvider = FutureProvider<List<AccountSummary>>((ref) {
  return ref.watch(ledgerRepoProvider).listAccounts();
});

final netWorthProvider = FutureProvider<List<NetWorthResult>>((ref) {
  return ref.watch(ledgerRepoProvider).getNetWorth();
});

/// Combined refresh — invalidate both at once.
void refreshDashboard(WidgetRef ref) {
  ref.invalidate(accountsProvider);
  ref.invalidate(netWorthProvider);
}
