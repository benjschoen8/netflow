import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../core/providers.dart';
import '../../theme/app_theme.dart';

/// A label field that shows a dropdown of previously-used labels.
/// If no labels exist yet → shows a plain text input immediately.
/// If labels exist → shows a dropdown where the last item is "+ Add new label",
/// which when selected expands an inline text field.
class LabelPickerField extends ConsumerStatefulWidget {
  const LabelPickerField({
    super.key,
    required this.controller,
    this.hint,
  });
  final TextEditingController controller;
  final String? hint;

  @override
  ConsumerState<LabelPickerField> createState() => _LabelPickerFieldState();
}

class _LabelPickerFieldState extends ConsumerState<LabelPickerField> {
  /// null = no selection yet / free-text mode
  String? _selected;
  bool    _addingNew = false;

  static const _addNewSentinel = '__add_new__';

  @override
  void initState() {
    super.initState();
    // If controller already has a value (e.g. edit mode), go straight to free-text
    if (widget.controller.text.isNotEmpty) {
      _addingNew = true;
    }
  }

  @override
  Widget build(BuildContext context) {
    final entriesAsync = ref.watch(allEntriesProvider);

    // Collect unique non-empty labels from history
    final rawLabels = entriesAsync.asData?.value
        .map((e) => e.label)
        .whereType<String>()
        .where((l) => l.isNotEmpty)
        .toSet()
        .toList() ?? [];
    rawLabels.sort();
    final existingLabels = rawLabels;

    // No history yet → plain text input
    if (existingLabels.isEmpty) {
      return _freeTextField();
    }

    // Has history → dropdown + optional inline input
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        DropdownButtonFormField<String>(
          value: _addingNew ? _addNewSentinel : (_selected?.isEmpty ?? true ? null : _selected),
          decoration: InputDecoration(
            labelText: 'Label (optional)',
            prefixIcon: const Icon(Icons.label_outline_rounded),
            border: const OutlineInputBorder(
                borderRadius: BorderRadius.all(Radius.circular(12))),
            filled: true,
            fillColor: AppTheme.surface2,
            enabledBorder: const OutlineInputBorder(
                borderRadius: BorderRadius.all(Radius.circular(12)),
                borderSide: BorderSide(color: AppTheme.border)),
            focusedBorder: const OutlineInputBorder(
                borderRadius: BorderRadius.all(Radius.circular(12)),
                borderSide: BorderSide(color: AppTheme.purple, width: 1.5)),
          ),
          dropdownColor: AppTheme.surface,
          style: const TextStyle(color: AppTheme.textPrimary, fontSize: 14),
          hint: const Text('Select or add label',
              style: TextStyle(color: AppTheme.textSecondary)),
          items: [
            // Existing labels
            ...existingLabels.map((l) => DropdownMenuItem(
                  value: l,
                  child: Row(
                    children: [
                      Container(
                        width: 8, height: 8,
                        decoration: BoxDecoration(
                          color: AppTheme.purple.withOpacity(0.6),
                          shape: BoxShape.circle,
                        ),
                      ),
                      const SizedBox(width: 10),
                      Text(l,
                          style: const TextStyle(
                              color: AppTheme.textPrimary, fontSize: 14)),
                    ],
                  ),
                )),
            // Separator
            const DropdownMenuItem<String>(
              enabled: false,
              value: null,
              child: Divider(color: AppTheme.border, height: 1),
            ),
            // Add new option
            DropdownMenuItem(
              value: _addNewSentinel,
              child: Row(
                children: [
                  Icon(Icons.add_rounded, size: 16, color: AppTheme.purple),
                  const SizedBox(width: 8),
                  const Text('Add new label',
                      style: TextStyle(
                          color: AppTheme.purple, fontWeight: FontWeight.w500, fontSize: 14)),
                ],
              ),
            ),
          ],
          onChanged: (v) {
            if (v == _addNewSentinel) {
              setState(() {
                _addingNew = true;
                _selected  = null;
              });
              widget.controller.clear();
            } else if (v != null) {
              setState(() {
                _selected  = v;
                _addingNew = false;
              });
              widget.controller.text = v;
            } else {
              setState(() {
                _selected  = null;
                _addingNew = false;
              });
              widget.controller.clear();
            }
          },
        ),

        // Inline new-label input expands when "Add new label" is selected
        if (_addingNew) ...[
          const SizedBox(height: 10),
          _freeTextField(autofocus: true),
        ],
      ],
    );
  }

  Widget _freeTextField({bool autofocus = false}) => TextField(
        controller: widget.controller,
        autofocus: autofocus,
        textCapitalization: TextCapitalization.words,
        decoration: InputDecoration(
          labelText: _addingNew ? 'New label' : 'Label (optional)',
          hintText: widget.hint ?? 'e.g. Lunch, Salary, Rent',
          prefixIcon: const Icon(Icons.label_outline_rounded),
          border: const OutlineInputBorder(
              borderRadius: BorderRadius.all(Radius.circular(12))),
          filled: true,
          fillColor: AppTheme.surface2,
          enabledBorder: const OutlineInputBorder(
              borderRadius: BorderRadius.all(Radius.circular(12)),
              borderSide: BorderSide(color: AppTheme.border)),
          focusedBorder: const OutlineInputBorder(
              borderRadius: BorderRadius.all(Radius.circular(12)),
              borderSide: BorderSide(color: AppTheme.purple, width: 1.5)),
          suffixIcon: _addingNew
              ? IconButton(
                  icon: const Icon(Icons.close_rounded,
                      size: 16, color: AppTheme.textSecondary),
                  onPressed: () {
                    setState(() => _addingNew = false);
                    widget.controller.clear();
                  },
                )
              : null,
        ),
      );
}
