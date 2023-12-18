function DisplayArr(arr, index) {
  if (index < arr.length) {
    console.log(arr[index]);
    return DisplayArr(arr, index + 1);
  }
}

function StoreArr(arr, index, newArr) {
  if (index === arr.length) return newArr;
  newArr.push(arr[index]);
  return StoreArr(arr, index + 1, newArr);
}

const arr1 = [1, 2, 4, 5, 9, 8, 1, 4, 6, 6];

console.log(DisplayArr(arr1, 0));
let arr2 = [];
StoreArr(arr1, 0, arr2);
console.log(arr2);
