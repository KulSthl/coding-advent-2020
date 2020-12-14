import 'dart:io';

main() async {
  var file = File('./day13/test.txt');

  if (await file.exists()) {
    // Read file
    var contents = await file.readAsLines();
    part_1(contents);
    part_2(contents);
  }
}

calc(int number, int busId) {
  var result = 0;
  if (number % busId == 0) {
    return busId;
  } else {
    for (var i = number; i % busId != 0; i++) {
      result = (i + 1);
    }
  }
  return result;
}

part_2(List<String> contents) {
  int minutes = 1;
  int first = 0;
  for (var i = 0; i < contents.length; i++) {
    if (i % 2 == 0) {
    } else {
      var temp = contents[i].split(",");
      first = int.tryParse(temp[0]);
      List<String> current = temp.getRange(1, temp.length).toList();
      List<int> numbers = [];
      List<double> rem = [];
      for (var i = 0; i < current.length; i++) {
        try {
          var num = int.tryParse(current[i]);
          rem.add(num.toDouble()-i );
          numbers.add((num));
        } catch (e) {}
      }
      print(numbers);
      print(rem);
      List<double> partialProduct = [];
      var run = true;
      var product = 1;
      for (var i = 0; i < numbers.length; i++) {
        product *= numbers[i];
      }
      for (var i = 0; i < numbers.length; i++) {
        partialProduct.add(product / numbers[i]);
      }
      double sum = 0;
      for (var i = 0; i < numbers.length; i++) {
        double x = numbers[i].toDouble();
        var inverse = computeInverse(partialProduct[i], x);
        sum += partialProduct[i] *
            inverse *
            rem[i];
      }
      print(sum % product);
    }
  }
}

double computeInverse(double a, double b) {
  double m = b, t, q;
  double x = 0, y = 1;
  if (b == 1) return 0;
  // Apply extended Euclid Algorithm
  while (a > 1) {
    // q is quotient
    q = a / b;
    t = b;
    // now proceed same as Euclid's algorithm
    b = a % b;
    a = t;
    t = x;
    x = y - q * x;
    y = t;
  }
  // Make x1 positive
  if (y < 0) y += m;
  return y;
}

part_1(List<String> contents) {
  var lowestBusId = [0, 99999999];
  int num = 0;
  for (var i = 0; i < contents.length; i++) {
    if (i % 2 == 0) {
      num = int.tryParse(contents[i]);
    } else {
      var temp = contents[i].split(",");
      temp.forEach((element) {
        if (element == 'x') {
        } else {
          int busId = int.tryParse(element);
          int result = calc(num, busId);
          if (lowestBusId[1] > result) {
            lowestBusId[1] = result;
            lowestBusId[0] = busId;
          }
        }
      });
      print(lowestBusId);
    }
  }
  print('${lowestBusId} = ${(lowestBusId[1] - num) * lowestBusId[0]}');
}
