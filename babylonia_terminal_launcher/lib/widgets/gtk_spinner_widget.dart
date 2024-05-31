import 'dart:math';

import 'package:flutter/material.dart';

class GtkSpinner extends StatefulWidget {
  const GtkSpinner({
    super.key,
    this.size = 15.0,
    this.strokeSize = 5.0,
    this.color = Colors.grey,
  });
  final double size;
  final double strokeSize;
  final Color color;

  @override
  State<GtkSpinner> createState() => _GtkSpinnerState();
}

class _GtkSpinnerState extends State<GtkSpinner>
    with SingleTickerProviderStateMixin {
  late AnimationController _controller;

  @override
  void initState() {
    super.initState();
    _controller = AnimationController(
      vsync: this,
      duration: const Duration(seconds: 1, milliseconds: 500),
    )..repeat();
  }

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.all(8.0),
      child: AnimatedBuilder(
        animation: _controller,
        builder: (context, child) {
          return CustomPaint(
            painter: _GtkSpinnerPainter(
              _controller.value * 2 * pi,
              widget.strokeSize,
              widget.color,
            ),
            size: Size(widget.size, widget.size),
          );
        },
      ),
    );
  }
}

class _GtkSpinnerPainter extends CustomPainter {
  _GtkSpinnerPainter(this.angle, this.strokeSize, this.color);

  final double angle;
  final double strokeSize;
  final Color color;

  @override
  void paint(Canvas canvas, Size size) {
    final double radius = size.width / 2;
    final Rect rect =
        Rect.fromCircle(center: Offset(radius, radius), radius: radius);

    // Define the gradient for the arc
    final Paint gradientPaint = Paint()
      ..shader = SweepGradient(
        startAngle: 0.0,
        endAngle: 2 * pi,
        colors: [
          color,
          color.withOpacity(0.0),
        ],
        stops: const [0.0, 0.90],
        transform: GradientRotation(angle),
      ).createShader(rect)
      ..style = PaintingStyle.stroke
      ..strokeCap = StrokeCap.round
      ..strokeWidth = strokeSize;

    // Draw the gradient arc
    canvas.drawArc(
      rect,
      angle,
      7 * pi / 4, // Draw an arc of 300 degrees
      false,
      gradientPaint,
    );

    // Add two circles at the start and end to create the rounded caps effect
    final Paint capPaint = Paint()
      ..color = color
      ..style = PaintingStyle.fill;

    canvas.drawCircle(
      Offset(radius + radius * cos(angle), radius + radius * sin(angle)),
      strokeSize / 2,
      capPaint,
    );
  }

  @override
  bool shouldRepaint(CustomPainter oldDelegate) => true;
}
