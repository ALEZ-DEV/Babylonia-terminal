// This file has been literally copied from https://github.com/ubuntu/yaru.dart/blob/main/lib/src/widgets/yaru_expansion_panel.dart
// and https://github.com/ubuntu/yaru.dart/blob/main/lib/src/widgets/yaru_expandable.dart
// Just want to make some change to it for specific needs instead of contributing to it, because I'm too lazy for that and I think they don't want these change

import 'package:flutter/material.dart';

import 'package:yaru/constants.dart';
import 'package:yaru/widgets.dart';

class SectionController extends ChangeNotifier {
  int selectedItem = 0;
  Function()? onChange;

  SectionController({required this.selectedItem});

  void updateSection(int newSelectedItem) {
    selectedItem = newSelectedItem;
    notifyListeners();
    if (onChange != null) {
      onChange!();
    }
  }
}

class SelectableYaruExpansionPanel extends StatefulWidget {
  const SelectableYaruExpansionPanel({
    super.key,
    required this.controller,
    required this.children,
    this.borderRadius =
        const BorderRadius.all(Radius.circular(kYaruContainerRadius)),
    this.border,
    required this.headers,
    this.width,
    this.height,
    this.padding,
    this.margin,
    this.expandIconPadding = const EdgeInsets.all(10),
    this.headerPadding = const EdgeInsets.only(left: 20),
    this.color,
    this.placeDividers = true,
    this.expandIcon,
    this.shrinkWrap = true,
    this.scrollPhysics = const ClampingScrollPhysics(),
    this.collapseOnExpand = true,
  }) : assert(headers.length == children.length);

  final SectionController controller;
  final List<Widget> children;
  final List<Widget> headers;
  final BorderRadius borderRadius;
  final BoxBorder? border;
  final double? width;
  final double? height;
  final EdgeInsetsGeometry? padding;
  final EdgeInsetsGeometry? margin;
  final EdgeInsetsGeometry expandIconPadding;
  final EdgeInsetsGeometry headerPadding;
  final Color? color;
  final bool placeDividers;
  final Widget? expandIcon;
  final bool shrinkWrap;
  final ScrollPhysics scrollPhysics;
  final bool collapseOnExpand;

  @override
  State<SelectableYaruExpansionPanel> createState() =>
      _SelectableYaruExpansionPanelState();
}

class _SelectableYaruExpansionPanelState
    extends State<SelectableYaruExpansionPanel> {
  late List<bool> _expandedStore;

  @override
  void initState() {
    super.initState();
    widget.controller.onChange = widget.collapseOnExpand
        ? () {
            _expandedStore[widget.controller.selectedItem];
            for (var n = 0; n < _expandedStore.length; n++) {
              if (_expandedStore[n]) {
                setState(() => _expandedStore[n] = false);
              }
            }
          }
        : null;
    _expandedStore =
        List<bool>.generate(widget.children.length, (index) => false);
  }

  @override
  Widget build(BuildContext context) {
    assert(widget.children.length == widget.headers.length);

    _expandedStore[widget.controller.selectedItem] = true;

    return YaruBorderContainer(
      border: widget.border,
      borderRadius: widget.borderRadius,
      color: widget.color,
      width: widget.width,
      height: widget.height,
      padding: widget.padding,
      margin: widget.margin,
      child: widget.placeDividers
          ? ListView.separated(
              shrinkWrap: widget.shrinkWrap,
              physics: widget.scrollPhysics,
              itemCount: widget.children.length,
              itemBuilder: _itemBuilder,
              separatorBuilder: (context, index) {
                if (index != widget.children.length - 1) {
                  return const Padding(
                    padding: EdgeInsets.symmetric(vertical: 1),
                    child: Divider(),
                  );
                } else {
                  return const SizedBox.shrink();
                }
              },
            )
          : ListView.builder(
              shrinkWrap: widget.shrinkWrap,
              physics: widget.scrollPhysics,
              itemCount: widget.children.length,
              itemBuilder: _itemBuilder,
            ),
    );
  }

  Widget? _itemBuilder(context, index) {
    return YaruExpandable(
      expandIconPadding: widget.expandIconPadding,
      isExpanded: _expandedStore[index],
      header: Padding(
        padding: widget.headerPadding,
        child: widget.headers[index],
      ),
      child: widget.children[index],
    );
  }
}

const _kAnimationDuration = Duration(milliseconds: 250);
const _kAnimationCurve = Curves.easeInOutCubic;

class YaruExpandable extends StatefulWidget {
  const YaruExpandable({
    super.key,
    required this.header,
    this.expandIconPadding = EdgeInsets.zero,
    required this.child,
    this.collapsedChild,
    this.gapHeight = 4.0,
    this.isExpanded = false,
    this.onChange,
  });

  final Widget header;
  final EdgeInsetsGeometry expandIconPadding;
  final Widget child;
  final Widget? collapsedChild;
  final double gapHeight;
  final bool isExpanded;
  final ValueChanged<bool>? onChange;

  @override
  State<YaruExpandable> createState() => _YaruExpandableState();
}

class _YaruExpandableState extends State<YaruExpandable> {
  late bool _isExpanded;

  @override
  void initState() {
    _isExpanded = widget.isExpanded;
    super.initState();
  }

  @override
  void didUpdateWidget(YaruExpandable oldWidget) {
    super.didUpdateWidget(oldWidget);
    if (oldWidget.isExpanded != widget.isExpanded) {
      _isExpanded = widget.isExpanded;
    }
  }

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        Row(
          mainAxisAlignment: MainAxisAlignment.spaceBetween,
          children: [
            widget.header,
            Padding(
              padding: widget.expandIconPadding,
              child: const SizedBox(
                height: 36,
                width: 36,
              ),
            ),
          ],
        ),
        AnimatedCrossFade(
          firstChild: _buildChild(widget.child),
          secondChild: widget.collapsedChild != null
              ? _buildChild(widget.collapsedChild!)
              : Container(),
          crossFadeState: _isExpanded
              ? CrossFadeState.showFirst
              : CrossFadeState.showSecond,
          sizeCurve: _kAnimationCurve,
          duration: _kAnimationDuration,
        ),
      ],
    );
  }

  Widget _buildChild(Widget child) {
    return Column(
      children: [
        SizedBox(height: widget.gapHeight),
        child,
      ],
    );
  }
}
