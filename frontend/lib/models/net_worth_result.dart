/// Mirrors the `NetWorthResult` struct returned by `GET /net-worth`.
class NetWorthResult {
  final String currency;
  final String totalAssets;
  final String totalDebts;
  final String netWorth;
  final bool isDeficit;

  const NetWorthResult({
    required this.currency,
    required this.totalAssets,
    required this.totalDebts,
    required this.netWorth,
    required this.isDeficit,
  });

  factory NetWorthResult.fromJson(Map<String, dynamic> json) => NetWorthResult(
        currency:    json['currency']     as String,
        totalAssets: json['total_assets'] as String,
        totalDebts:  json['total_debts']  as String,
        netWorth:    json['net_worth']    as String,
        isDeficit:   json['is_deficit']   as bool,
      );
}
