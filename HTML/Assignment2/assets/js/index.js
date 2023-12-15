const createImageCard = (imageUrl) => {
  const card = document.createElement("div");
  card.classList.add("image-card");

  const image = document.createElement("img");
  image.src = imageUrl;
  image.alt = "Image";

  card.appendChild(image);
  document.getElementById("imageContainer").appendChild(card);
};


//Logic code

(() => {
  let arr1 = [1, 2, 3, 4, 5];
  let firstElement = arr1.shift();
  Function2(firstElement, ...arr1);
})();

function Function2(fe, ...args) {
  let arr2 = [6, 7, 8, 9];
  arr2.unshift(fe);
  arr2.push(...args);
  let sum = arr2.reduce((acc, ele) => acc + ele, 0);
  var promise = new Promise((res, rej) => {
    sum > 30 ? res(sum) : rej("sum is less than 30");
  });

  promise
    .then((item) => {
      fetch(`https://picsum.photos/v2/list?limit=${item}`)
        .then((res) => res.json())
        .then((value) => {
          value.forEach((item) => createImageCard(item.download_url));
        });
    })
    .catch((error) => {
      alert(error);
    });
}
