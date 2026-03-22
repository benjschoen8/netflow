import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'api/ffi_repository.dart';
import '../models/account_summary.dart';
import '../models/account_detail.dart';
import '../models/credit_card_info.dart';
import '../models/ledger_entry.dart';
import '../models/net_worth_result.dart';
import '../models/statement.dart';

// ── UI Preferences ────────────────────────────────────────────────────────────

enum AppFontSize { small, medium, large }

class FontSizeNotifier extends Notifier<AppFontSize> {
  @override
  AppFontSize build() => AppFontSize.medium;
  void set(AppFontSize size) => state = size;
}

final fontSizeProvider = NotifierProvider<FontSizeNotifier, AppFontSize>(
  FontSizeNotifier.new,
);

double fontScale(AppFontSize s) => switch (s) {
      AppFontSize.small  => 0.85,
      AppFontSize.medium => 1.0,
      AppFontSize.large  => 1.18,
    };

// ── Repository ────────────────────────────────────────────────────────────────
//
// Single instance — FfiRepository holds no mutable state; the Rust side
// owns the db connection via the OnceLock in ledger::interface::ffi.

final ledgerRepoProvider = Provider<FfiRepository>((_) => const FfiRepository());

// ── Bootstrap ─────────────────────────────────────────────────────────────────
//
// Creates the user finances record on first run (idempotent after that).
// Returns true if an existing record was found, false if it was just created.

final bootstrapProvider = FutureProvider<bool>((ref) async {
  final repo = ref.watch(ledgerRepoProvider);
  try {
    await ref.watch(ledgerRepoProvider).listAccounts();
    return true;
  } catch (e) {
    if (e is ApiException && e.isNotFound) {
      await repo.init();
      return false;
    }
    rethrow;
  }
});

// ── Dashboard data ────────────────────────────────────────────────────────────

final accountsProvider = FutureProvider<List<AccountSummary>>((ref) {
  return ref.watch(ledgerRepoProvider).listAccounts();
});

final netWorthProvider = FutureProvider<List<NetWorthResult>>((ref) {
  return ref.watch(ledgerRepoProvider).getNetWorth();
});

final allEntriesProvider = FutureProvider<List<LedgerEntry>>((ref) async {
  final accounts = await ref.watch(accountsProvider.future);
  if (accounts.isEmpty) return [];
  final repo    = ref.watch(ledgerRepoProvider);
  final results = await Future.wait(
    accounts.map((a) => repo
        .listEntries(a.accountId)
        .catchError((_) => <LedgerEntry>[])),
  );
  final all = results.expand((list) => list).toList()
    ..sort((a, b) => b.occurredAt.compareTo(a.occurredAt));
  return all;
});

// ── Per-account detail providers ──────────────────────────────────────────────

final cashDetailProvider = FutureProvider.family<CashAccountDetail, String>(
  (ref, id) => ref.watch(ledgerRepoProvider).getCashDetail(id),
);

final walletDetailProvider = FutureProvider.family<PhysicalWalletDetail, String>(
  (ref, id) => ref.watch(ledgerRepoProvider).getWalletDetail(id),
);

final digitalWalletDetailProvider = FutureProvider.family<DigitalWalletDetail, String>(
  (ref, id) => ref.watch(ledgerRepoProvider).getDigitalWalletDetail(id),
);

final investmentDetailProvider = FutureProvider.family<InvestmentAccountDetail, String>(
  (ref, id) => ref.watch(ledgerRepoProvider).getInvestmentDetail(id),
);

final loanDetailProvider = FutureProvider.family<LoanAccountDetail, String>(
  (ref, id) => ref.watch(ledgerRepoProvider).getLoanDetail(id),
);

final cardInfoProvider = FutureProvider.family<CreditCardInfo, String>(
  (ref, id) => ref.watch(ledgerRepoProvider).getCreditCardInfo(id),
);

final statementsProvider = FutureProvider.family<List<Statement>, String>(
  (ref, id) => ref.watch(ledgerRepoProvider).listStatements(id),
);

// ── Global refresh ────────────────────────────────────────────────────────────

void refreshDashboard(WidgetRef ref) {
  ref.invalidate(accountsProvider);
  ref.invalidate(netWorthProvider);
  ref.invalidate(allEntriesProvider);
  ref.invalidate(cashDetailProvider);
  ref.invalidate(walletDetailProvider);
  ref.invalidate(digitalWalletDetailProvider);
  ref.invalidate(investmentDetailProvider);
  ref.invalidate(loanDetailProvider);
  ref.invalidate(cardInfoProvider);
  ref.invalidate(statementsProvider);
}
