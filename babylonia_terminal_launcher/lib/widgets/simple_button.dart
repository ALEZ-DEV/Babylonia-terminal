import 'package:flutter/material.dart';
import 'package:yaru/theme.dart';

class SimpleButton extends StatelessWidget {
  const SimpleButton({super.key, required this.child, required this.onPressed});
  final Function()? onPressed;
  final Widget? child;

  @override
  Widget build(BuildContext context) {
    return ElevatedButton(
      style: ButtonStyle(
        backgroundColor: MaterialStateProperty.resolveWith((states) {
          if (states.contains(MaterialState.disabled)) {
            return Colors.grey;
          }
          return Colors.blue[500];
        }),
        side: MaterialStateProperty.all(BorderSide.none),
        shape: MaterialStateProperty.all<RoundedRectangleBorder>(
          RoundedRectangleBorder(
            borderRadius: BorderRadius.circular(100.0),
            side: const BorderSide(color: Colors.red),
          ),
        ),
      ),
      onPressed: onPressed,
      child: child,
    );
  }
}
