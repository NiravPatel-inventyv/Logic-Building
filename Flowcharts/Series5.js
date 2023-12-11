// series 1 232 34543 ... if two digit found show only unit number
let n = prompt("Enter number :");
for (let i = 1; i <= n; i++) {
  let str = " ";
  for (let j = i; j < 2 * i - 1; j++) {
    str += j % 10;
  }
  for (let j = 2 * i - 1; j > i - 1; j--) {
    str += j % 10;
  }
  console.log(str);
}
