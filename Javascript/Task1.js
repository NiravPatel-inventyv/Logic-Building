//  IIFE (Immediately invoke function expression) which will call Function2
(() => {
  let arr1 = [1, 2, 3, 4, 5];
  let firstElement = arr1.shift();
  Function2(firstElement, ...arr1);
})();

//function 2 which will create promise and resolve only if it is greater than 30
function Function2(fe, ...args) {
  let arr2 = [6, 7, 8, 9];
  arr2.unshift(fe);
  arr2.push(...args);
  let sum = arr2.reduce((acc, ele) => acc + ele,0);
  var promise = new Promise((res, rej) => {
    sum > 30 ? res(sum) : rej("sum is less than 30")
  });

// handling promise and if resolved fetch data 
  promise.then((item) => {
    fetch(`https://picsum.photos/v2/list?limit=${item}`)
    .then(res => res.json())
    .then(value => {
        const images = value.map(item => item.download_url);
        return images
    })
  }).catch((error) => {
   alert(error)
  });
}

